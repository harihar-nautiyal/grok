use anyhow::Result;
use gemini_rust::GeminiBuilder;
use gemini_rust::prelude::*;
use std::env;

async fn generate_short_reply(prompt: &str) -> Result<String> {
    let mut gemini = GeminiBuilder::new()
        .api_key(std::env::var("GEMINI_API_KEY")?)
        .build()
        .await?;

    let model = Model::Gemini1_5Flash;

    let request = ContentBuilder::text(prompt)
        .model(model)
        .max_output_tokens(256)
        .build();

    let response = gemini.generate_content(&request).await?;

    if let Some(candidate) = response.candidates.into_iter().next() {
        Ok(candidate.content_text().unwrap_or_default())
    } else {
        Ok(String::from("(no reply)"))
    }
}
