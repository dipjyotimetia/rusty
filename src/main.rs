
mod response_struct;
use crate::response_struct:: TriviaResponse;
use reqwest::Client;

async fn play_game() ->  Result<(), Box<dyn std::error::Error>>  { 
    let client = Client::new();
    let res = client
        .get("https://opentdb.com/api.php?amount=10&category=27&type=multiple")
        .send()
        .await?; 

    let trivia_response: TriviaResponse = res.json().await?;

    match trivia_response.response_code {
        0 => println!("Success"),
        1 => println!("No results"),
        2 => println!("Invalid parameter"),
        3 => println!("Token not found"),
        4 => println!("Token empty"),
        _ => println!("Unknown error"),
    }

    for result in trivia_response.results {
        println!("{:?}", result.question);
        println!("{:?}", result.correct_answer);
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(err) = play_game().await {
        println!("Error during the game: {}", err);
    }
}

