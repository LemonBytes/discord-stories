use serde_json::{json, Value};

pub fn generate_schema() -> Value {
    let json_prompt_schema = json!({
        "type": "object",
        "properties": {
            "story": {
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
                                "hashed_text": { "type": "string" },
                                "speaker_type": { "type": "string" },
                                "gender": { "type": "string" }
                            },
                            "required": [
                                "voice_name",
                                "user_name",
                                "speaker_text",
                                "audio_duration_in_frames",
                                "hashed_text",
                                "speaker_type",
                                "gender"
                            ]
                        }
                    }
                },
                "required": ["story_type", "genre"]
            }
        },
        "required": ["story"]
    });

    return json_prompt_schema;
}
