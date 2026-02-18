use anyhow::Result;
use gemini_rust::Gemini;
use matrix_sdk::ruma::events::SyncMessageLikeEvent;
use matrix_sdk::ruma::events::room::message::{
    MessageType, OriginalSyncRoomMessageEvent, RoomMessageEventContent,
};
use matrix_sdk::{Room, RoomState};
use ruma::UserId;
use ruma::events::AnySyncMessageLikeEvent;
use ruma::events::AnySyncTimelineEvent;
use ruma::events::room::message::Relation;
use std::sync::Arc;

pub async fn call(
    event: OriginalSyncRoomMessageEvent,
    room: Room,
    gemini: Arc<Gemini>,
) -> Result<()> {
    if room.state() != RoomState::Joined {
        return Ok(());
    }

    if event.content.mentions.is_none() {
        return Ok(());
    }

    let user = UserId::parse("@grok:unifwe.com")?;

    let MessageType::Text(text_content) = event.content.msgtype else {
        return Ok(());
    };

    if !event.content.mentions.unwrap().user_ids.contains(&user) {
        return Ok(());
    }

    let prompt = text_content.body.clone();
    let client_ref: &Gemini = Arc::as_ref(&gemini);
    let system_instruction = "You are Grok-style assistant on Matrix. \
 briefly, factually, and confidently. \
If a claim is wrong, correct it politely.";
    let mut final_prompt = format!("System:\n{}\n\nUser:\n{}", system_instruction, prompt);

    if let Some(Relation::Reply { in_reply_to }) = &event.content.relates_to {
        let replied_event_id = &in_reply_to.event_id;

        let timeline_event = room.event(replied_event_id, None).await?;
        let raw = timeline_event.into_raw();
        let any_event: AnySyncTimelineEvent = raw.deserialize()?;

        match any_event {
            AnySyncTimelineEvent::MessageLike(AnySyncMessageLikeEvent::RoomMessage(
                SyncMessageLikeEvent::Original(ev),
            )) => {
                if let MessageType::Text(original_text) = ev.content.msgtype {
                    let original_body = original_text.body;
                    final_prompt = format!(
                        "System:\n{}\n\nUser:\n{}\n\nReply:\n{}",
                        system_instruction, prompt, original_body
                    );
                }
            }
            _ => {}
        }
    }

    let response = client_ref
        .generate_content()
        .with_user_message(final_prompt)
        .execute()
        .await?;

    let reply_text = response.text();

    let content = RoomMessageEventContent::text_plain(reply_text);

    room.send(content).await?;

    Ok(())
}
