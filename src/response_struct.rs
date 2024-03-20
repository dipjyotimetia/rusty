use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TriviaResponse {
    #[serde(rename = "response_code")]
    pub response_code: i64,

    #[serde(rename = "results")]
    pub results: Vec<ResultAnswser>,
}

#[derive(Serialize, Deserialize)]
pub struct ResultAnswser {
    #[serde(rename = "type")]
    result_type: Type,

    #[serde(rename = "difficulty")]
    difficulty: Difficulty,

    #[serde(rename = "category")]
    category: Category,

    #[serde(rename = "question")]
    pub question: String,

    #[serde(rename = "correct_answer")]
    pub correct_answer: String,

    #[serde(rename = "incorrect_answers")]
    incorrect_answers: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub enum Category {
    #[serde(rename = "Animals")]
    Animals,
}

#[derive(Serialize, Deserialize)]
pub enum Difficulty {
    #[serde(rename = "easy")]
    Easy,

    #[serde(rename = "hard")]
    Hard,

    #[serde(rename = "medium")]
    Medium,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "multiple")]
    Multiple,
}