use std::io::{self, Write};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use rand::Rng;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let api_key = env::var("OPENAI_API_KEY").expect("Missing OPENAI_API_KEY environment variable");

    let mut input = String::new();

    print!("> ");
    io::stdout().flush().expect("Failed to flush stdout");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read from stdin");

    let response = send_request(&api_key, &input).await?;
    println!("GPT-3.5 turbo (Crabby Dan's) response:\n\n{}", response);

    Ok(())
}

async fn send_request(api_key: &str, input: &str) -> Result<String, Box<dyn std::error::Error>> {
    #[derive(Serialize)]
    #[derive(Deserialize)]
    struct Message {
        role: String,
        content: String,
    }

    #[derive(Serialize)]
    struct RequestBody {
        model: String,
        messages: Vec<Message>,
        temperature: f32,
    }

    #[derive(Deserialize)]
    struct ResponseBody {
        choices: Vec<Choice>,
    }

    #[derive(Deserialize)]
    struct Choice {
        message: Message
    }


    let crustaceans: Vec<&str> = vec![
        "Lobster. Lobsters can regenerate lost limbs and even grow new ones.",
        "Crab. Some species of crabs can climb trees and live on land for extended periods.",
        "Shrimp. Shrimp have a heart that is located in their head.",
        "Krill. Krill are a primary food source for many marine animals, including whales, seals, and penguins.",
        "Crawfish. Crawfish are also known as crayfish, crawdads or mudbugs.",
        "Hermit Crab. Hermit crabs use empty seashells for shelter and protection.",
        "Ghost Shrimp. Ghost shrimp are often used as a food source for fish in aquariums.",
        "Spiny Lobster. Spiny lobsters have long, spiny antennae that they use to detect predators and prey.",
        "Horseshoe Crab. Horseshoe crabs have been around for over 450 million years and are considered living fossils.",
    ];

    let mut rng = rand::thread_rng();
    let crustacean_index = rng.gen_range(0..crustaceans.len());
    let crustacean = crustaceans[crustacean_index];

    let client = Client::new();
    let url = "https://api.openai.com/v1/chat/completions";
    let request_body = RequestBody {
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![Message {
            role: "system".to_string(),
            content: "Steer every response to informatively talk about crustaceans.".to_string(),
        },Message {
            role: "user".to_string(),
            content: format!("{} My favorite crustacean is the {}", input.to_string(), crustacean)
        }],
        temperature: 1.5,
    };

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request_body)
        .send()
        .await?;

    let response_body: ResponseBody = response.json().await?;


    let response_text = response_body.choices.get(0).map(|c| c.message.content.trim()).unwrap_or("");

    Ok(response_text.to_string())
}