use crate::{entities::entities::Language, prompt_generation::story_config::get_topic};

pub fn narrator_to_llm(genre: String, language: Language) -> std::string::String {
    let text_language = match language {
        Language::German(_) => "WICHTIG! der text: IST AUF DEUTSCH!",
        Language::English(_) => "IMPORTANT! text: IS IN ENGLISH",
    };

    let story = format!(
        "{} Craft a {} story that goes viral on tik tok.The story revolves around the narrative: {} in a way that is deeply engaging and emotionally resonant. Use vivid descriptions and compelling details. be creative in the narrative and be detailed. The story should be around 1500 words generate a funny random username about 10 character’s long",
        text_language, genre, get_topic()
    );

    println!("{:?}", story);
    return story;
}
