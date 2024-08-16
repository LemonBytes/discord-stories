use crate::{json, prompt_generation::generate_prompt::generate_comments_llm};
use serde_json::Value;

pub fn generate_comments_prompt(schema: String, genre: String, story_type: String) -> Value {
    let model_answer = json!({
        "story": {
            "story_type": story_type,
            "genre": genre,
            "fragments": [
                {
                    "voice_name": "default",
                    "user_name": "CheekyChuckle",
                    "speaker_text": "Why is boob cleavage ok but not butt cleavage?",
                    "audio_duration_in_frames": 0.0,
                    "hashed_text": "1",
                    "speaker_type": "title",
                    "gender": "male"
                },
                {
                    "voice_name": "default",
                    "user_name": "BoobLover3000",
                    "speaker_text": "It's fascinating how society seems to accept boob cleavage but reacts strongly against butt cleavage. Why do you think one is considered more acceptable than the other? Is it a matter of sexualization or simply cultural norms?",
                    "audio_duration_in_frames": 0.0,
                    "hashed_text": "2",
                    "speaker_type": "sub_text",
                    "gender": "female"
                },
                {
                    "voice_name": "default",
                    "user_name": "ButtMysteryMan",
                    "speaker_text": "I've noticed the same thing! It’s like there’s a double standard where one type of cleavage is seen as 'classy' while the other is deemed inappropriate. What are your thoughts on the impact this has on body image and self-expression?",
                    "audio_duration_in_frames": 0.0,
                    "hashed_text": "3",
                    "speaker_type": "sub_text",
                    "gender": "male"
                },
                {
                    "voice_name": "default",
                    "user_name": "CleavageConnoisseur",
                    "speaker_text": "This disparity often feels like a reflection of deeper societal biases. Could it be that boob cleavage is normalized because it’s more associated with femininity and sexuality, whereas butt cleavage challenges traditional modesty standards?",
                    "audio_duration_in_frames": 0.0,
                    "hashed_text": "4",
                    "speaker_type": "sub_text",
                    "gender": "female"
                },
                {
                    "voice_name": "default",
                    "user_name": "GiggleGuru",
                    "speaker_text": "Why does it seem like showing a little more skin on your chest is totally fine, but a bit of cheek is considered scandalous? Could this be a case of cultural overreaction or just a funny quirk of social norms?",
                    "audio_duration_in_frames": 0.0,
                    "hashed_text": "5",
                    "speaker_type": "sub_text",
                    "gender": "male"
                },
                {
                    "voice_name": "default",
                    "user_name": "CheekyChap",
                    "speaker_text": "It's hilarious how showing some sideboob is deemed classy while a little butt crack sends everyone into a tizzy. What do you think this says about our society's priorities?",
                    "audio_duration_in_frames": 0.0,
                    "hashed_text": "6",
                    "speaker_type": "sub_text",
                    "gender": "female"
                },
                {
                    "voice_name": "default",
                    "user_name": "JollyJester",
                    "speaker_text": "If boob cleavage is fine, why does showing a little more of your rear get you side-eyed? Could it be that we've just got our priorities mixed up, or is it something deeper?",
                    "audio_duration_in_frames": 0.0,
                    "hashed_text": "7",
                    "speaker_type": "sub_text",
                    "gender": "male"
                },
                {
                    "voice_name": "default",
                    "user_name": "WittyWanderer",
                    "speaker_text": "Why is it that we seem to have a much stronger reaction to butt cleavage than boob cleavage? Is it just a matter of tradition or is there something more to it?",
                    "audio_duration_in_frames": 0.0,
                    "hashed_text": "8",
                    "speaker_type": "sub_text",
                    "gender": "female"
                }
            ]
        }
    });

    let contents = json!({
        "contents":[
            {
                "role": "user",
                "parts": [
                    {
                        "text": "Generate a funny question which is like 'Why is boob cleavage ok but not butt cleavage?'"
                    }
                ]
            },
            {
                "role": "model",
                "parts": [
                    {
                        "text": model_answer.to_string()
                    }
                ]
            },
            {
               "role":"user",
               "parts":[
                  {
                     "text": generate_comments_llm(genre) + "In this JSON schema:" + &schema
                  }
               ]
            }
         ],
    });

    let comments_prompt = json!({
        "system_instruction": {
            "parts": {
                "text": "You are a question asker on a discord server, ask a gripping, dramatic question that hooks readers instantly and makes them want to share or comment. The comments should be about 150 characters long, explicit, emotionally charged, and filled with tension. Include elements that provoke strong reactions, like betrayal, secrets, or unexpected twists. The title should be a question to engage the audience, such as: - 'Foods you CAN'T live WITHOUT?' - 'How do you wash your butt if you don’t have arms? *serious question*' - 'My cat gets excited when I sing, what does this mean?' - 'What’s the best thing about being a parent?' - 'What's the saddest question you've ever heard?' - 'Do albino people have to take melatonin to sleep since their bodies don’t produce any?' The first fragment should always align with the title question, starting with a shocking revelation or an emotionally charged moment. The comments should always be of type sub_text. Your output is in this JSON schema:".to_owned() + &schema
            }
        },
        "contents": contents.get("contents"),
         "generationConfig": {
            "response_mime_type": "application/json",
        }
    });

    comments_prompt
}
