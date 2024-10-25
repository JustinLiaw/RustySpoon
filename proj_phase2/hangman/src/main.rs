//words.txt file from https://github.com/Xethron/Hangman/blob/master/words.txt
use std::fs;
extern crate rand;
use rand::{thread_rng, Rng};


fn pick_word(words: &[String]) -> &str{
    let index = thread_rng().gen_range(0..=words.len());
    return &words[index];
}

fn build_word_array() -> Vec<String>{
    let file = fs::read_to_string("words.txt")
    .expect("File Read");
    let mut words = Vec::new();

    for line in file.lines() {
        words.push(line.to_string());
    }

    return words;
}

fn build_hangman(){

}

fn letter_in_word(){

}

// let mut rng = rand::thread_rng();
// //Dynamic Array of Words
// let words = build_word_array();

// println!("Welcome to Hangman!");
// let secret: &str = pick_word(&words);
// println!("{secret}");

fn main() {

    
}
