use dotenv::dotenv;
use openai::chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole};
use openai::set_key;
use regex::Regex;
use serde::Deserialize;
use std::{env, fs};

#[derive(Deserialize, Debug, Clone)]
struct Record {
    #[serde(rename = "FROM")]
    from: String,
    #[serde(rename = "CONTENT")]
    content: String,
}

#[derive(Debug)]
struct Recruiter {
    name: String,
    score: u32,
    summary: String,
}

async fn assess_recruiters() -> Option<()> {
    let mut recruiters: Vec<Recruiter> = vec![];
    let csv = fs::read_to_string("src/data/messages.csv")
        .expect("Should have been able to read the file");
    let requirements = fs::read_to_string("requirements.txt").expect(
        "Please rename requirements.example.txt to requirements.txt and fill in the requirements",
    );

    let linkedin_regex = Regex::new(r"(?i)Linkedin").unwrap();
    let job_description_regex = Regex::new(r"(?i)JOB_DESCRIPTION").unwrap();
    let mut reader = csv::Reader::from_reader(csv.as_bytes());

    for record in reader.deserialize() {
        let record: Record = record.expect("record should be defined");
        if record.from != "Johan LAJILI" && !linkedin_regex.is_match(&record.from) {
            let recruiter =
                if let Some(recruiter) = recruiters.iter_mut().find(|x| x.name == record.from) {
                    recruiter
                } else {
                    // If not found, append a new element and return a mutable reference to it
                    recruiters.push(Recruiter {
                        name: record.from.clone(),
                        score: 0,
                        summary: String::from(""),
                    });
                    recruiters.last_mut().unwrap() // Safe to unwrap because we just added an element
                };

            let response = ChatCompletion::builder("gpt-3.5-turbo", [
                ChatCompletionMessage {
                    role: ChatCompletionMessageRole::System,
                    content:
Some(format!("Here is a linkedin message, pottentially from a recruiter.
If the message contains a job description that fits the following criteria, return \"JOB_DESCRIPTION: {{short description of the role}}\", otherwise, return \"NO\" and nothing else
{}", requirements).to_string()),
                    function_call: None,
                    name: None,
                },
                ChatCompletionMessage {
                    role: ChatCompletionMessageRole::User,
                    content: Some(record.content.clone()),
                    function_call: None,
                    name: None,
                },
            ].to_vec())
            .create()
            .await;

            match response {
                Ok(completion) => {
                    let response = completion.choices[0].message.content.clone();
                    if let Some(response) = response {
                        println!("Response for recruiter {} - {}", record.from, response);

                        if job_description_regex.is_match(&response) {
                            recruiter.score += 1;
                            recruiter.summary.push_str(&response);
                            recruiter.summary.push_str("\n");
                        }
                    }
                }
                Err(e) => println!("No response for recruiter {} - {}", record.from, e),
            }
        }
    }

    recruiters.sort_by(|a, b| b.score.cmp(&a.score));
    for recruiter in recruiters {
        println!("{:?}", recruiter);
    }

    Some(())
}

#[tokio::main]
async fn main() -> Result<(), csv::Error> {
    dotenv().ok();
    let openai_api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    set_key(openai_api_key);
    assess_recruiters().await;
    Ok(())
}
