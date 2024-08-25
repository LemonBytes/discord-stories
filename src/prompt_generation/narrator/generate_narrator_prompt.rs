use crate::{json, prompt_generation::narrator::narrator_to_llm::narrator_to_llm};
use serde_json::Value;

pub fn generate_narrator_prompt(schema: String, genre: String, story_type: String) -> Value {
    let model_answer = json!({
        "story": {
            "story_type": story_type.to_owned(),
            "genre": genre,
            "fragments": [
                {
                    "voice_name": "default",
                    "user_name": "darkrainbow",
                    "speaker_text": "My Sister’s Husband Is Cheating, But She Refuses to See It.",
                    "audio_duration_in_frames": 0.0,
                    "hashed_text": "1",
                    "speaker_type": "title",
                    "gender": "female"
                },
                {
                    "voice_name": "default",
                    "user_name": "darkrainbow",
                    "speaker_text": "My sister (33) has been married to her husband (34) for nearly 10 years, and they have 3 kids together. But something is off. I’ve noticed how our distant cousin (32F), who recently moved in with them after leaving her abusive husband, has been getting too close to him. It started with harmless flirting, but now she’s outright grinding on him at family gatherings. My sister is uncomfortable, but her husband just laughs it off. The other day, I caught our cousin texting someone during a party. I couldn’t believe it when I saw my brother-in-law’s name on her phone. She claimed it was nothing, but I know what I saw. Should I tell my sister and risk tearing our family apart, or is it better to stay silent and hope it’s all just a misunderstanding?",
                    "audio_duration_in_frames": 0.0,
                    "hashed_text": "2",
                    "speaker_type": "main",
                    "gender": "female"
                },
                {
                    "voice_name": "default",
                    "user_name": "darkrainbow",
                    "speaker_text": "That’s a tough call. If you stay silent, things might get worse. But if you speak up, it could destroy everything.",
                    "audio_duration_in_frames": 0.0,
                    "hashed_text": "3",
                    "speaker_type": "main",
                    "gender": "female"
                },
                {
                    "voice_name": "default",
                    "user_name": "darkrainbow",
                    "speaker_text": "Exactly! I keep thinking about the kids. They’ve been through so much already. But at the same time, how can I ignore what I saw?",
                    "audio_duration_in_frames": 0.0,
                    "hashed_text": "4",
                    "speaker_type": "main",
                    "gender": "female"
                },
                {
                    "voice_name": "default",
                    "user_name": "darkrainbow",
                    "speaker_text": "Have you talked to your cousin directly about this? Maybe she’ll slip up and confirm your suspicions.",
                    "audio_duration_in_frames": 0.0,
                    "hashed_text": "5",
                    "speaker_type": "main",
                    "gender": "female"
                },
                {
                    "voice_name": "default",
                    "user_name": "darkrainbow",
                    "speaker_text": "I tried. She gets all defensive and says she’s just being friendly. But her eyes… they tell a different story. And then there’s my sister, so desperate to believe her marriage is fine.",
                    "audio_duration_in_frames": 0.0,
                    "hashed_text": "6",
                    "speaker_type": "main",
                    "gender": "female"
                },
                {
                    "voice_name": "default",
                    "user_name": "darkrainbow",
                    "speaker_text": "It’s almost like your sister is in denial. Sometimes people choose not to see the truth because it’s too painful.",
                    "audio_duration_in_frames": 0.0,
                    "hashed_text": "7",
                    "speaker_type": "main",
                    "gender": "female"
                },
                {
                    "voice_name": "default",
                    "user_name": "darkrainbow",
                    "speaker_text": "But what if I tell her and I’m wrong? I could ruin her life over nothing. But what if I’m right, and she ends up hurt even more in the long run?",
                    "audio_duration_in_frames": 0.0,
                    "hashed_text": "8",
                    "speaker_type": "main",
                    "gender": "female"
                },
                {
                    "voice_name": "default",
                    "user_name": "darkrainbow",
                    "speaker_text": "This is a no-win situation. Maybe you could gather more evidence before making any moves. At least then you’ll be sure.",
                    "audio_duration_in_frames": 0.0,
                    "hashed_text": "9",
                    "speaker_type": "main",
                    "gender": "female"
                },
                {
                    "voice_name": "default",
                    "user_name": "darkrainbow",
                    "speaker_text": "I’ve thought about that too. But the idea of spying on my own family feels so wrong. How did it even get to this point? It’s like a nightmare I can’t wake up from.",
                    "audio_duration_in_frames": 0.0,
                    "hashed_text": "10",
                    "speaker_type": "main",
                    "gender": "female"
                },
                {
                    "voice_name": "default",
                    "user_name": "darkrainbow",
                    "speaker_text": "Sometimes the hardest decisions are the ones that haunt us the most. But maybe by doing nothing, you’re letting things spiral out of control.",
                    "audio_duration_in_frames": 0.0,
                    "hashed_text": "11",
                    "speaker_type": "main",
                    "gender": "female"
                },
                {
                    "voice_name": "default",
                    "user_name": "darkrainbow",
                    "speaker_text": "You’re right. But if I’m going to do this, I need to be absolutely sure. I owe that much to my sister and the kids. I just don’t know if I’m strong enough to handle what I might find.",
                    "audio_duration_in_frames": 0.0,
                    "hashed_text": "12",
                    "speaker_type": "main",
                    "gender": "female"
                },
                {
                    "voice_name": "default",
                    "user_name": "darkrainbow",
                    "speaker_text": "Whatever you decide, just know you’re not alone. This is heavy stuff, but you’ve got people who care about you and your sister. Lean on them if you need to.",
                    "audio_duration_in_frames": 0.0,
                    "hashed_text": "13",
                    "speaker_type": "main",
                    "gender": "female"
                },
                {
                    "voice_name": "default",
                    "user_name": "darkrainbow",
                    "speaker_text": "Thanks. I just wish things were simpler. But I guess that’s life. Nothing is ever as straightforward as it seems.",
                    "audio_duration_in_frames": 0.0,
                    "hashed_text": "14",
                    "speaker_type": "main",
                    "gender": "female"
                },
                {
                    "voice_name": "default",
                    "user_name": "darkrainbow",
                    "speaker_text": "Yeah, life’s messy like that. But you’ve got a good head on your shoulders. Whatever you do, trust your instincts.",
                    "audio_duration_in_frames": 0.0,
                    "hashed_text": "15",
                    "speaker_type": "main",
                    "gender": "female"
                },
                {
                    "voice_name": "default",
                    "user_name": "darkrainbow",
                    "speaker_text": "I will. I just hope that, in the end, it’s the right call.",
                    "audio_duration_in_frames": 0.0,
                    "hashed_text": "16",
                    "speaker_type": "main",
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
                        "text": "Generate a crazy story that involves a sister discussing her sister's marriage and her husband."
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
                     "text": narrator_to_llm(genre) + "In this JSON schema:" + &schema
                  }
               ]
            }
         ],
    });

    let narrator_prompt = json!({
        "system_instruction": {
            "parts": {
                "text": "You are a first-person narrator, tasked with crafting gripping and dramatic stories that immediately hook readers and compel them to share or comment. These stories must be explicit, emotionally charged, and brimming with tension. Incorporate elements that evoke strong reactions, such as betrayal, deep secrets, or shocking twists. Utilize 'rage bait' and thought-provoking questions to drive high engagement. Generate funny user names not related to the story. The story's title should be provocative, intriguing, and slightly controversial, ideally around 60 characters. Titles must capture attention and provoke curiosity, such as: - 'My Sister’s Husband Is Cheating, But She Refuses to See It.' - 'I Caught My Brother-in-Law Texting My Cousin; Should I Tell My Sister?' - 'My Sister’s Marriage Is a Mess, and It’s Not Even Her Fault.' - 'Can I Stay Silent About My Sister's Cheating Husband?' Titles can also be in the form of a question to directly engage the audience, such as: - 'Should I Expose My Sister’s Husband?' - 'Is It My Place to Tell My Sister About Her Husband’s Affair?' - 'Am I Wrong for Not Telling My Sister What I Saw?' The story should always align closely with the title, beginning with a shocking revelation or an intense emotional moment that pulls the reader in. The first-person narrator must be the central character and speaker_type 'main',  JSON schema:".to_owned() + &schema
            }
        },
        "contents": contents.get("contents"),
         "generationConfig": {
            "response_mime_type": "application/json",
        }
    });

    narrator_prompt
}
