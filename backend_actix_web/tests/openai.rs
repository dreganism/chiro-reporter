use backend_actix_web::services::openai::call_openai_chat;
use std::env;
use dotenv::dotenv;

#[tokio::test]
#[ignore]
async fn test_call_openai_chat_works() {
    dotenv().ok(); // 👈 Explicitly load .env for test

    assert!(
        env::var("OPENAI_API_KEY").is_ok(),
        "OPENAI_API_KEY should be set"
    );

    let result = call_openai_chat("Test prompt".to_string()).await;
    assert!(result.is_ok(), "Expected Ok(_), got error: {:?}", result.err());

    let output = result.unwrap();
    assert!(
        !output.trim().is_empty(),
        "Expected non-empty response from OpenAI"
    );
}
