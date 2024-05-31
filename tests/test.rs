use chatbot_openai::chatbot::get_ai_response;
use serde_json::json;

#[test]
fn test_get_api_response() {
    let body = json!({
        "choices": [
            {"message": {
                "content": " Hello! How can I help you today?\n"
                }
            }
        ]
    });

    let response = get_ai_response(&body);

    assert_eq!(response, "Hello! How can I help you today?")
}
