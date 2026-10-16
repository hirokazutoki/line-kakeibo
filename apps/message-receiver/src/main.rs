use shared::{InputQueueMessage, ResponseQueueMessage, EventMessage};

fn main() {
    // 仮の呼び出し例
    let webhook_event_id = String::new();
    let user_id = String::new();
    let reply_token = String::new();
    let message = EventMessage::Text { text: String::new() };

    if let Err(e) = handle_event(webhook_event_id, user_id, reply_token, message) {
        eprintln!("handle_event failed: {e}");
    }
}

/// 送信処理
fn handle_event(
    webhook_event_id: String,
    user_id: String,
    reply_token: String,
    message: EventMessage,
) -> Result<(), Box<dyn std::error::Error>> {
    match message {
        EventMessage::Text { text, .. } => {
            // テキストはResponse Queueへ送信する
            let msg = ResponseQueueMessage::Text {
                event_id: webhook_event_id,
                user_id,
                reply_token,
                text,
            };
            let body = serde_json::to_string(&msg)?;

            // ... Response Queueへbodyを送信する処理
        }
        EventMessage::Image { id } => {
            // 画像はInput Queueへ送信する
            let msg = InputQueueMessage {
                event_id: webhook_event_id,
                user_id,
                reply_token,
                message_id: id,
            };
            let body = serde_json::to_string(&msg)?;

            // Input Queueへbodyを送信する処理
        }
    }

    Ok(())
}
