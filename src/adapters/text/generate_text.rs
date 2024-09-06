use crate::{
    entities::entities::{Language, StoryType},
    prompt_generation::{
        generate_prompt::generate_api_prompt, generate_schema::generate_gpt_schema,
        story_config::get_topic,
    },
    write_operations::format_json::format_json,
};
use reqwest::Client;
use serde_json::json;

pub async fn generate_text_vertex(client: &Client, language: Language, story_type: StoryType) {
    let provider = gcp_auth::provider().await;
    let scopes: &[&str; 1] = &["https://www.googleapis.com/auth/cloud-platform"];
    let token = provider.unwrap().token(scopes).await;
    let project_id: &str = dotenv!("PROJECT_ID");
    let model_id: &str = dotenv!("MODEL_ID");

    let url = format!(
        "https://us-central1-aiplatform.googleapis.com/v1/projects/{}/locations/europe-central2/publishers/google/models/{}:generateContent",
        project_id, model_id
    );

    let story_prompt = generate_api_prompt(language, story_type);

    // Send the POST request
    let response = client
        .post(&url)
        .bearer_auth(token.unwrap().as_str())
        .header("Content-Type", "application/json")
        .json(&story_prompt)
        .send()
        .await
        .unwrap();

    // Handle the response
    if response.status().is_success() {
        let resp_json: serde_json::Value = response.json().await.unwrap();

        let candidates = &resp_json.get("candidates").unwrap();
        let first_candidates = candidates.get(0).unwrap();
        let content = first_candidates.get("content").unwrap();
        let parts = content.get("parts").unwrap();
        let first_part = parts.get(0).unwrap();
        let text = first_part.get("text").unwrap();

        println!("{:?}", text);
        // Convert the Value back into a pretty-printed JSON string
        let input = serde_json::to_string_pretty(text).unwrap();

        let _ = format_json(&input);
    } else {
        eprintln!("Request failed with status: {:?}", response);
    }
}

pub async fn generate_text_gpt(client: &Client, language: Language, story_type: StoryType) {
    let project_id: &str = dotenv!("GPT_PROJECT_ID");
    let model_id: &str = dotenv!("MODEL_ID");
    let api_key: &str = dotenv!("GPT_API_KEY");
    let url = format!("https://api.openai.com/v1/chat/completions",);

    //let story_prompt = generate_api_prompt(language, story_type);

    let request_body = json!({
      "model": "gpt-4o",
      "messages": [
        {
          "role": "system",
          "content": format!("Du bist ein Geschichtenschreiber, das extrem fesselnde, extreme und humorvolle Gespräched die auf einem Discord-Server stattfinden generiert. Diese Gespräche sollten so krank und zum fremdschämen wie möglich, aber realistisch sein. Der Gesprächsfaden sollte voller Wendungen und starker Emotionen sein, aber so nah wie möglich am Thema, und im Hauptthema so detailiert wie möglich sein, um das Publikum solange wie möglich Aufmerksamkeit zu entlocken, sei sehr kreativ in diesem Punkt!. Der Dialog sollte aus MINDESTENS 25 Austauschen zwischen Benutzern bestehen, die alle unterschiedliche Persönlichkeiten und starke Meinungen haben und vulgär reden. Diese Persönlichkeiten sind 18 Jahre alt, und du solltest neusten Jugendwörter nutzen. Verwende zufällig EINES dieser Themen:  1. sehr berühmte Promis. 2. Sexuelle Traumata. 3. Beziehungsprobleme. 4. wilde Lebensgeschichten. 5. schockierende Verrate. 6. Die Rechte Politik der AFD. 7. kontroverse Meinungen. 8. Verschwörungstheorien. 9. Absurde Mitarbeiter und Chef Situationen. 10. gestörte Beziehungen. 11. kaputte asoziale Familien. Sorge dafür, dass das Gespräch nachvollziehbar, kontrovers, krank und todes witzig ist, mit genug Wendungen, um die Zuschauer solange wie möglich bei der Stange zu halten. 'Wichtige Anforderungen:Hohe Engagementrate: Starte mit einem starken Einstieg, der innerhalb der ersten paar Zeilen Aufmerksamkeit erregt und eine intensive oder spannende Situation aufbaut.Schnelles Tempo: Halte den Dialog in Bewegung, ohne langweilige Momente; jeder Kommentar eines Nutzers sollte die Spannung deutlich erhöhen, extremen oder schwarzen Humor hinzufügen oder die Situation komplett eskalieren. Deutliche Charaktere: Erfinde absurde Persönlichkeiten. Verwende kreative, vulgäre und schräge Benutzernamen (maximal 10 Zeichen) und stelle sicher, dass verschiedene Perspektiven sehr deutlich und lautstark vertreten sind. Aufmerksamkeit: Sorge dafür, dass der Inhalt so absurd ist das man nicht wegschauen kann sowohl als auch, unterhaltsam und teilenswert. und den Drang auslöst, zu kommentieren. Das Gespräch sollte Elemente enthalten, die wahrscheinlich weit verbreitet und diskutiert werden oder für Fremdscham oder Wut sorgen. Breche die Vierte Wand: Die Nutzer sollten die Vierte Wand durchbrechen und direkte Fragen an die Zuschauer stellen. Stelle sicher, dass die Plot-Twists häufig, unerwartet und extrem sind und so vulgär wie möglich oder einfach nur extreme dumm, um den Unterhaltungswert in die Höhe zu treiben. DEINE AUSGABE IST IN DIESEN JSON SCHEMA:{}", &generate_gpt_schema().to_string()),
        },
        {
          "role": "user",
          "content": "Starte ein Gespräch, das die Zuhörer fesselt. Nutze all dein Wissen über Storytelling. Sei witzig, interessant, extrem und so vulgär wie möglich!"
        }
      ],
      "response_format":  {"type": "json_object"}
    });

    // Send the POST request
    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .body(request_body.to_string())
        .send()
        .await
        .expect("Failed to send request");

    // Handle the response
    if response.status().is_success() {
        let resp_json: serde_json::Value = response.json().await.unwrap();

        let choices = &resp_json.get("choices").unwrap();
        let content = choices.get(0).unwrap();
        let message = content.get("message").unwrap();
        let text = message.get("content").unwrap();

        println!("{:?}", text);
        // Convert the Value back into a pretty-printed JSON string
        let input = serde_json::to_string_pretty(text).unwrap();

        let _ = format_json(&input);
    } else {
        eprintln!("Request failed with status: {:?}", response);
    }
}
