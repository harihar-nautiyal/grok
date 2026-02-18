use dotenv::dotenv;
use gemini_rust::{GeminiBuilder, Model};
use grok::handlers::call::call;
use grok::handlers::invite::auto_join;
use matrix_sdk::Client;
use matrix_sdk::Room;
use matrix_sdk::config::SyncSettings;
use matrix_sdk::ruma::UserId;
use matrix_sdk::ruma::events::room::message::OriginalSyncRoomMessageEvent;
use std::env;
use std::sync::Arc;
use tracing::info;
use tracing::subscriber::set_global_default;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn,grok=info"));

    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(false)
        .with_line_number(false)
        .with_thread_ids(true)
        .with_env_filter(filter)
        .finish();

    set_global_default(subscriber)?;

    let gemini_api = env::var("GEMINI_API").expect("GEMINI_API environment variable not set");
    let username = env::var("USERNAME").expect("USERNAME environment variable not set");
    let password = env::var("PASSWORD").expect("PASSWORD environment variable not set");
    let server = env::var("SERVER").expect("SERVER environment variable not set");

    info!("Initializing gemini client");

    let gemini = Arc::new(
        GeminiBuilder::new(&gemini_api)
            .with_model(Model::Gemini25FlashLite)
            .build()?,
    );

    let user_id_str = format!("@{}:{}", username, server);
    let user = UserId::parse(&user_id_str)?;

    info!("Logging as {}", user);

    let client = Client::builder()
        .server_name(user.server_name())
        .build()
        .await?;

    client
        .matrix_auth()
        .login_username(&username, &password)
        .send()
        .await?;

    client.add_event_handler(move |ev: OriginalSyncRoomMessageEvent, room: Room| {
        call(ev, room, gemini)
    });

    client.add_event_handler(auto_join);

    client.sync(SyncSettings::default()).await?;

    Ok(())
}
