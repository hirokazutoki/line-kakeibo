use shared::{InputQueueMessage, ResponseQueueMessage, EventMessage};

fn main(webhook_event_id: String, message: EventMessage) {

    // 送信処理
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
}
