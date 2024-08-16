use super::{
    comments_prompt::generate_comments_prompt,
    generate_schema::generate_schema,
    narrator_prompt::generate_narrator_prompt,
    story_config::{get_genre, get_story_type},
};
use serde_json::Value;

use crate::{
    entities::entities::StoryType,
    prompt_generation::story_config::{get_peoples_involved, get_situations, get_topic},
};

pub fn generate_api_prompt() -> Value {
    let binding = generate_schema();
    let schema_as_str = binding.to_owned().to_string();
    let story_type = get_story_type();
    let genre = get_genre();

    let request_body = match story_type {
        StoryType::Narrator(story_type) => {
            generate_narrator_prompt(schema_as_str, genre, story_type)
        }
        StoryType::Comments(story_type) => {
            generate_comments_prompt(schema_as_str, genre, story_type)
        }
        StoryType::Chat(_) => todo!(),
        StoryType::Call(_) => todo!(),
    };

    request_body
}

pub fn generate_comments_llm(genre: String) -> std::string::String {
    let topic = get_topic(genre.clone());
    //let context = get_context();
    let question = format!(
        "Create a {} question designed to spark an intense and engaging conversation about: {}. The question should be provocative, thought-provoking, and capable of eliciting strong emotional responses. Ensure the question invites deep discussion and controversy. Include at least 4 comments that are rich in narrative detail, emotionally charged, and reveal personal experiences or moral dilemmas. Each comment should add a new perspective or twist to the conversation, enhancing the depth and complexity of the discussion.",
        genre, topic
    );
    println!("{:?}", question);
    question
}

pub fn generate_narrator_to_llm(genre: String) -> std::string::String {
    let people = get_peoples_involved();
    let situation = get_situations();

    let story = format!(
        "Craft a {} story that revolves around {}. The narrative should explore {} in a way that is deeply engaging and emotionally resonant. Use vivid descriptions and compelling details.",
        genre, people, situation
    );

    println!("{:?}", story);
    return story;
}
