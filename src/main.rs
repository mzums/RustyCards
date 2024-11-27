use clap::{Arg, ArgMatches, Command};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::Deserialize;
use std::fs::File;
use std::io::{self, Read};
use colored::Colorize;

#[derive(Deserialize, Debug, Clone)]
struct Question {
    question: String,
    options: Vec<String>,
    answers: Vec<i32>,
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
                .default_value("data/questions.json")
                .help("The file containing questions in JSON format"),
        )
        .arg(
            Arg::new("shuffle")
                .short('s')
                .long("shuffle")
                .help("Randomize the order of questions")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches()
}

fn get_questions(matches: &ArgMatches) -> Result<Vec<Question>, Box<dyn std::error::Error>> {
    let file_path = matches.get_one::<String>("file").unwrap();
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let questions: Vec<Question> = serde_json::from_str(&contents)?;
    Ok(questions)
}

fn display_question(question: &Question, idx: usize) {
    println!("Question {}: {}", idx + 1, question.question);
    for (j, option) in question.options.iter().enumerate() {
        println!(" {}) {}", j + 1, option);
    }
}

fn handle_input(answers: &[i32], score: i32) -> i32 {
    let mut guess = String::new();
    println!("Enter your answer (separate with spaces if there are many possible)");
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input");

    let guesses: Vec<i32> = guess
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    if guesses.is_empty() {
        println!("Invalid input. Please enter numbers separated by spaces.");
        return score;
    }

    if guesses.len() == answers.len() && guesses.iter().all(|g| answers.contains(g)) {
        println!("Correct!\n");
        score + 1
    } else {
        println!("The correct answers were: {:?}\n", answers);
        score
    }
}

fn main() {
    let title = "RustyCards!\n".bold().yellow();
    println!("{title}");

    let mut score = 0;
    let matches = get_matches();

    let questions = match get_questions(&matches) {
        Ok(questions) => questions,
        Err(err) => {
            eprintln!("Error loading questions: {}", err);
            return;
        }
    };
    
    let mut rng = thread_rng();

    let mut shuffled_questions = questions.clone();
    if matches.get_flag("shuffle") {
        shuffled_questions.shuffle(&mut rng);
    }

    for (i, question) in shuffled_questions.iter_mut().enumerate() {
        display_question(&question, i);
        score = handle_input(&question.answers, score);
    }
    println!("Your score was {score}/{}", questions.len());
}
