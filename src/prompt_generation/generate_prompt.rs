use super::{
    chat_prompt::generate_chat_prompt,
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
        StoryType::Chat(story_type) => generate_chat_prompt(schema_as_str, genre, story_type),
        StoryType::Call(_) => todo!(),
    };

    request_body
}

pub fn generate_comments_llm(genre: String) -> std::string::String {
    let topic = get_topic();
    //let context = get_context();
    let question = format!(
        "Create a {} question designed to spark an intense and engaging conversation about: {}. The question should be provocative, thought-provoking, and capable of eliciting strong emotional responses. Ensure the question invites deep discussion and controversy. Include at least 8 comments that are rich in narrative detail, emotionally charged, and reveal personal experiences or moral dilemmas. Each comment should add a new perspective or twist to the conversation, enhancing the depth and complexity of the discussion.",
        genre, topic
    );
    println!("{:?}", question);
    question
}

pub fn generate_chat_to_llm(genre: String) -> std::string::String {
    let topic = get_topic();

    let chat = format!(
        "🔥 Ready to dive into some serious drama? 🔥 Create a {} question that’ll make everyone’s heads spin and their keyboards explode! The question should be edgy, thought-provoking, and guaranteed to stir up some intense reactions. Think betrayal, secrets, or major moral dilemmas related to {}. Your goal is to get people talking, debating, and sharing wild stories. Include at least 8 comments that are loaded with juicy details, personal drama, and all the feels. Each comment should drop a new twist or fresh perspective to keep the convo hot and happening. Let’s get the drama rolling! 🎉",
        genre, topic
    );

    println!("{:?}", chat);
    return chat;
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
