use crate::{
    entities::entities::Language, json, prompt_generation::chat::chat_to_llm::chat_to_llm,
};
use serde_json::Value;

use super::chat_model_training::get_model_trainings_answers;

pub fn generate_chat_prompt(schema: String, story_type: String, language: Language) -> Value {
    let model_answers = get_model_trainings_answers(story_type);

    let contents = json!({
        "contents":[
            {
                "role": "user",
                "parts": [
                    {
                        "text": "Generate a crazy conversation about: What should I do if my best friend is trying to sabotage my big event? in this JSON"
                    }
                ]
            },
            {
                "role": "model",
                "parts": [
                    {
                        "text": model_answers[0].to_string()
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
                        "text": model_answers[1].to_string()
                    }
                ]
            },
            {
                "role": "user",
                "parts": [
                    {
                        "text": "Generate a crazy conversation about: a shocking announcement by NASA, revealing a mysterious one-word message from aliens: 'RUN.' As the news spreads, the characters react with panic, excitement, and conspiracy theories. The story captures the fear and uncertainty of the situation as it unfolds, making it a perfect scenario for a viral social media storm. in this JSON Schema:".to_owned() + &schema
                    }
                ]
            },
            {
                "role": "model",
                "parts": [
                    {
                        "text": model_answers[2].to_string()
                    }
                ]
            },
            {
               "role":"user",
               "parts":[
                  {
                     "text": chat_to_llm(language) + " IMPORTANT: add at least 30 Messages to the Conversation"
                  }
               ]
            }
         ],
    });

    let chat_prompt = json!({
        "system_instruction": {
                "parts": {
                    "text": "You’re part of a lively Discord server where drama and intrigue are always around the corner. Your objective is to craft a captivating and dramatic chat-based story involving multiple characters. The goal is to create content that will keep viewers engaged and wanting more, perfect for a viral TikTok series. Sometimes the Characters break the 4th wall so that the viewer wants to leave a comment use rage bait. Be detailed so that the viewer knows whats going on: Follow these steps to create a compelling narrative:\n\n1. **Initial Interaction**: Start with Character A reaching out to Character B about a specific activity or situation. Begin casually, but hint at underlying tension or concern. For example, Character A might message Character B about unusual or worrying behavior. **Text Overlay/Voiceover:** 'Hey, [Character B], you good? You’ve been acting kinda weird lately... 😟' The scene should set up intrigue right away.\n\n2. **Rising Tension**: Show Character A's growing concern or frustration as Character B is slow to respond or involved in a troubling situation. This could range from a late-night event to risky behavior. As the conversation progresses, **increase the suspense with tense music and dynamic text edits**. **Text Overlay/Voiceover:** 'Why aren’t you answering? What’s going on? 😰' Keep viewers on edge.\n\n3. **Group Reactions**: Introduce additional characters who react to the situation in various ways. This creates drama and variety in perspectives, fueling the intrigue:\n\n  - **Friend 1**: Shows support for Character A or expresses similar concerns. **Text Overlay/Voiceover:** 'I’m worried too. This isn’t like [Character B]. 😟'\n  - **Friend 2**: Displays indifference or annoyance with the situation. **Text Overlay/Voiceover:** 'Chill, it’s probably nothing. 🙄'\n  - **Acquaintance 1**: Offers advice or potential solutions. **Text Overlay/Voiceover:** 'Maybe we should check in IRL? What if something’s wrong? 🤔'\n  - **Acquaintance 2**: Makes jokes or downplays the issue. **Text Overlay/Voiceover:** 'Bet [Character B] is just ghosting us for the memes. 😂'\n  - **Other Characters**: Include random members of the group chat who chime in with their thoughts, questions, or reactions. **Text Overlay/Voiceover:** 'What’s happening? Did something go down? 👀'\n\n   Use fast cuts, message pop-up sound effects, and a rising beat to create a chaotic yet engaging atmosphere.\n\n4. **Climax**: Reach the peak of conflict or tension with misunderstandings, disagreements, or emotional outbursts. Focus on the interactions among Character A, Character B, and the additional characters. Highlight how the drama unfolds with **split-screen reactions and intense music**. **Text Overlay/Voiceover:**\n   - **Character A:** 'Why are you ignoring me? Do you even care? 💔'\n   - **Character B:** 'It’s not what you think. You don’t understand… 😡'\n\n   Other characters take sides, and emotions run high. The Discord server explodes with opinions, creating a dramatic and intense scene.\n\n5. **Resolution**: Conclude with an unresolved or open-ended outcome, leaving the situation tense or uncertain. This creates a cliffhanger that will keep viewers hooked and eager for the next TikTok episode. **Text Overlay/Voiceover:** 'And just like that… everything changed. 😶'\n\n   **Call to Action:** End the TikTok with a teaser: 'What happens next? Stay tuned… 👀' Encourage viewers to comment and share their thoughts, asking questions like, 'Who do you think is in the wrong? Comment below! 💬'\n\n**IMPORTANT:** Ensure that the conversation remains engaging and realistic, with a focus on emotional intensity and character dynamics. Use **trending sounds, dynamic edits, and engaging text overlays** to maximize viewer engagement and make each TikTok part feel dramatic and immersive. Avoid using hashtags within the text.\n\n**Example Scenario and Reactions:**\n- **Initial Message:** 'Hey [Character B], I noticed you haven’t been yourself lately. Everything okay?'\n- **Rising Tension:** Character A becomes increasingly concerned as Character B delays responses or engages in troubling behavior. The chat starts to buzz with confusion and worry.\n- **Group Reactions:** Friends and randoms in the chat: 'I’m worried.' / 'Not a big deal.' / 'Did something happen?'\n- **Climax:** Heated exchange, emotions explode.\n- **Resolution:** Be creative in the discussion and be detailed, add as much persons with different personalities as you want, these personalities should show extense emotions and traits of psychological disorders and insult each other harchly and adress with username.\n\n**In this JSON schema:** \" + schema ".to_owned() + &schema
                }
        },
        "contents": contents.get("contents"),
         "generationConfig": {
            "response_mime_type": "application/json",
        }
    });

    chat_prompt
}
