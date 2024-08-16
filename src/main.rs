use adapters::generate_audio;
use base64::engine::general_purpose::STANDARD;
use base64::read::DecoderReader;
use gcp_auth::provider;
use generate_story::{format_json, remove_escaped_characters};
use prompt_generation::generate_prompt::generate_api_prompt;
use reqwest::blocking::Client;
use reqwest::{header::CONTENT_TYPE, Url};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::error::Error;
use std::fs;
use std::io::Read;
use std::{
    fs::File,
    io::copy,
    process::{Command, Stdio},
};
use write_duration::Story;
use write_duration::{write_duration, write_voices};

#[macro_use]
extern crate dotenv_codegen;

pub mod adapters {
    pub mod generate_audio;
}

pub mod prompt_generation {
    pub mod comments_prompt;
    pub mod generate_prompt;
    pub mod generate_schema;
    pub mod narrator_prompt;
    pub mod story_config;
}

pub mod generate_story;
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
    let story_path = r"C:\Users\miche\Desktop\Projekte\discord-stories\apps\generateVideo\public\temp_assets\story_fragments.json";
    let audio_path = r"C:\Users\miche\Desktop\Projekte\discord-stories\apps\generateVideo\public\temp_assets\story_audio\";
    /*   let provider = gcp_auth::provider().await;
    let scopes = &["https://www.googleapis.com/auth/cloud-platform"];
    let token = provider.unwrap().token(scopes).await;

    // TO DO
    // create seperate module
    // add starting_frame and ending_frame to in write_duration
    // add dynamic paths for file manipulation
    // add clean up after file creation

    //######################################################################################
    //################################## GENERATE TEXT
    //######################################################################################

    let project_id = dotenv!("PROJECT_ID");
    let model_id = dotenv!("MODEL_ID");

    let client_gemini = reqwest::Client::new();

    // Construct the URL
    let url = format!(
               "https://us-central1-aiplatform.googleapis.com/v1/projects/{}/locations/europe-central2/publishers/google/models/{}:generateContent",
               project_id, model_id
           );

    let story_prompt = generate_api_prompt();

    //println!("{}", story_prompt);

    // Send the POST request
    let response = client_gemini
        .post(&url)
        .bearer_auth(token.unwrap().as_str())
        .header("Content-Type", "application/json")
        .json(&story_prompt)
        .send()
        .await
        .unwrap();

    // Handle the response
    if response.status().is_success() {
        let resp_json: serde_json::Value = response.json().await.unwrap();

        let candidates = &resp_json.get("candidates").unwrap();
        let first_candidates = candidates.get(0).unwrap();
        let content = first_candidates.get("content").unwrap();
        let parts = content.get("parts").unwrap();
        let first_part = parts.get(0).unwrap();
        let text = first_part.get("text").unwrap();

        println!("{:?}", text);
        // Convert the Value back into a pretty-printed JSON string
        let input = serde_json::to_string_pretty(text).unwrap();

        let _ = format_json(&input);
    } else {
        eprintln!("Request failed with status: {:?}", response);
    } */

    //######################################################################################
    ////################################ GENERATE VOICE
    //######################################################################################

    // TO DO
    // create seperate module
    // add starting_frame and ending_frame to in write_duration */
    /*   let params = [("key", dotenv!("TTS_API_KEY"))];

    let client = reqwest::Client::new();
    let base_url = Url::parse("https://texttospeech.googleapis.com").unwrap();
    let endpoint = base_url.join("/v1/text:synthesize").unwrap();
    let call_url = Url::parse_with_params(Url::as_str(&endpoint), params).unwrap();

    let file = fs::File::open(story_path).expect("file should open read only");

    let story: Story = serde_json::from_reader(&file).expect("file should be proper JSON");

    write_voices(story_path);

    for speaker in story.fragments {
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

        let response = client
            .post(call_url.clone())
            .header(CONTENT_TYPE, "application/json")
            .body(serialized_body)
            .send()
            .await
            .unwrap();

        let body = response.json::<ResponseBody>().await.expect("body invalid");
        println!("{:?}", body);
        let mut out = File::create(audio_path.to_owned() + &speaker.hashed_text + ".mp3")
            .expect("failed to create file");

        let mut decoder = DecoderReader::new(body.audio_content.as_bytes(), &STANDARD);
        let _ = copy(&mut decoder, &mut out).unwrap();
    }

    write_duration(story_path, audio_path); */

    // We are expecting an MP3 file

    //######################################################################################
    ////################################ GENERATE VIDEO
    //######################################################################################

    // check why composition id is not found
    // implemnet retry mechanisme

    Command::new("npx.cmd")
        .stdout(Stdio::inherit())
        .current_dir(r"C:\Users\miche\Desktop\Projekte\discord-stories\apps\generateVideo")
        .args([
            "remotion",
            "render",
            "./src/index.ts",
            "Story",
            "./public/temp_assets/temp/uncaptioned_story.mp4",
            "--concurrency=1",
        ])
        .output()
        .expect("error");

    /*     Command::new("node")
        .stdout(Stdio::inherit())
        .current_dir("./apps/generateVideo")
        .args(["sub.mjs", "./public/temp_assets/temp/uncaptioned_story.mp4"])
        .output()
        .expect("error");

    Command::new("npx.cmd")
        .stdout(Stdio::inherit())
        .current_dir("./apps/generateVideo")
        .args([
            "remotion",
            "render",
            "./src/index.ts",
            "CaptionedVideo",
            "./public/out/captioned_story.mp4",
            "--concurrency=1",
        ])
        .output()
        .expect("error"); */
}
