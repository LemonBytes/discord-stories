use adapters::generate_audio;
use base64::engine::general_purpose::STANDARD;
use base64::read::DecoderReader;
use reqwest::blocking::Client;
use reqwest::{header::CONTENT_TYPE, Url};
use serde::{Deserialize, Serialize};
use std::fs;
use std::{
    fs::File,
    io::copy,
    process::{Command, Stdio},
};
use write_duration::write_duration;
use write_duration::Story;

#[macro_use]
extern crate dotenv_codegen;

pub mod adapters {
    pub mod generate_audio;
}

pub mod write_duration;

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

#[tokio::main]
async fn main() {
    let params = [("key", dotenv!("TTS_API_KEY"))];

    // GENERATE VOICE
    let client = reqwest::Client::new();
    let base_url = Url::parse("https://texttospeech.googleapis.com").unwrap();
    let endpoint = base_url.join("/v1/text:synthesize").unwrap();
    let call_url = Url::parse_with_params(Url::as_str(&endpoint), params).unwrap();

    let file = fs::File::open("./apps/generateVideo/public/temp_assets/story.json")
        .expect("file should open read only");

    let story: Story = serde_json::from_reader(&file).expect("file should be proper JSON");

    for speaker in story.story {
        let request_body = RequestBody {
            voice: Voice {
                language_code: String::from("en-US"),
                name: String::from(speaker.voice_name),
            },
            audio_config: AudioConfig {
                audio_encoding: String::from("MP3"),
                speaking_rate: 1.0,
            },
            input: Input {
                text: speaker.speaker_text,
            },
        };

        let serialized_body = serde_json::to_string(&request_body).unwrap();
        println!("{:?}", serialized_body);
        let response = client
            .post(call_url.clone())
            .header(CONTENT_TYPE, "application/json")
            .body(serialized_body)
            .send()
            .await
            .unwrap();

        let body = response.json::<ResponseBody>().await.expect("body invalid");

        let mut out = File::create(
            "./apps/generateVideo/public/temp_assets/story_audio/".to_owned()
                + &speaker.hashed_text
                + ".mp3",
        )
        .expect("failed to create file");

        let mut decoder = DecoderReader::new(body.audio_content.as_bytes(), &STANDARD);
        let _ = copy(&mut decoder, &mut out).unwrap();
    }

    let story_path = "./apps/generateVideo/public/temp_assets/story.json";
    let audio_path = "./apps/generateVideo/public/temp_assets/story_audio/";
    write_duration(story_path, audio_path);

    //

    //""

    // We are expecting an MP3 file
    /*
    Command::new("npx.cmd")
        .stdout(Stdio::inherit())
        .current_dir("./apps/generateVideo")
        .args([
            "remotion",
            "render",
            "Story",
            "./public/temp_assets/temp/story.mp4",
        ])
        .output()
        .expect("error");

    Command::new("node")
        .stdout(Stdio::inherit())
        .current_dir("./apps/generateVideo")
        .args(["sub.mjs", "./public/temp/temp_assets/story.mp4"])
        .output()
        .expect("error");

    Command::new("npx.cmd")
        .stdout(Stdio::inherit())
        .current_dir("./apps/generateVideo")
        .args([
            "remotion",
            "render",
            "CaptionedVideo",
            "./public/out/captioned_story.mp4",
        ])
        .output()
        .expect("error"); */
}
