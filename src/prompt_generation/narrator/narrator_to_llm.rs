use crate::prompt_generation::story_config::{get_peoples_involved, get_situations};

pub fn narrator_to_llm(genre: String) -> std::string::String {
    let people = get_peoples_involved();
    let situation = get_situations();

    let story = format!(
        "Craft a {} story that revolves around The narrative should explore {} in a way that is deeply engaging and emotionally resonant. Use vivid descriptions and compelling details. be creative in the narrative and be detailed, add as much person with different personalities as you want",
        genre, situation
    );

    println!("{:?}", story);
    return story;
}
