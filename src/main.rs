#![recursion_limit = "256"]
use adapters::audio::handle_generate_audio::handle_generate_audio;
use adapters::text::generate_text::generate_text;
use adapters::video::generate_video::generate_video;
use entities::entities::Language;
use serde_json::json;

#[macro_use]
extern crate dotenv_codegen;

pub mod prompt_generation {
    pub mod generate_prompt;
    pub mod generate_schema;
    pub mod story_config;
    pub mod chat {
        pub mod chat_model_training;
        pub mod chat_to_llm;
        pub mod generate_chat_prompt;
    }
    pub mod narrator {
        pub mod generate_narrator_prompt;
        pub mod narrator_to_llm;
    }
    pub mod comments {
        pub mod comments_to_llm;
        pub mod generate_comments_prompt;
    }
}
pub mod write_operations {
    pub mod format_json;
    pub mod write_audio_duration;
    pub mod write_voices;
}

pub mod adapters {
    pub mod audio {
        pub mod generate_audio;
        pub mod handle_generate_audio;
    }
    pub mod text {
        pub mod generate_text;
    }

    pub mod video {
        pub mod generate_video;
    }
}

pub mod entities {
    pub mod entities;
}

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();

    let german = Language::German(String::from("de-DE"));
    let english = Language::English(String::from("en-US"));

    generate_text(&client, english.clone()).await;

    handle_generate_audio(client, english.clone()).await;

    generate_video();
}
