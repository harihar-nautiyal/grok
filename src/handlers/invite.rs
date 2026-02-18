use matrix_sdk::ruma::events::room::member::StrippedRoomMemberEvent;
use matrix_sdk::{Client, Room};
use tokio::time::{Duration, sleep};
use tracing::{error, info};

pub async fn auto_join(room_member: StrippedRoomMemberEvent, client: Client, room: Room) {
    if room_member.state_key != client.user_id().unwrap() {
        return;
    }

    tokio::spawn(async move {
        info!("Autojoining room {}", room.room_id());
        let mut delay = 2;

        while let Err(err) = room.join().await {
            error!(
                "Failed to join room {} ({err:?}), retrying in {delay}s",
                room.room_id()
            );

            sleep(Duration::from_secs(delay)).await;
            delay *= 2;

            if delay > 3600 {
                error!("Can't join room {} ({err:?})", room.room_id());
                break;
            }
        }
        info!("Successfully joined room {}", room.room_id());
    });
}
