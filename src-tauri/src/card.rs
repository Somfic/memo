use std::collections::HashMap;

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Serialize, Deserialize, Type)]
pub struct Card {
    pub prompt: Prompt,
    pub answer: Answer,
    pub next_review: DateTime<Utc>,
    pub repititions: Repititions,
    pub ease_factor: f32,
}

#[derive(Serialize, Deserialize, Type)]
#[serde(tag = "type", content = "value")]
pub enum Prompt {
    Text(String),
}

#[derive(Serialize, Deserialize, Type)]
#[serde(tag = "type", content = "value")]
pub enum Answer {
    Text(String),
}

#[derive(Serialize, Deserialize, Type)]
pub struct Repititions {
    pub successful: u32,
    pub total: u32,
}

impl Repititions {
    fn mark_failed(&mut self) {
        self.successful = 0;
        self.total += 1;
    }

    fn mark_successful(&mut self) {
        self.successful += 1;
        self.total += 1;
    }
}

#[derive(Serialize, Deserialize, Type, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnswerQuality {
    OkEasy = 5,
    OkHesitated = 4,
    OkDifficult = 3,
    NotOkRemembered = 2,
    NotOkFamiliar = 1,
    NotOkForgot = 0,
}

impl Card {
    fn review(&mut self, quality: AnswerQuality) {
        self.next_review = self.next_review_for_quality(quality);

        let quality = quality as u8 as f32;
        self.ease_factor += 0.1 - (5.0 - quality) * (0.08 + (5.0 - quality) * 0.02);
        if self.ease_factor < 1.3 {
            self.ease_factor = 1.3;
        }
    }

    fn next_review_for_quality(&self, quality: AnswerQuality) -> DateTime<Utc> {
        let quality = quality as u8;

        if quality < 3 {
            Utc::now() + Duration::days(1)
        } else {
            // user was right
            let next_review_in_days = match self.repititions.successful {
                1 => 1,
                2 => 6,
                _ => {
                    let days = self.next_review.timestamp() - Utc::now().timestamp();
                    let days = (days / 86400) as f32;
                    let next_review_in_days = (days * self.ease_factor).round() as i64;
                    if next_review_in_days < 1 {
                        1
                    } else {
                        next_review_in_days
                    }
                }
            };

            Utc::now() + Duration::days(next_review_in_days)
        }
    }
}

#[tauri::command]
#[specta::specta]
pub fn preview_next_review(card: Card) -> HashMap<AnswerQuality, DateTime<Utc>> {
    let mut result = HashMap::new();
    for quality in [
        AnswerQuality::OkEasy,
        AnswerQuality::OkHesitated,
        AnswerQuality::OkDifficult,
        AnswerQuality::NotOkRemembered,
        AnswerQuality::NotOkFamiliar,
        AnswerQuality::NotOkForgot,
    ] {
        result.insert(quality, card.next_review_for_quality(quality));
    }
    result
}

#[derive(Serialize, Deserialize, Type)]
pub struct PreviewNextReviewResponse {
    pub ok_easy: DateTime<Utc>,
    pub ok_hesitated: DateTime<Utc>,
    pub ok_difficult: DateTime<Utc>,
    pub not_ok_remembered: DateTime<Utc>,
    pub not_ok_familiar: DateTime<Utc>,
    pub not_ok_forgot: DateTime<Utc>,
}
