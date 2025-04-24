use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Card {
    id: u32,
    question: Question,
    interval: u32,
    repetitions: u32,
    ease_factor: f32,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
enum Question {
    Closed(String, bool),
    Open(String, String),
}
