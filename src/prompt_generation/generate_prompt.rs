use super::{
    chat::generate_chat_prompt::generate_chat_prompt,
    comments::generate_comments_prompt::generate_comments_prompt,
    generate_schema::generate_schema,
    narrator::generate_narrator_prompt::generate_narrator_prompt,
    story_config::{get_genre, get_story_type},
};
use crate::entities::entities::{Language, StoryType};
use serde_json::Value;

pub fn generate_api_prompt(language: Language, story_type: StoryType) -> Value {
    let binding = generate_schema();
    let schema_as_str = binding.to_owned().to_string();

    let genre = get_genre();

    let request_body = match story_type {
        StoryType::Narrator(story_type) => {
            generate_narrator_prompt(schema_as_str, genre, story_type, language)
        }
        StoryType::Comments(story_type) => {
            generate_comments_prompt(schema_as_str, genre, story_type)
        }
        StoryType::Chat(story_type) => generate_chat_prompt(schema_as_str, story_type, language),
        StoryType::Call(_) => todo!(),
    };

    request_body
}

pub fn generate_api_gpt_api_prompt(language: Language, story_type: StoryType) -> Value {
    let binding = generate_schema();
    let schema_as_str = binding.to_owned().to_string();

    let genre = get_genre();

    let request_body = match story_type {
        StoryType::Narrator(story_type) => {
            generate_narrator_prompt(schema_as_str, genre, story_type, language)
        }
        StoryType::Comments(story_type) => {
            generate_comments_prompt(schema_as_str, genre, story_type)
        }
        StoryType::Chat(story_type) => generate_chat_prompt(schema_as_str, story_type, language),
        StoryType::Call(_) => todo!(),
    };

    request_body
}
