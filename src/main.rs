use rand::seq::SliceRandom;
use std::io::{self, Write};

fn main() {
    let words = vec!["apple", "banana", "cherry", "date", "elderberry"];
    let mut rng = rand::thread_rng();
    let chosen_word = words.choose(&mut rng).unwrap();
    let mut guessed_letters: Vec<char> = vec!['-'; chosen_word.len()];
    let mut all_guesses: Vec<char> = Vec::new();
    let mut attempts = 6;


    println!("Welcome to Hangman!");

    loop {
        println!("Word: {}", guessed_letters.iter().collect::<String>());
        println!("Attempts Left: {}", attempts);
        println!("Guessed letters: {}", all_guesses.iter().collect::<String>());

        println!("Enter a letter: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();
        let guess = guess.trim().chars().next().unwrap();

        let mut found = false;
        for (i, letter) in chosen_word.chars().enumerate() {
            all_guesses.push(letter);
            if letter == guess {
                guessed_letters[i] = letter;
                found = true;
            }
        }

        if !found {
            attempts -= 1;
        }

        if attempts == 0 {
            println!("Game over! The word was: {}", chosen_word);
            break;
        }

        if guessed_letters.iter().all(|&c| c != '-') {
            println!("Congratulations! You guessed the word: {}", chosen_word);
            break;
        }
    }
}
