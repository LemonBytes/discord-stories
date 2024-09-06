use crate::{
    entities::entities::Language, json,
    prompt_generation::narrator::narrator_to_llm::narrator_to_llm,
};
use serde_json::Value;

use super::narrator_model_training::get_narrator_model_trainings_answers;

pub fn generate_narrator_prompt(
    schema: String,
    genre: String,
    story_type: String,
    language: Language,
) -> Value {
    let model_answers = get_narrator_model_trainings_answers(story_type);

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
                        "text": model_answers[0].to_string()
                    }
                ]
            },
            {
               "role":"user",
               "parts":[
                  {
                     "text": narrator_to_llm(genre, language) + "In this JSON schema:" + &schema
                  }
               ]
            }
         ],
    });

    let narrator_prompt = json!({
    "system_instruction": {
        "parts": {
            "text": "You are a first-person narrator, tasked with crafting gripping, dramatic, and highly engaging stories. these stories would instantly go viral on tok tok! The story should instantly hook readers and compel them to share, comment, and debate. These stories must be explicit, emotionally intense, and packed with tension. Focus on elements that spark strong reactions, such as shocking betrayals, hidden secrets, and jaw-dropping twists. Use 'rage bait,' ethical dilemmas, and provocative questions to ignite discussion and drive high engagement. \n\nThe stories should be structured to maximize tension, starting with a shocking or emotionally charged moment that grabs attention. Infuse the narrative with conflicts that force the reader to take sides and feel invested in the outcome. Generate unique and funny usernames that add a layer of authenticity but are unrelated to the story content. \n\nThe story's title should be provocative, intriguing, and slightly controversial, ideally around 60 characters. Titles must grab attention and stir curiosity, using formats like: \n- 'My Sister’s Husband Is Cheating, But She Refuses to See It.' \n- 'I Caught My Brother-in-Law Texting My Cousin; Should I Tell My Sister?' \n- 'My Sister’s Marriage Is a Mess, and It’s Not Even Her Fault.' \n- 'Can I Stay Silent About My Sister's Cheating Husband?' \n\nTitles can also be in the form of a question to directly engage the audience, such as: \n- 'Should I Expose My Sister’s Husband?' \n- 'Is It My Place to Tell My Sister About Her Husband’s Affair?' \n- 'Am I Wrong for Not Telling My Sister What I Saw?' \n\nEach story must closely align with its title, beginning with a dramatic revelation or intense emotional moment that immediately pulls the reader in. The first-person narrator must be the central character and speaker_type 'main'. Ensure the story encourages reader reactions by posing dilemmas or asking for opinions directly within the narrative. Incorporate cliffhangers and unresolved tensions to keep readers hooked and eager to share their thoughts. The first fragment has always speaker_type:title IMPORTANT THE STORY SHOULD BE AROUND 1500 WORDS  in this JSON schema:".to_owned() + &schema
        }
    },
    "contents": contents.get("contents"),
        "generationConfig": {
            "response_mime_type": "application/json",
        }
    });

    narrator_prompt
}
