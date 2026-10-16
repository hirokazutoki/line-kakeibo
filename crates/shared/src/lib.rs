use serde::{Deserialize, Serialize};

/// Input Queueに投入されるメッセージ
/// - 画像メッセージ専用
#[derive(Debug, Serialize, Deserialize)]
pub struct InputQueueMessage {
    pub event_id: String,
    pub user_id: String,
    pub reply_token: String,
    pub message_id: String,
}

/// 画像解析結果（仮のモック定義）
#[derive(Debug, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub text: String,
}

/// Response Queueに投入されるメッセージ
/// - Text : テキストメッセージ
/// - AnalysisResult : 画像解析の結果
#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseQueueMessage {
    Text {
        event_id: String,
        user_id: String,
        reply_token: String,
        text: String,
    },
    AnalysisResult {
        event_id: String,
        user_id: String,
        reply_token: String,
        result: AnalysisResult,
    },
}

pub enum EventMessage {
    Text {
        text: String,
    },
    Image {
        id: String,
    }
}