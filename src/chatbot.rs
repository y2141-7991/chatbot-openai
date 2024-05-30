use reqwest::{header, Client};
use serde_json::json;
use serde_json::Value;
use std::io;
use std::io::Write;

pub async fn run_chat_loop(
    client: &Client,
    api_key: &str,
    url: &str,
) -> Result<(), reqwest::Error> {
    let mut conversation = String::from("The following is a conversation with an AI assistant.\n");

    loop {
        println!("Human: ");

        io::stdout().flush().unwrap();
        let user_input: String = read_user_input();

        if user_input.to_lowercase() == "quit" || user_input.to_lowercase() == "exit" {
            break;
        }

        conversation.push_str("Human: ");
        conversation.push_str(&user_input);
        conversation.push_str("\nAI: ");

        let json = json!({
            "model": "gpt-3.5-turbo",
            "messages": [
                {"role": "system", "content": conversation}
              ]
        });

        let body = call_api(client, api_key, url, json).await?;
        let api_response = get_ai_response(&body);

        println!("AI: {}", api_response);

        conversation.push_str(api_response);
        conversation.push('\n');
    }

    Ok(())
}

fn get_ai_response(body: &Value) -> &str {
    body["choices"][0]["message"]["content"]
        .as_str()
        .unwrap()
        .trim()
}

pub async fn call_api(
    client: &Client,
    api_key: &str,
    url: &str,
    json: serde_json::Value,
) -> Result<Value, reqwest::Error> {
    let response = client
        .post(url)
        .header(header::AUTHORIZATION, format!("Bearer {}", api_key))
        .header(header::CONTENT_TYPE, "application/json")
        .json(&json)
        .send()
        .await?;

    let body: Value = response.json().await?;
    Ok(body)
}

fn read_user_input() -> String {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();

    user_input.trim().to_string()
}
