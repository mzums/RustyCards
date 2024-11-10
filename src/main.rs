use clap::{Arg, ArgMatches, Command};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize, Debug)]
struct Question {
    question: String,
    options: Vec<String>,
    answer: String,
}

fn get_matches() -> ArgMatches {
    Command::new("RustyCards")
        .version("1.0")
        .author("mzums mzuums@gmail.com")
        .about("A flashcard/quiz terminal app")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .value_parser(clap::value_parser!(String))
                .default_value("data/questions.json"),
        )
        .arg(
            Arg::new("shuffle")
                .short('s')
                .long("shuffle")
                .help("random order")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches()
}

fn get_questions(matches: &ArgMatches) -> Vec<Question> {
    let file_path = matches.get_one::<String>("file").unwrap();
    let mut file = File::open(file_path).expect("Failed to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("An error while reading the file");
    serde_json::from_str(&contents).expect("Failed to parse JSON")
}

fn main() {
    let matches = get_matches();

    let mut questions = get_questions(&matches);
    
    if matches.get_flag("shuffle") {
        let mut rng = thread_rng();
        questions.shuffle(&mut rng);
    }

    for (i, question) in questions.iter().enumerate() {
        println!("Question {}: {}", i + 1, question.question);
        for (j, option) in question.options.iter().enumerate() {
            println!(" {}) {}", j+1, option);
        }
        println!();
    }
}
