use crate::entities::entities::{Language, Story};
use crate::write_operations::write_audio_duration::write_duration;
use crate::write_operations::write_voices::write_voices;
use base64::engine::general_purpose::STANDARD;
use base64::read::DecoderReader;
use reqwest::{header::CONTENT_TYPE, Client, Url};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::{fs::File, io};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Voice {
    language_code: String,
    name: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AudioConfig {
    audio_encoding: String,
    speaking_rate: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Input {
    text: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct RequestBody {
    audio_config: AudioConfig,
    input: Input,
    voice: Voice,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ResponseBody {
    audio_content: String,
}

const STORY_PATH: &str = r"C:\Users\miche\Desktop\Projekte\discord-stories\apps\generateVideo\public\temp_assets\story_fragments.json";
const AUDIO_PATH: &str = r"C:\Users\miche\Desktop\Projekte\discord-stories\apps\generateVideo\public\temp_assets\story_audio\";

pub async fn generate_audio(client: Client, language: Language) -> Result<(), Box<dyn Error>> {
    let params = [("key", dotenv!("TTS_API_KEY"))];

    let base_url = Url::parse("https://texttospeech.googleapis.com")?;
    let endpoint = base_url.join("/v1/text:synthesize")?;
    let call_url = Url::parse_with_params(Url::as_str(&endpoint), params)?;

    let file = File::open(STORY_PATH)?;
    let story: Story = serde_json::from_reader(&file)?;

    let language_code = match language {
        Language::German(_) => "de-DE".to_owned().to_string(),
        Language::English(_) => "en-US".to_owned().to_string(),
    };

    write_voices(STORY_PATH, language);

    for speaker in story.fragments {
        let request_body = RequestBody {
            voice: Voice {
                language_code: language_code.clone(),
                name: String::from(speaker.voice_name),
            },
            audio_config: AudioConfig {
                audio_encoding: String::from("MP3"),
                speaking_rate: 1.35,
            },
            input: Input {
                text: speaker.speaker_text,
            },
        };

        let serialized_body = serde_json::to_string(&request_body)?;

        let response = client
            .post(call_url.clone())
            .header(CONTENT_TYPE, "application/json")
            .body(serialized_body)
            .send()
            .await?;

        let body = response.json::<ResponseBody>().await?;
        println!("{:?}", body);

        let mut out = File::create(AUDIO_PATH.to_owned() + &speaker.hashed_text + ".mp3")?;
        let mut decoder = DecoderReader::new(body.audio_content.as_bytes(), &STANDARD);
        io::copy(&mut decoder, &mut out)?;
    }

    write_duration(STORY_PATH, AUDIO_PATH);

    Ok(())
}
