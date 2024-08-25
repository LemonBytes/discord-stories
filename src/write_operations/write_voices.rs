use std::{collections::HashMap, fs};

use crate::{
    entities::entities::{Gender, Language, Story},
    prompt_generation::story_config::get_voices_google,
};

fn get_gender(speaker_gender: &str) -> Gender {
    match speaker_gender.to_lowercase().as_str() {
        "male" => Gender::Male(speaker_gender.to_string()),
        "female" => Gender::Female(speaker_gender.to_string()),
        _ => Gender::Male(speaker_gender.to_string()),
    }
}

pub fn write_voices(story_path: &str, language: Language) {
    let file_read = fs::File::options()
        .read(true)
        .open(story_path)
        .expect("failed at reading process");

    let mut story: Story = serde_json::from_reader(&file_read).expect("File should be proper JSON");

    // HashMap to track which voice has been assigned to each user_name
    let mut assigned_voices: HashMap<String, String> = HashMap::new();

    for speaker in &mut story.fragments {
        if let Some(voice) = assigned_voices.get(&speaker.user_name) {
            // If the user_name already has an assigned voice, use it
            speaker.voice_name = voice.clone();
        } else {
            println!("{:?}", get_gender(&speaker.gender));

            // Otherwise, generate a new voice based on the gender
            let random_voice = get_voices_google(get_gender(&speaker.gender), language.clone());
            // Assign the generated voice to the speaker
            speaker.voice_name = random_voice.clone();
            // Store the assigned voice in the HashMap
            assigned_voices.insert(speaker.user_name.clone(), random_voice);
        }
    }

    // Optionally, save the modified story back to the file or handle it as needed
    let file_write = fs::File::create(story_path).expect("Failed at writing process");
    serde_json::to_writer(file_write, &story).expect("Failed to write JSON");
}
