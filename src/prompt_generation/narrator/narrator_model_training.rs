use serde_json::{json, Value};

pub fn get_narrator_model_trainings_answers(story_type: String) -> Vec<Value> {
    let model_answer = json!({
            "story_type": story_type.to_owned(),
            "genre": "drama",
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
                    "speaker_text": "My sister (33) has been married to her husband (34) for nearly 10 years, and they have 3 kids together. But something is off. I’ve noticed how our distant cousin (32F), who recently moved in with them after leaving her abusive husband, has been getting too close to him. It started with harmless flirting, but now she’s outright grinding on him at family gatherings. My sister is uncomfortable, but her husband just laughs it off. The other day, I caught our cousin texting someone during a party. I couldn’t believe it when I saw my brother-in-law’s name on her phone. She claimed it was nothing, but I know what I saw. Should I tell my sister and risk tearing our family apart, or is it better to stay silent and hope it’s all just a misunderstanding? I decided to follow our cousin discreetly to uncover the truth. Last night, I saw her sneaking out of the house, and my heart raced as I followed her to a dimly lit bar on the edge of town. To my shock, there was my brother-in-law, laughing and drinking with her. They seemed way too comfortable together. I saw them leave together, and my heart sank. I managed to overhear their conversation as I was about to leave. They were talking about a 'secret trip' they were planning. My brother-in-law’s voice was filled with a kind of intimacy that I knew wasn’t just friendship. I’m devastated and torn. I know I have to tell my sister, but I’m afraid of the consequences. What if she hates me for it? What if it breaks up our family? I need to decide quickly, but I don’t know what’s right. What would you do in my shoes? Should I confront them or gather more evidence before telling my sister? I’m at a loss and need advice desperately.",
                    "audio_duration_in_frames": 0.0,
                    "hashed_text": "2",
                    "speaker_type": "main",
                    "gender": "female"
                },
            ]
    });

    let model_answer_two = json!({
            "story_type": story_type.to_owned(),
            "genre": "crazy",
            "fragments": [
                {
                    "voice_name": "default",
                    "user_name": "truthseeker42",
                    "speaker_text": "My Best Friend’s Fiancé Is Hiding a Major Secret, and I’m Torn About What to Do.",
                    "audio_duration_in_frames": 0.0,
                    "hashed_text": "1",
                    "speaker_type": "title",
                    "gender": "male"
                },
                {
                    "voice_name": "default",
                    "user_name": "truthseeker42",
                    "speaker_text": "My best friend, Emily (29), is about to marry the love of her life, Mark (31). I’ve been Emily’s confidant for years, and I was thrilled when she got engaged. But lately, I’ve stumbled upon something that could change everything. I was at a mutual friend's party when I overheard Mark talking to a friend about 'taking care of unfinished business' and mentioned something about a big decision he made 'before meeting Emily.' I did some digging and found out that Mark was previously married and has a child with his ex-wife, who lives in another state. Emily has no idea about this. I found the court records and a social media profile for Mark’s child. My heart is breaking because I know Emily deserves to know the truth before she marries him. But I’m also terrified of what this will do to her and our friendship. I’ve always promised to be honest with her, but I’m afraid this might ruin her happiness. Should I reveal Mark’s secret or let Emily enjoy her engagement in blissful ignorance? I decided to confront Mark to get more information, hoping he’d come clean on his own. I asked him casually about his past, and he seemed nervous but managed to deflect. When I pressed further, he admitted to having been married before but downplayed it as a minor detail. He made it sound like a brief, insignificant part of his life. When I confronted him about his child, he got defensive and said it was complicated and that he’s trying to be a good father, even if he’s not involved in their daily life. Now, I’m even more conflicted. Mark’s story makes him seem like he’s trying to move on and make things right, but Emily deserves to know the full truth. If I tell her, it could potentially break up the wedding and cause a lot of pain. If I don’t, am I failing as a friend? What would you do? Should I tell Emily everything I know, or should I respect Mark’s wish for privacy and let things be? My mind is racing, and I need your advice.",
                    "audio_duration_in_frames": 0.0,
                    "hashed_text": "2",
                    "speaker_type": "main",
                    "gender": "male"
                },
            ]
    });

    let model_answer_three = json!({

            "story_type": story_type.to_owned(),
            "genre": "sad",
            "fragments": [
                {
                    "voice_name": "default",
                    "user_name": "IHeartVeggis",
                    "speaker_text": "I Found Out My Best Friend’s Husband Is Living a Double Life, and I Don’t Know What to Do.",
                    "audio_duration_in_frames": 0.0,
                    "hashed_text": "1",
                    "speaker_type": "title",
                    "gender": "female"
                },
                {
                    "voice_name": "default",
                    "user_name": "IHeartVeggis",
                    "speaker_text": "My best friend, Laura (30), has been happily married to James (32) for the past five years. They seem like the perfect couple—always affectionate, always supportive. I’ve always admired their relationship, and I was thrilled when they welcomed their first child, a beautiful baby girl, a year ago. But recently, I’ve stumbled upon something that could shatter Laura’s world. \n\nIt all started when I noticed James acting a bit off. He was constantly glued to his phone and seemed unusually secretive. I initially thought he was just stressed from work, but my concern grew when I saw him meeting up with someone late at night. Curiosity got the best of me, and I decided to follow him discreetly. To my shock, James was meeting with a woman at a hotel. They were extremely affectionate, and it was clear they had a close relationship. \n\nThe next day, I did some digging and discovered that the woman was someone he had been romantically involved with before meeting Laura. What’s even more troubling is that I found a series of messages between them discussing their ongoing affair and plans for the future. James had been lying to Laura about working late and other excuses, while maintaining this relationship. \n\nI’m devastated for Laura. She deserves to know the truth, but I’m torn. If I tell her, it might destroy her marriage and family. I’ve been close to Laura for years, and I don’t want to be the bearer of such painful news. I’ve thought about confronting James directly to see if he would come clean on his own, but I’m afraid that would only make things worse. What if he turns the tables and accuses me of overstepping? What if Laura finds out and blames me for not telling her sooner? \n\nTo complicate matters, I’ve also noticed James acting increasingly distant from their daughter. He’s missed several important milestones and family events, claiming work commitments, but I’m beginning to suspect that it’s because he’s dividing his time between two lives. Laura is a wonderful mother and wife, and she doesn’t deserve to be misled. I’m torn between protecting my friend and maintaining my integrity. Should I come forward and reveal everything I’ve uncovered, or should I let Laura continue living in ignorance? I need advice desperately. What would you do if you were in my shoes?",
                    "audio_duration_in_frames": 0.0,
                    "hashed_text": "2",
                    "speaker_type": "main",
                    "gender": "female"
                }
            ]

    });

    let answers = vec![model_answer, model_answer_two, model_answer_three];

    answers
}
