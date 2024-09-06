use crate::entities::{
    self,
    entities::{Story, StoryType},
};
use std::process::{Command, Stdio};

pub fn generate_video(story_type: StoryType) {
    match story_type {
        StoryType::Narrator(_) => {
            // Render the uncaptioned video
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
                .expect("Failed to render uncaptioned story");

            // Run node script to add subtitles
            Command::new("node")
                .stdout(Stdio::inherit())
                .current_dir("./apps/generateVideo")
                .args(["sub.mjs", "./public/temp_assets/temp/uncaptioned_story.mp4"])
                .output()
                .expect("Failed to add subtitles");

            // Render the captioned video
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
                .expect("Failed to render captioned story");
        }

        StoryType::Chat(_) => {
            // Render the chat video
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
                .expect("Failed to render chat video");
        }

        StoryType::Call(_) => {
            // Handle the Call variant (to be implemented)
            todo!("Call variant is not yet implemented.");
        }

        StoryType::Comments(_) => {
            // Handle the Comments variant
            // Implement functionality if needed or leave as a no-op
            println!("Comments variant detected. No specific handling implemented.");
        }
    }
}
