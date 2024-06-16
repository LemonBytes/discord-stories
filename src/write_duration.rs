use std::{
    fs::{self, File},
    io::Write,
};

use lofty::{config::ParseOptions, file::AudioFile, mpeg::MpegFile};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum SpeakerType {
    Title(String),
    Main(String),
    SubText(String),
    SubVoice(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Speaker {
    pub voice_name: String,
    pub user_name: String,
    pub speaker_text: String,
    pub audio_duration_in_frames: f32,
    pub hashed_text: String,
    pub speaker_type: SpeakerType,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct Story {
    pub story: Vec<Speaker>,
}

pub fn write_duration(story_path: &str, audio_path: &str) {
    const FRAME_RATE: f32 = 30.0;
    const BUFFER_FRAMES: f32 = 30.0;

    let file_read = fs::File::options()
        .read(true)
        .open(story_path)
        .expect("failed at reading process");

    let mut story: Story = serde_json::from_reader(&file_read).expect("file should be proper JSON");

    for speaker in &mut story.story {
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
