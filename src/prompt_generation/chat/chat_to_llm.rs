use crate::{
    entities::entities::Language,
    prompt_generation::story_config::{get_genre, get_topic},
};

pub fn chat_to_llm(language: Language) -> std::string::String {
    let binding = get_genre();
    let genre = binding.as_str();

    println!("{:?}", language);

    let text_language = match language {
        Language::German(_) => "WICHTIG! der sub_text: IST AUF DEUTSCH!",
        Language::English(_) => "IMPORTANT! sub_text: IS IN ENGLISH",
    };

    let user_framing = "The user names should involve poo and pee sometimes or weird word combinations or pop culture the user names should not be longer than 10 Characters.the personalities should talk like 16 years old.The dialogue should be sarcastic, full of insults, and feature a mix of slang and modern teenage lingo. ";

    let chat_framing = format!(
         "{} Craft a {} entertaining, emotionally charged Discord conversation. About a topic which goes viral in tik tok The dialogue should be sarcastic, full of insults, and feature a mix of slang and modern teenage lingo.",
         text_language, genre, 
    );

    //let personality_framing = format!("Be creative in the discussion and be very detailed, add as much persons with different personalities as you want, with distinct and extreme psychological profiles engage in a heated debate. The discussion rapidly devolves into a chaotic and contentious exchange of insults, accusations, and raw emotions, revealing the complexity and volatility of their personalities. Each character’s unique traits and disorders drive the dialogue, ensuring a dramatic and unforgettable confrontation. ");

    let chat = format!("{} {}", chat_framing, user_framing);

    println!("{}", chat);
    return chat;
}
