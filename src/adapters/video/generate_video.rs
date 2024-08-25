use crate::entities::entities::Story;
use std::{
    fs,
    process::{Command, Stdio},
};

const STORY_PATH: &str = r"C:\Users\miche\Desktop\Projekte\discord-stories\apps\generateVideo\public\temp_assets\story_fragments.json";

pub fn generate_video() {
    let file = fs::File::open(STORY_PATH).expect("file should open read only");

    //let story: Story = serde_json::from_reader(&file).expect("file should be proper JSON");

    Command::new("npx.cmd")
        .stdout(Stdio::inherit())
        .current_dir(r"C:\Users\miche\Desktop\Projekte\discord-stories\apps\generateVideo")
        .args([
            "--max-old-space-size=8192",
            "remotion",
            "render",
            "./src/index.ts",
            "Story",
            "./public/temp_assets/temp/uncaptioned_story.mp4",
            "--concurrency=1",
        ])
        .output()
        .expect("error");

    /*    match story.story_type {
        entities::entities::StoryType::Narrator(_) => {
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

            Command::new("node")
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
                .expect("error");
        }
        entities::entities::StoryType::Comments(_) => {}
        entities::entities::StoryType::Chat(_) => {
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
        }
        entities::entities::StoryType::Call(_) => todo!(),
    } */
}
