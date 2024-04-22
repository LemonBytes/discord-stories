use adapters::generate_audio;
use reqwest;
use std::process::{Command, Stdio};

pub mod adapters {
    pub mod generate_audio;
}

#[tokio::main]
async fn main() {
    /*  Command::new("npx.cmd")
    .stdout(Stdio::inherit())
    .current_dir("./apps/generateVideo")
    .args(["remotion", "render", "Story", "out/video.mp4"])
    .output()
    .expect("error"); */

    let client = reqwest::Client::new();
    let response = client
        .get("https://www.rust-lang.org")
        .send()
        .await
        .unwrap()
        .text()
        .await;

    println!("body = {response:?}");
}
