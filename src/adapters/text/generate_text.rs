use crate::{
    entities::entities::Language, prompt_generation::generate_prompt::generate_api_prompt,
    write_operations::format_json::format_json,
};
use reqwest::Client;

pub async fn generate_text(client: &Client, language: Language) {
    let provider = gcp_auth::provider().await;
    let scopes: &[&str; 1] = &["https://www.googleapis.com/auth/cloud-platform"];
    let token = provider.unwrap().token(scopes).await;
    let project_id: &str = dotenv!("PROJECT_ID");
    let model_id: &str = dotenv!("MODEL_ID");

    let url = format!(
        "https://us-central1-aiplatform.googleapis.com/v1/projects/{}/locations/europe-central2/publishers/google/models/{}:generateContent",
        project_id, model_id
    );

    let story_prompt = generate_api_prompt(language);

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
