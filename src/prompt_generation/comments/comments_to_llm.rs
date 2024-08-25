use crate::prompt_generation::story_config::get_topic;

pub fn comments_to_llm(genre: String) -> std::string::String {
    let topic = get_topic();
    //let context = get_context();
    let question = format!(
        "Create a {} question designed to spark an intense and engaging conversation about: {}. The question should be provocative, thought-provoking, and capable of eliciting strong emotional responses. Ensure the question invites deep discussion and controversy. Include at least 8 comments that are rich in narrative detail, emotionally charged, and reveal personal experiences or moral dilemmas. Each comment should add a new perspective or twist to the conversation, enhancing the depth and complexity of the discussion.",
        genre, topic
    );
    println!("{:?}", question);
    question
}
