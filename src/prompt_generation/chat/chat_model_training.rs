use serde_json::{json, Value};

pub fn get_model_trainings_answers(story_type: String) -> Vec<Value> {
    let model_answer = json!({
        "story_type": story_type,
        "genre": "drama",
        "fragments": [
            {
                "voice_name": "default",
                "user_name": "BabeAlert",
                "speaker_text": "Hey, what are you guys doing?",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "1",
                "speaker_type": "sub_text",
                "gender": "female"
            },
            {
                "voice_name": "default",
                "user_name": "ChillBro92",
                "speaker_text": "Getting drinks with the boys! What's up?",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "2",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "BabeAlert",
                "speaker_text": "Oh yeah? Who's there with you?",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "3",
                "speaker_type": "sub_text",
                "gender": "female"
            },
            {
                "voice_name": "default",
                "user_name": "ChillBro92",
                "speaker_text": "Just the usual crew. Not too busy tonight.",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "4",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "BabeAlert",
                "speaker_text": "Yeah, I bet. Remember what happened last time? 🙄",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "5",
                "speaker_type": "sub_text",
                "gender": "female"
            },
            {
                "voice_name": "default",
                "user_name": "ChillBro92",
                "speaker_text": "Relax, babe. We're just chilling.",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "6",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "PartyPal99",
                "speaker_text": "Chilling? More like getting WASTED! 😂",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "7",
                "speaker_type": "sub_text",
                "gender": "female"
            },
            {
                "voice_name": "default",
                "user_name": "BabeAlert",
                "speaker_text": "Wait, are you drunk already? Seriously?!",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "8",
                "speaker_type": "sub_text",
                "gender": "female"
            },
            {
                "voice_name": "default",
                "user_name": "ChillBro92",
                "speaker_text": "No, just a couple of drinks. It's all good.",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "9",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "BabeAlert",
                "speaker_text": "I don't trust you when you say that. Are you coming home soon?",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "10",
                "speaker_type": "sub_text",
                "gender": "female"
            },
            {
                "voice_name": "default",
                "user_name": "ChillBro92",
                "speaker_text": "Yeah, yeah, after this drink. Why are you so paranoid?",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "11",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "BabeAlert",
                "speaker_text": "Paranoid?! Maybe because last time you didn't come home until 3 AM! 😡",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "12",
                "speaker_type": "sub_text",
                "gender": "female"
            },
            {
                "voice_name": "default",
                "user_name": "DrunkKing420",
                "speaker_text": "3 AM? That's rookie hours! Let's make it 4 tonight! 😂",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "13",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "BabeAlert",
                "speaker_text": "Who the hell is that? Are you even listening to me?!",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "14",
                "speaker_type": "sub_text",
                "gender": "female"
            },
            {
                "voice_name": "default",
                "user_name": "ChillBro92",
                "speaker_text": "Relax, babe. Just having fun. I'll be home soon.",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "15",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "NosyNeighbor",
                "speaker_text": "Is that why you were stumbling down the street last week? 😏",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "16",
                "speaker_type": "sub_text",
                "gender": "female"
            },
            {
                "voice_name": "default",
                "user_name": "BabeAlert",
                "speaker_text": "Stumbling?! What are they talking about??",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "17",
                "speaker_type": "sub_text",
                "gender": "female"
            },
            {
                "voice_name": "default",
                "user_name": "ChillBro92",
                "speaker_text": "Ignore them. They're just messing around.",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "18",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "PartyPal99",
                "speaker_text": "Messing around? Like when you lost your wallet and cried for an hour? 🤣",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "19",
                "speaker_type": "sub_text",
                "gender": "female"
            },
            {
                "voice_name": "default",
                "user_name": "BabeAlert",
                "speaker_text": "Oh my god, I can't believe this. Are you driving home like this??",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "20",
                "speaker_type": "sub_text",
                "gender": "female"
            },
            {
                "voice_name": "default",
                "user_name": "ChillBro92",
                "speaker_text": "I'm fine. Stop worrying. I'll be home in no time.",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "21",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "PartyPal99",
                "speaker_text": "Yeah, if he doesn't end up in a ditch first! 😬",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "22",
                "speaker_type": "sub_text",
                "gender": "female"
            },
            {
                "voice_name": "default",
                "user_name": "BabeAlert",
                "speaker_text": "This isn't funny. I swear, if you don't come home right now, we're done!",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "23",
                "speaker_type": "sub_text",
                "gender": "female"
            },
            {
                "voice_name": "default",
                "user_name": "ChillBro92",
                "speaker_text": "Whoa, calm down! I'm leaving now, OK? Chill out.",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "24",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "NosyNeighbor",
                "speaker_text": "Better hope the cops don't catch you. Saw a patrol near the bar earlier. 👀",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "25",
                "speaker_type": "sub_text",
                "gender": "female"
            },
            {
                "voice_name": "default",
                "user_name": "ChillBro92",
                "speaker_text": "Great. Just what I needed. Thanks a lot. 🙄",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "26",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "BabeAlert",
                "speaker_text": "Just get home safe. Please. I'm really worried now. 💔",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "27",
                "speaker_type": "sub_text",
                "gender": "female"
            },
            {
                "voice_name": "default",
                "user_name": "ChillBro92",
                "speaker_text": "I'll be fine, babe. See you soon. Love you.",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "28",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "BabeAlert",
                "speaker_text": "Love you too. Just... please hurry.",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "29",
                "speaker_type": "sub_text",
                "gender": "female"
            }
        ]
    });

    let model_answer_two = json!({
        "story_type": story_type,
        "genre": "drama",
        "fragments": [
            {
                "voice_name": "default",
                "user_name": "BroGuy123",
                "speaker_text": "Dude, you're not gonna believe what happened.",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "1",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "ChadKing",
                "speaker_text": "What's up, man? You good?",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "2",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "BroGuy123",
                "speaker_text": "My girlfriend just blew through all my savings. Every. Single. Cent.",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "3",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "ChadKing",
                "speaker_text": "Bro, what?! On what? Shoes and makeup?",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "4",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "BroGuy123",
                "speaker_text": "Worse. She threw a party without telling me. Rented out a whole club, VIP section and everything.",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "5",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "MoneyMan21",
                "speaker_text": "LOL that's what happens when you let your girl control the money. Rookie mistake, dude.",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "6",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "BroGuy123",
                "speaker_text": "I didn't LET her! She just did it behind my back. Now I'm broke!",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "7",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "ChadKing",
                "speaker_text": "Bro, you gotta put your foot down. Can't let her walk all over you like that.",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "8",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "BroGuy123",
                "speaker_text": "What am I supposed to do? I can't just kick her out. I love her, man...",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "9",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "AlphaBro",
                "speaker_text": "Love? That's where you messed up, bro. You can't let emotions make you soft. She's got you wrapped around her finger.",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "10",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "BroGuy123",
                "speaker_text": "It's not like that... or maybe it is. I don't know.",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "11",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "MoneyMan21",
                "speaker_text": "Bro, first rule of being a man: never let a woman control your money. Now look where you are.",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "12",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "ChadKing",
                "speaker_text": "Facts. You're supposed to be the provider, not the other way around.",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "13",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "BroGuy123",
                "speaker_text": "So what, I just dump her? After everything?",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "14",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "AlphaBro",
                "speaker_text": "If she doesn't respect you or your money, she doesn't respect you, period. You gotta take control, man.",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "15",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "MoneyMan21",
                "speaker_text": "Honestly, it sounds like she's just using you. No way she's staying if you're broke now.",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "16",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "BroGuy123",
                "speaker_text": "She said she loves me... but now I'm starting to doubt it.",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "17",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "ChadKing",
                "speaker_text": "Bro, love isn't spending all your money and leaving you high and dry. That's manipulation.",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "18",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "AlphaBro",
                "speaker_text": "Real talk. She's gotta go. You need to focus on you and get your life back on track.",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "19",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "BroGuy123",
                "speaker_text": "But what if I can't? What if I really do need her?",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "20",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "MoneyMan21",
                "speaker_text": "Bro, that's the weak mindset. You don't need anyone but yourself. Don't let her make you soft.",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "21",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "ChadKing",
                "speaker_text": "Exactly. You're better off solo until you find someone who respects you. No more leeches.",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "22",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "AlphaBro",
                "speaker_text": "Time to level up, bro. Leave the dead weight behind.",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "23",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "BroGuy123",
                "speaker_text": "You're right... It's just hard, you know?",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "24",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "ChadKing",
                "speaker_text": "Of course it is. But you'll thank yourself later. Trust me.",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "25",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "BroGuy123",
                "speaker_text": "I just can't believe she would do this. I thought we were building a future together.",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "25",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "AlphaBro",
                "speaker_text": "Building a future? She's building a future for herself on your dime. There's a big difference.",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "26",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "MoneyMan21",
                "speaker_text": "Bro, it’s about self-respect. If she cared, she wouldn't have drained you dry.",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "27",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "ChadKing",
                "speaker_text": "Get your finances in order and focus on yourself for a while. You’ll see things more clearly.",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "28",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "BroGuy123",
                "speaker_text": "Maybe you're right. I need to get my life back on track. But it's hard to let go.",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "29",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "AlphaBro",
                "speaker_text": "Hard, but necessary. A man needs to take control and not let anyone mess with his future.",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "30",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "MoneyMan21",
                "speaker_text": "It’s a lesson learned. Next time, keep your finances in check and set boundaries.",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "31",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "ChadKing",
                "speaker_text": "You got this. Focus on your goals and don’t let anyone derail you again.",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "32",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "BroGuy123",
                "speaker_text": "Thanks for the support, guys. I think it’s time I made some changes.",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "33",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "AlphaBro",
                "speaker_text": "Glad to hear it. Take charge and show everyone what you're really made of.",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "34",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "MoneyMan21",
                "speaker_text": "And remember, bro: Never let anyone mess with your money or your mind.",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "35",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "ChadKing",
                "speaker_text": "Yeah, stay strong. You’ve got this. 💪",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "36",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "BroGuy123",
                "speaker_text": "Alright. I’m going to handle this and get things sorted. Thanks for having my back.",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "37",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "AlphaBro",
                "speaker_text": "Anytime, bro. Keep us updated. You’re stronger than you think.",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "38",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "MoneyMan21",
                "speaker_text": "Yeah, stay on top of things and keep that confidence high. You got this!",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "39",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "ChadKing",
                "speaker_text": "For sure. We'll be here for you. Good luck, bro.",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "40",
                "speaker_type": "sub_text",
                "gender": "male"
            }
        ]
    });

    let model_answer_three = json!({
        "story_type": story_type,
        "genre": "drama",
        "fragments": [
            {
                "voice_name": "default",
                "user_name": "SpaceWatcher",
                "speaker_text": "OMG, guys! Did you see the news? 🚨",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "1",
                "speaker_type": "sub_text",
                "gender": "female"
            },
            {
                "voice_name": "default",
                "user_name": "ConspiracyDude",
                "speaker_text": "What now? Another UFO sighting? 😂",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "2",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "SpaceWatcher",
                "speaker_text": "No, even crazier! NASA just announced they found a message from aliens! 🛸",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "3",
                "speaker_type": "sub_text",
                "gender": "female"
            },
            {
                "voice_name": "default",
                "user_name": "SciFiFan",
                "speaker_text": "WHAT?! Is this for real?! 😱",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "4",
                "speaker_type": "sub_text",
                "gender": "female"
            },
            {
                "voice_name": "default",
                "user_name": "ConspiracyDude",
                "speaker_text": "Called it! I've been saying this for YEARS! The truth is finally out!",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "5",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "TechieGirl",
                "speaker_text": "Hold up, what does the message say? Is it friendly or... 👽",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "6",
                "speaker_type": "sub_text",
                "gender": "female"
            },
            {
                "voice_name": "default",
                "user_name": "SpaceWatcher",
                "speaker_text": "That's the thing. It's just one word: 'RUN'. 😳",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "7",
                "speaker_type": "sub_text",
                "gender": "female"
            },
            {
                "voice_name": "default",
                "user_name": "SciFiFan",
                "speaker_text": "Oh. My. God. Are we being invaded?!",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "8",
                "speaker_type": "sub_text",
                "gender": "female"
            },
            {
                "voice_name": "default",
                "user_name": "ConspiracyDude",
                "speaker_text": "Told ya! This is just the beginning. Time to prep those bunkers, people!",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "9",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "TechieGirl",
                "speaker_text": "What if it's just a warning? Maybe they're trying to help us?",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "10",
                "speaker_type": "sub_text",
                "gender": "female"
            },
            {
                "voice_name": "default",
                "user_name": "SpaceWatcher",
                "speaker_text": "Or maybe it's too late... 😰",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "11",
                "speaker_type": "sub_text",
                "gender": "female"
            },
            {
                "voice_name": "default",
                "user_name": "SciFiFan",
                "speaker_text": "I can't believe this is happening. What do we do now?",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "12",
                "speaker_type": "sub_text",
                "gender": "female"
            },
            {
                "voice_name": "default",
                "user_name": "ConspiracyDude",
                "speaker_text": "It's survival mode, baby! Stock up on supplies and get ready for the worst!",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "13",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "SpaceWatcher",
                "speaker_text": "NASA said more details are coming. Stay tuned. 😬",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "14",
                "speaker_type": "sub_text",
                "gender": "female"
            },
            {
                "voice_name": "default",
                "user_name": "SciFiFan",
                "speaker_text": "I'm freaking out! What if this is the end?! 😱",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "15",
                "speaker_type": "sub_text",
                "gender": "female"
            },
            {
                "voice_name": "default",
                "user_name": "ConspiracyDude",
                "speaker_text": "It's not the end—it's the truth coming out! Finally, we're being told the real story!",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "16",
                "speaker_type": "sub_text",
                "gender": "male"
            },
            {
                "voice_name": "default",
                "user_name": "TechieGirl",
                "speaker_text": "Well, whatever it is, we better be ready. This is about to go viral everywhere.",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "17",
                "speaker_type": "sub_text",
                "gender": "female"
            },
            {
                "voice_name": "default",
                "user_name": "SpaceWatcher",
                "speaker_text": "It's already trending! Everyone is talking about it. #AlienMessage #Run",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "18",
                "speaker_type": "sub_text",
                "gender": "female"
            },
            {
                "voice_name": "default",
                "user_name": "SciFiFan",
                "speaker_text": "I'm glued to my screen now. This is unreal.",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "19",
                "speaker_type": "sub_text",
                "gender": "female"
            },
            {
                "voice_name": "default",
                "user_name": "ConspiracyDude",
                "speaker_text": "Told ya so. The truth is out there, and it's finally here.",
                "audio_duration_in_frames": 0.0,
                "hashed_text": "20",
                "speaker_type": "sub_text",
                "gender": "male"
            }
        ]
    });

    let answers = vec![model_answer, model_answer_two, model_answer_three];

    answers
}
