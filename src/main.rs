use chatbot_openai::chatbot::run_chat_loop;
use reqwest::Client;

use std::env;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = Client::new();

    let api_key = env::var("OPEN_API_KEY").expect("Do not have OPEN_API_KEY in env var");
    let url = "https://api.openai.com/v1/chat/completions";
    run_chat_loop(&client, &api_key, url).await?;
    Ok(())
}
