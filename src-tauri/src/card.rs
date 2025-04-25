use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Serialize, Deserialize, Type)]
pub struct Card {
    pub id: u32,
    pub question: Question,
    pub next_review: DateTime<Utc>,
    pub repititions: Repititions,
    pub ease_factor: f32,
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

#[derive(Serialize, Deserialize, Type)]
#[serde(tag = "type", content = "value")]
pub enum Question {
    Closed(String, bool),
    Open(String, String),
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};

    #[test]
    fn test_mark_failed() {
        let mut reps = Repititions {
            successful: 3,
            total: 5,
        };
        reps.mark_failed();
        assert_eq!(reps.successful, 0);
        assert_eq!(reps.total, 6);
    }

    #[test]
    fn test_mark_successful() {
        let mut reps = Repititions {
            successful: 3,
            total: 5,
        };
        reps.mark_successful();
        assert_eq!(reps.successful, 4);
        assert_eq!(reps.total, 6);
    }

    #[test]
    fn test_review_wrong_answer() {
        let mut card = Card {
            id: 1,
            question: Question::Closed("Is Rust awesome?".to_string(), true),
            next_review: Utc::now(),
            repititions: Repititions {
                successful: 2,
                total: 3,
            },
            ease_factor: 2.5,
        };

        card.review(AnswerQuality::Forgotten);

        assert_eq!(card.repititions.successful, 0);
        assert_eq!(card.repititions.total, 4);
        assert!(card.next_review > Utc::now());
    }

    #[test]
    fn test_review_correct_answer() {
        let mut card = Card {
            id: 1,
            question: Question::Closed("Is Rust awesome?".to_string(), true),
            next_review: Utc::now(),
            repititions: Repititions {
                successful: 2,
                total: 3,
            },
            ease_factor: 2.5,
        };

        card.review(AnswerQuality::Perfect);

        assert_eq!(card.repititions.successful, 3);
        assert_eq!(card.repititions.total, 4);
        assert!(card.next_review > Utc::now());
        assert!(card.ease_factor >= 1.3);
    }

    #[test]
    fn test_review_hesitant_answer() {
        let mut card = Card {
            id: 1,
            question: Question::Closed("What is the capital of France?".to_string(), true),
            next_review: Utc::now(),
            repititions: Repititions {
                successful: 1,
                total: 2,
            },
            ease_factor: 2.0,
        };

        card.review(AnswerQuality::Hesitant);

        assert_eq!(card.repititions.successful, 2);
        assert_eq!(card.repititions.total, 3);
        assert!(card.next_review > Utc::now());
        assert!(card.ease_factor >= 1.3);
    }

    #[test]
    fn test_review_difficult_answer() {
        let mut card = Card {
            id: 2,
            question: Question::Open(
                "Explain the theory of relativity.".to_string(),
                "A brief explanation.".to_string(),
            ),
            next_review: Utc::now(),
            repititions: Repititions {
                successful: 3,
                total: 5,
            },
            ease_factor: 2.5,
        };

        card.review(AnswerQuality::Difficult);

        assert_eq!(card.repititions.successful, 4);
        assert_eq!(card.repititions.total, 6);
        assert!(card.next_review > Utc::now());
        assert!(card.ease_factor >= 1.3);
    }

    #[test]
    fn test_review_forgotten_answer() {
        let mut card = Card {
            id: 3,
            question: Question::Closed("What is 2 + 2?".to_string(), true),
            next_review: Utc::now(),
            repititions: Repititions {
                successful: 5,
                total: 10,
            },
            ease_factor: 1.5,
        };

        card.review(AnswerQuality::Forgotten);

        assert_eq!(card.repititions.successful, 0);
        assert_eq!(card.repititions.total, 11);
        assert!(card.next_review > Utc::now());
        assert_eq!(card.ease_factor, 1.3);
    }

    #[test]
    fn test_review_perfect_answer_with_high_repititions() {
        let mut card = Card {
            id: 4,
            question: Question::Open(
                "Define quantum mechanics.".to_string(),
                "A branch of physics.".to_string(),
            ),
            next_review: Utc::now(),
            repititions: Repititions {
                successful: 10,
                total: 15,
            },
            ease_factor: 2.8,
        };

        card.review(AnswerQuality::Perfect);

        assert_eq!(card.repititions.successful, 11);
        assert_eq!(card.repititions.total, 16);
        assert!(card.next_review > Utc::now());
        assert!(card.ease_factor > 2.8);
    }
}
