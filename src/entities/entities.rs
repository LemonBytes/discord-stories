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
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
#[derive(Clone)]
pub enum Gender {
    Male(String),
    Female(String),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
#[derive(Clone)]
pub enum StoryType {
    Narrator(String),
    Chat(String),
    Call(String),
    Comments(String),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
#[derive(Clone)]

pub enum StoryGenre {
    Funny(String),
    Horrific(String),
    Sad(String),
    Perverted(String),
    Crazy(String),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Story {
    pub story_type: StoryType,
    pub genre: StoryGenre,
    pub fragments: Vec<Fragments>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Fragments {
    pub voice_name: String,
    pub user_name: String,
    pub speaker_text: String,
    pub audio_duration_in_frames: f32,
    pub speaker_order: u8,
    pub speaker_type: SpeakerType,
    pub gender: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
#[derive(Clone)]
pub enum Language {
    German(String),
    English(String),
}
