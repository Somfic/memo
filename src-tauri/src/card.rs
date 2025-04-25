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
    successful: u32,
    total: u32,
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

enum AnswerQuality {
    Perfect = 5,
    Hesitant = 4,
    Difficult = 3,
    Remembered = 2,
    Familiar = 1,
    Forgotten = 0,
}

impl Card {
    fn review(&mut self, quality: AnswerQuality) {
        let quality = quality as u8;
        let today = Utc::now().timestamp();

        if quality < 3 {
            // user was wrong
            self.repititions.mark_failed();
            self.next_review = Utc::now() + Duration::days(1);
        } else {
            // user was right
            self.repititions.mark_successful();

            let next_review_in_days = match self.repititions.successful {
                1 => 1,
                2 => 6,
                _ => {
                    let days = self.next_review.timestamp() - today;
                    let days = (days / 86400) as f32;
                    let next_review_in_days = (days * self.ease_factor).round() as i64;
                    if next_review_in_days < 1 {
                        1
                    } else {
                        next_review_in_days
                    }
                }
            };

            self.next_review = Utc::now() + Duration::days(next_review_in_days);
        }

        // adjust ease factor
        let quality = quality as f32;
        self.ease_factor += 0.1 - (5.0 - quality) * (0.08 + (5.0 - quality) * 0.02);
        if self.ease_factor < 1.3 {
            self.ease_factor = 1.3;
        }
    }
}
