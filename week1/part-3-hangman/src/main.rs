// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    // println!("random word: {}", secret_word);

    // Your code here! :)
    let mut word_so_far: Vec<char> = Vec::new();
    let mut not_guessed_chars: Vec<char> = Vec::new();
    let mut guessed_chars: Vec<char> = Vec::new();
    let mut remained_guesses = NUM_INCORRECT_GUESSES;
    for i in 0..secret_word_chars.len() {
        word_so_far.push('-');
        not_guessed_chars.push(secret_word_chars[i]);
    }
    loop {
        print!("The word so far is ");
        for c in word_so_far.iter() {
            print!("{}", *c);
        }
        println!("");
        print!("You have guessed the following letters: ");
        for c in guessed_chars.iter() {
            print!("{}", *c);
        }
        println!("");
        println!("You have {} guesses left", remained_guesses);
        print!("Please guess a letter: ");
        // Make sure the prompt from the previous line gets displayed:
        io::stdout().flush().expect("Error flushing stdout.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line.");
        let guess_char: char = guess.chars().collect::<Vec<char>>()[0];
        guessed_chars.push(guess_char);
        if not_guessed_chars.contains(&guess_char) {
            not_guessed_chars.retain(|&x| x != guess_char);
            for (i, x) in secret_word_chars.iter().enumerate() {
                if guess_char == *x {
                    word_so_far[i] = guess_char;
                }
            }
        } else {
            remained_guesses -= 1;
            println!("Sorry, that letter is not in the word");
        }
        println!("");
        if remained_guesses == 0 {
            println!("Sorry, you ran out of guesses!");
            break;
        }
        if not_guessed_chars.is_empty() {
            println!(
                "Congratulations you guessed the secret word: {}!",
                secret_word
            );
            break;
        }
    }
}
