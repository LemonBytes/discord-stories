use crate::{json, prompt_generation::comments::comments_to_llm::comments_to_llm};
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

    let model_answer_two = json!({
        "story": {
            "story_type": story_type,
            "genre": genre,
            "fragments": [
                {
                    "voice_name": "default",
                    "user_name": "CheekyChuckle",
                    "speaker_text": "What's hard about dating you?",
                    "audio_duration_in_frames": 0.0,
                    "hashed_text": "1",
                    "speaker_type": "title",
                    "gender": "male"
                },
                {
                    "voice_name": "default",
                    "user_name": "BoobLover3000",
                    "speaker_text": "i'm terrible at opening up and communicating about my needs.",
                    "audio_duration_in_frames": 0.0,
                    "hashed_text": "2",
                    "speaker_type": "sub_text",
                    "gender": "female"
                },
                {
                    "voice_name": "default",
                    "user_name": "ButtMysteryMan",
                    "speaker_text": "Same. I wasn't allowed to have needs for a large portion of my life so now it's almost impossible to communicate them.",
                    "audio_duration_in_frames": 0.0,
                    "hashed_text": "3",
                    "speaker_type": "sub_text",
                    "gender": "male"
                },
                {
                    "voice_name": "default",
                    "user_name": "CleavageConnoisseur",
                    "speaker_text": "When I have a bad day I don't want to talk to anyone. I don't want to be helped either. I just want to exist in silence for a while",
                    "audio_duration_in_frames": 0.0,
                    "hashed_text": "4",
                    "speaker_type": "sub_text",
                    "gender": "female"
                },
                {
                    "voice_name": "default",
                    "user_name": "GiggleGuru",
                    "speaker_text": "I get really clingy when I fall in love. I know it and I should tone it down but for some reason i have to force myself a lot to do so",
                    "audio_duration_in_frames": 0.0,
                    "hashed_text": "5",
                    "speaker_type": "sub_text",
                    "gender": "male"
                },
                {
                    "voice_name": "default",
                    "user_name": "CheekyChap",
                    "speaker_text": "Dude same. My poor husband. I just wanna be next to him all the time no matter what we are doing. I want to hear every thought of his as it happens. I want to wear his shirt while HE’S wearing his shirt. 😂 I just WANT him. He doesn’t hate it but he also doesn’t love it so I guess a win is a win.",
                    "audio_duration_in_frames": 0.0,
                    "hashed_text": "6",
                    "speaker_type": "sub_text",
                    "gender": "female"
                },
                {
                    "voice_name": "default",
                    "user_name": "JollyJester",
                    "speaker_text": "here's the plan. 1. Buy him a shirt way to large for him. 2. Have him put it on in front of you. 3. You can both fit! Do it!",
                    "audio_duration_in_frames": 0.0,
                    "hashed_text": "7",
                    "speaker_type": "sub_text",
                    "gender": "male"
                },
                {
                    "voice_name": "default",
                    "user_name": "WittyWanderer",
                    "speaker_text": "I want to hear every thought of his This is me too, but didn't actually realize it until just now.I'm single, but when I like somebody I wanna know everything about them. What they're thinking why they think it so on.",
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
                        "text": model_answer_two.to_string()
                    }
                ]
            },
            {
               "role":"user",
               "parts":[
                  {
                     "text": comments_to_llm(genre) + "In this JSON schema:" + &schema
                  }
               ]
            }
         ],
    });

    let comments_prompt = json!({
        "system_instruction": {
            "parts": {
                "text": "You are a member of a vibrant online community, and your goal is to ask a question so gripping and dramatic that it instantly hooks readers and compels them to share or comment. The question should be shocking or provocative, touching on themes like betrayal, hidden secrets, or moral dilemmas. The title must be a compelling, emotionally charged question designed to provoke a strong reaction, such as:'Would you tell your partner if you accidentally found out their best friend is cheating?''What’s the darkest secret you’ve kept from your family?''Have you ever accidentally exposed a lie that destroyed a relationship?' 'If you could press a button to ruin your worst enemy’s life, but you’d lose something precious, would you do it?' Begin with an intense or shocking revelation that aligns with the question. Ensure the story's first line grips the reader, pulling them into the narrative. Comments should be around 200 characters long, filled with tension, raw emotion, and vivid details. They should provoke heated debate, empathy, or even disbelief, making the story impossible to ignore or not share. Always include elements of surprise, moral conflict, or a twist that leaves the audience eager to respond. The comments should be emotionally charged and provoke thought, leaving viewers eager to share their own experiences or opinions. Your output is in this JSON schema:".to_owned() + &schema
            }
        },
        "contents": contents.get("contents"),
         "generationConfig": {
            "response_mime_type": "application/json",
        }
    });

    comments_prompt
}
