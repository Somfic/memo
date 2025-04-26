use std::collections::HashMap;

use chrono::{Date, DateTime, Duration, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Serialize, Deserialize, Type)]
pub struct Card {
    pub prompt: Prompt,
    pub answer: Answer,
    pub next_review: NaiveDate,
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
        self.ease_factor = f32::max(
            1.3,
            self.ease_factor + 0.1 - (5.0 - quality) * (0.08 + (5.0 - quality) * 0.02),
        );
    }

    fn next_review_for_quality(&self, quality: AnswerQuality) -> NaiveDate {
        let next_review_in_days = if matches!(
            quality,
            AnswerQuality::NotOkForgot
                | AnswerQuality::NotOkFamiliar
                | AnswerQuality::NotOkRemembered
        ) {
            1
        } else {
            match self.repititions.total + 1 {
                1 => 1,
                2 => 6,
                _ => {
                    let days = self
                        .next_review
                        .signed_duration_since(Utc::now().date_naive())
                        .num_days();

                    println!("ease_factor: {}", self.ease_factor);
                    println!("days: {}", days);

                    i64::max(1, (days as f32 * self.ease_factor).round() as i64)
                }
            }
        };

        (Utc::now() + Duration::days(next_review_in_days)).date_naive()
    }
}

#[tauri::command]
#[specta::specta]
pub fn preview_next_review(card: Card) -> HashMap<AnswerQuality, NaiveDate> {
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
