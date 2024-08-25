use crate::{adapters::audio::generate_audio::generate_audio, entities::entities::Language};
use reqwest::Client;

pub async fn handle_generate_audio(client: Client, language: Language) {
    let mut attempts = 0;
    const MAX_ATTEMPTS: u32 = 3;

    while attempts < MAX_ATTEMPTS {
        match generate_audio(client.clone(), language.clone()).await {
            Ok(_) => {
                // Success, break out of the loop and continue with the rest of the program.
                println!("Audio generation succeeded.");
                break;
            }
            Err(e) => {
                // Increment attempts and print the error
                attempts += 1;
                println!("Attempt {} failed with error: {}. Retrying...", attempts, e);

                // If the maximum attempts have been reached, handle the failure case
                if attempts >= MAX_ATTEMPTS {
                    println!("Failed after {} attempts. Aborting.", attempts);
                    // You can handle this case further, such as logging the failure or exiting the program.
                    break;
                }
            }
        }
    }
}
