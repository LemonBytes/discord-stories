use std::{
    collections::HashMap,
    fs::{self, File},
    io::Write,
};

use lofty::{config::ParseOptions, file::AudioFile, mpeg::MpegFile};

use crate::{entities::entities::Story, prompt_generation::story_config::get_voices_google};

pub fn write_duration(story_path: &str, audio_path: &str) {
    const FRAME_RATE: f32 = 30.0;
    const BUFFER_FRAMES: f32 = 30.0;

    let file_read = fs::File::options()
        .read(true)
        .open(story_path)
        .expect("failed at reading process");

    let mut story: Story = serde_json::from_reader(&file_read).expect("file should be proper JSON");

    for speaker in &mut story.fragments {
        let mut file_content =
            File::open(audio_path.to_owned() + &speaker.hashed_text + ".mp3").unwrap();

        let mp3_file = MpegFile::read_from(&mut file_content, ParseOptions::new()).unwrap();
        let frames = mp3_file.properties().duration().as_secs_f32() * FRAME_RATE + BUFFER_FRAMES;

        speaker.audio_duration_in_frames = frames.floor();
    }

    let mut file_write = fs::File::options()
        .write(true)
        .truncate(true)
        .open(story_path)
        .expect("failed at writing process");

    let json = serde_json::to_string(&story).expect("Serialization failed");
    let _ = file_write.write(&json.as_bytes());
}

pub fn write_voices(story_path: &str) {
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
            // Otherwise, generate a new voice based on the gender
            let random_voice = get_voices_google(speaker.gender.clone());
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
