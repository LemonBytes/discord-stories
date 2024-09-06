use serde_json::{json, Value};

pub fn generate_schema() -> Value {
    let json_prompt_schema = json!({
        "type": "object",
        "properties": {
                "type": "object",
                "properties": {
                    "story_type": { "type": "string" },
                    "genre": { "type": "string" },
                    "fragments": {
                        "type": "array",
                        "items": {
                            "type": "object",
                            "properties": {
                                "voice_name": { "type": "string" },
                                "user_name": { "type": "string" },
                                "speaker_text": { "type": "string" },
                                "audio_duration_in_frames": { "type": "number" },
                                "speaker_order": { "type": "string" },
                                "speaker_type": { "type": "string" },
                                "gender": { "type": "string" }
                            },
                            "required": [
                                "voice_name",
                                "user_name",
                                "speaker_text",
                                "audio_duration_in_frames",
                                "speaker_order",
                                "speaker_type",
                                "gender"
                            ]
                        }
                    }
                },
                "required": ["story_type", "genre"]
            }

    });

    return json_prompt_schema;
}

pub fn generate_gpt_schema() -> Value {
    let json_prompt_schema = json!({
            "story_type": {
                "type": "string",
                "description": "The type of the story, e.g., 'chat'."
            },
            "genre": {
                "type": "string",
                "description": "The genre of the story, e.g., 'crazy', 'funny', 'sad'."
            },
            "fragments": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "voice_name": {
                            "type": "string",
                            "description": "always default"
                        },
                        "user_name": {
                            "type": "string",
                            "description": "The name of the user who provided the text."
                        },
                        "speaker_text": {
                            "type": "string",
                            "description": "The text spoken by the speaker."
                        },
                        "audio_duration_in_frames": {
                            "type": "number",
                            "description": "The duration of the audio in frames. always 0.0"
                        },
                        "speaker_order": {
                            "type": "number",
                            "description": "The speaker order."
                        },
                        "speaker_type": {
                            "type": "string",
                            "description": "The type of the speaker, e.g., 'sub_text'."
                        },
                        "gender": {
                            "type": "string",
                            "description": "The gender of the speaker, e.g., 'male', 'female'."
                        }
                    },
                    "required": [
                        "voice_name",
                        "user_name",
                        "speaker_text",
                        "audio_duration_in_frames",
                        "speaker_order",
                        "speaker_type",
                        "gender"
                    ]
                },
                "description": "An array of fragments with information about the voice and text."
            }
    });
    return json_prompt_schema;
}
