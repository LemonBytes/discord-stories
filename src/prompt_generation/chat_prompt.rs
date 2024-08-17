use crate::{
    json,
    prompt_generation::generate_prompt::{generate_chat_to_llm, generate_narrator_to_llm},
};
use serde_json::Value;

pub fn generate_chat_prompt(schema: String, genre: String, story_type: String) -> Value {
    let model_answer = json!({
                "story_type": "chat",
                "genre": "drama",
                "fragments": [
                    {
                        "voice_name": "default",
                        "user_name": "DramaQueen42",
                        "speaker_text": "🚨 When Your BFF Tries to Sabotage Your Big Event—What’s the Move? 🚨",
                        "audio_duration_in_frames": 0.0,
                        "hashed_text": "1",
                        "speaker_type": "title",
                        "gender": "female"
                    },
                    {
                        "voice_name": "default",
                        "user_name": "EpicFailBot3000",
                        "speaker_text": "Okay, so my bestie (17) and I were super hyped about this epic gaming event we’ve been planning. But lately, things have been super sus. Our mutual friend (16F) is acting like she’s in a spy movie, trying to sabotage everything. The other day, I caught her texting my rival during our strategy session. She said it’s nothing, but I’m having major trust issues. Do I call her out and risk our squad, or keep it chill and hope for the best?",
                        "audio_duration_in_frames": 0.0,
                        "hashed_text": "2",
                        "speaker_type": "sub_text",
                        "gender": "female"
                    },
                    {
                        "voice_name": "default",
                        "user_name": "FlameThrowerGamer",
                        "speaker_text": "This is like a plot twist no one asked for. If I stay silent, the drama might escalate. But if I go all detective, I could totally wreck our friendship.",
                        "audio_duration_in_frames": 0.0,
                        "hashed_text": "3",
                        "speaker_type": "sub_text",
                        "gender": "male"
                    },
                    {
                        "voice_name": "default",
                        "user_name": "NoChillNinja",
                        "speaker_text": "I know, right? Plus, there’s the pressure of the event. What if all this drama turns out to be a big nothing?",
                        "audio_duration_in_frames": 0.0,
                        "hashed_text": "4",
                        "speaker_type": "sub_text",
                        "gender": "male"
                    },
                    {
                        "voice_name": "default",
                        "user_name": "SpicySushi69",
                        "speaker_text": "Have you tried just asking her straight up? Maybe she’ll slip up and spill the tea.",
                        "audio_duration_in_frames": 0.0,
                        "hashed_text": "5",
                        "speaker_type": "sub_text",
                        "gender": "female"
                    },
                    {
                        "voice_name": "default",
                        "user_name": "LOLzRUs",
                        "speaker_text": "I did. She just got super defensive and was like, 'Chill, I’m just being friendly.' But her whole vibe is off. I’m caught in this dilemma: stay quiet or spill the beans.",
                        "audio_duration_in_frames": 0.0,
                        "hashed_text": "6",
                        "speaker_type": "sub_text",
                        "gender": "male"
                    },
                    {
                        "voice_name": "default",
                        "user_name": "PixelatedPanic",
                        "speaker_text": "It’s like she’s in denial mode. Some people can’t handle the truth because it’s too much to bear.",
                        "audio_duration_in_frames": 0.0,
                        "hashed_text": "7",
                        "speaker_type": "sub_text",
                        "gender": "female"
                    },
                    {
                        "voice_name": "default",
                        "user_name": "DramaMama2024",
                        "speaker_text": "But what if I’m totally wrong and end up ruining our friendship for nothing? Or worse, what if I’m right and things get way worse?",
                        "audio_duration_in_frames": 0.0,
                        "hashed_text": "8",
                        "speaker_type": "sub_text",
                        "gender": "female"
                    },
                    {
                        "voice_name": "default",
                        "user_name": "ConfusedCat123",
                        "speaker_text": "This is a total no-win. Maybe I should just gather more evidence before making any moves.",
                        "audio_duration_in_frames": 0.0,
                        "hashed_text": "9",
                        "speaker_type": "sub_text",
                        "gender": "male"
                    },
                    {
                        "voice_name": "default",
                        "user_name": "InstaFail101",
                        "speaker_text": "I’ve thought about that too. But spying on my friend feels like a betrayal. How did things get so out of hand?",
                        "audio_duration_in_frames": 0.0,
                        "hashed_text": "10",
                        "speaker_type": "sub_text",
                        "gender": "female"
                    },
                    {
                        "voice_name": "default",
                        "user_name": "EpicFailAlert",
                        "speaker_text": "Sometimes, the hardest choices are the ones that mess with your head. But doing nothing might just let the chaos continue.",
                        "audio_duration_in_frames": 0.0,
                        "hashed_text": "11",
                        "speaker_type": "sub_text",
                        "gender": "male"
                    },
                    {
                        "voice_name": "default",
                        "user_name": "TechieTrouble",
                        "speaker_text": "You’re right. I need to be absolutely certain before I make a move. I just hope I’m strong enough to handle whatever I uncover.",
                        "audio_duration_in_frames": 0.0,
                        "hashed_text": "12",
                        "speaker_type": "sub_text",
                        "gender": "female"
                    },
                    {
                        "voice_name": "default",
                        "user_name": "GamerGirlBoss",
                        "speaker_text": "Whatever you decide, just know you’ve got support. This is rough, but you’re not alone. Reach out if you need to.",
                        "audio_duration_in_frames": 0.0,
                        "hashed_text": "13",
                        "speaker_type": "sub_text",
                        "gender": "female"
                    },
                    {
                        "voice_name": "default",
                        "user_name": "RageQuitExpert",
                        "speaker_text": "Thanks. I just wish things were simpler. But I guess life’s always a mixed bag of drama and unpredictability.",
                        "audio_duration_in_frames": 0.0,
                        "hashed_text": "14",
                        "speaker_type": "sub_text",
                        "gender": "male"
                    },
                    {
                        "voice_name": "default",
                        "user_name": "MemeLord2024",
                        "speaker_text": "Yeah, life’s a rollercoaster. But you’ve got a good head on your shoulders. Trust your gut on this one.",
                        "audio_duration_in_frames": 0.0,
                        "hashed_text": "15",
                        "speaker_type": "sub_text",
                        "gender": "female"
                    },
                    {
                        "voice_name": "default",
                        "user_name": "ProGamerMom",
                        "speaker_text": "I will. Here’s hoping it’s the right move in the end.",
                        "audio_duration_in_frames": 0.0,
                        "hashed_text": "16",
                        "speaker_type": "sub_text",
                        "gender": "female"
                    }
                ]
    });

    let model_answer_two = json!({
                "story_type": "interactive",
                "genre": "crazy",
                "fragments": [
                    {
                        "voice_name": "default",
                        "user_name": "DramaDiva2000",
                        "speaker_text": "🚨 My Cousin's Wild Party Prank Almost Ruined Everything—What’s the Move? 🚨",
                        "audio_duration_in_frames": 0.0,
                        "hashed_text": "1",
                        "speaker_type": "title",
                        "gender": "female"
                    },
                    {
                        "voice_name": "default",
                        "user_name": "ChaosCaptain99",
                        "speaker_text": "Alright, so my cousin (18) decided to throw a surprise party for her BFF, but things went totally haywire. I found out she secretly invited my biggest nemesis (17M) to crash the party. The party turned into a full-blown circus, with confetti cannons and unexpected karaoke battles. Everyone’s now talking about the 'Great Prank of 2024.' Should I call her out for ruining the fun or just laugh it off and hope it blows over?",
                        "audio_duration_in_frames": 0.0,
                        "hashed_text": "2",
                        "speaker_type": "sub_text",
                        "gender": "female"
                    },
                    {
                        "voice_name": "default",
                        "user_name": "FunkyMonkey7",
                        "speaker_text": "This is like a scene straight out of a comedy movie. If you stay quiet, the madness might just continue. But if you confront her, you might kill the vibe.",
                        "audio_duration_in_frames": 0.0,
                        "hashed_text": "3",
                        "speaker_type": "sub_text",
                        "gender": "male"
                    },
                    {
                        "voice_name": "default",
                        "user_name": "OopsIDidItAgain",
                        "speaker_text": "Totally! And there’s the aftermath of the party. What if I’m overreacting and it’s all just part of the craziness?",
                        "audio_duration_in_frames": 0.0,
                        "hashed_text": "4",
                        "speaker_type": "sub_text",
                        "gender": "male"
                    },
                    {
                        "voice_name": "default",
                        "user_name": "ConfettiCrazy88",
                        "speaker_text": "Have you tried talking to her about how her prank affected everyone? Maybe she’ll realize she went too far.",
                        "audio_duration_in_frames": 0.0,
                        "hashed_text": "5",
                        "speaker_type": "sub_text",
                        "gender": "female"
                    },
                    {
                        "voice_name": "default",
                        "user_name": "KaraokeKingpin",
                        "speaker_text": "I did. She just laughed it off and said, ‘It’s all in good fun!’ But the damage is done, and I’m stuck in this chaotic mess.",
                        "audio_duration_in_frames": 0.0,
                        "hashed_text": "6",
                        "speaker_type": "sub_text",
                        "gender": "male"
                    },
                    {
                        "voice_name": "default",
                        "user_name": "PartyAnimalXD",
                        "speaker_text": "It’s like she’s living in her own wacky world where nothing’s off-limits. Sometimes people just love the drama, even if it’s a bit too much.",
                        "audio_duration_in_frames": 0.0,
                        "hashed_text": "7",
                        "speaker_type": "sub_text",
                        "gender": "female"
                    },
                    {
                        "voice_name": "default",
                        "user_name": "RaveRascal",
                        "speaker_text": "But what if I’m making a mountain out of a molehill? Or worse, what if her next prank is even crazier?",
                        "audio_duration_in_frames": 0.0,
                        "hashed_text": "8",
                        "speaker_type": "sub_text",
                        "gender": "male"
                    },
                    {
                        "voice_name": "default",
                        "user_name": "MadnessMaker",
                        "speaker_text": "This is a total wild ride. Maybe I should just buckle up and prepare for whatever comes next.",
                        "audio_duration_in_frames": 0.0,
                        "hashed_text": "9",
                        "speaker_type": "sub_text",
                        "gender": "female"
                    },
                    {
                        "voice_name": "default",
                        "user_name": "PartyPooper123",
                        "speaker_text": "I’ve thought about that. But ignoring it might just set the stage for even more outrageous antics.",
                        "audio_duration_in_frames": 0.0,
                        "hashed_text": "10",
                        "speaker_type": "sub_text",
                        "gender": "male"
                    },
                    {
                        "voice_name": "default",
                        "user_name": "EpicFailFrenzy",
                        "speaker_text": "Sometimes, you just have to roll with the punches. But the way things are going, who knows what the next plot twist will be?",
                        "audio_duration_in_frames": 0.0,
                        "hashed_text": "11",
                        "speaker_type": "sub_text",
                        "gender": "female"
                    },
                    {
                        "voice_name": "default",
                        "user_name": "WackyWizard",
                        "speaker_text": "Yeah, the drama never ends. But maybe embracing the chaos is the best way to deal with it.",
                        "audio_duration_in_frames": 0.0,
                        "hashed_text": "12",
                        "speaker_type": "sub_text",
                        "gender": "male"
                    },
                    {
                        "voice_name": "default",
                        "user_name": "CosmicClown",
                        "speaker_text": "Whatever you decide, just remember: you’ve got this. Sometimes, the wildest situations turn into the best stories.",
                        "audio_duration_in_frames": 0.0,
                        "hashed_text": "13",
                        "speaker_type": "sub_text",
                        "gender": "female"
                    },
                    {
                        "voice_name": "default",
                        "user_name": "HypeMasterFlex",
                        "speaker_text": "Thanks! I guess I’ll just have to roll with it and see how this wild ride turns out.",
                        "audio_duration_in_frames": 0.0,
                        "hashed_text": "14",
                        "speaker_type": "sub_text",
                        "gender": "male"
                    },
                    {
                        "voice_name": "default",
                        "user_name": "LOLzForDays",
                        "speaker_text": "Exactly! Life’s a rollercoaster, and sometimes you just have to enjoy the ride. Trust yourself and go with the flow.",
                        "audio_duration_in_frames": 0.0,
                        "hashed_text": "15",
                        "speaker_type": "sub_text",
                        "gender": "female"
                    },
                    {
                        "voice_name": "default",
                        "user_name": "JesterJunkie",
                        "speaker_text": "Will do! Here’s to hoping that the next twist is as fun as this one.",
                        "audio_duration_in_frames": 0.0,
                        "hashed_text": "16",
                        "speaker_type": "sub_text",
                        "gender": "male"
                    }
                ]
    });

    let contents = json!({
        "contents":[
            {
                "role": "user",
                "parts": [
                    {
                        "text": "Generate a crazy conversation about: What should I do if my best friend is trying to sabotage my big event? in this JSON Schema:".to_owned() + &schema
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
                        "text": "Generate a crazy conversation about: What should I do about my cousin's crazy party prank that almost messed up everything? in this JSON Schema:".to_owned() + &schema
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
                     "text": generate_chat_to_llm(genre) + "In this JSON schema:" + &schema
                  }
               ]
            }
         ],
    });

    let chat_prompt = json!({
        "system_instruction": {
            "parts": {
                "text": "You’re part of an active Discord server where everyone’s buzzing about the latest drama. Your mission is to spark an epic conversation with a question so intriguing and intense that it gets everyone talking. Your question should be bold and thought-provoking, touching on themes like betrayal, hidden secrets, or tough moral choices.Here’s how to craft your post: Ask a Provocative Question: Frame your question to grab attention and stir up conversation. For example: If you found out your friend was secretly trying to mess up your biggest project, would you confront them or keep it to yourself? What’s a family secret you’ve kept that could totally change your relationship with them if it got out? Have you ever accidentally revealed something that wrecked a friendship? How did you handle it? Start with a Bang: Begin with a dramatic or shocking revelation related to your question to hook readers right away.Encourage Lively Comments: Ask users to reply with comments of around 200 characters. They should be packed with emotion, vivid details, and personal insights. Aim for comments that spark debate and invite others to share their own experiences. Foster Debate and Interaction: Make sure your story includes unexpected twists or moral dilemmas to keep the conversation lively. Encourage users to react, debate, and share their thoughts in a way that feels genuine and engaging. Example Posts and Comments: Post: 🚨 You discover your best friend is planning to sabotage your big event. 🚨 Do you call them out and risk the friendship, or stay silent and hope it doesn’t blow up? Let’s hear your thoughts! Comment: Whoa, this is wild! 😱 I’d probably confront them but only if I’m sure. Don’t want to be the villain if I’m wrong. What would you do? 😬Comment: I’d keep quiet and find another way to save the event. Confronting them might make things worse. 😰 Anyone been in a similar spot? Comment: Definitely confront them! 🤯 If they’re plotting against me, I need to know and deal with it. 💥 What if they just deny it though 🔥 Jump in and let’s see where this drama goes! 🔥"
            }
        },
        "contents": contents.get("contents"),
         "generationConfig": {
            "response_mime_type": "application/json",
        }
    });

    chat_prompt
}
