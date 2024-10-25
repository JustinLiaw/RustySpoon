//words.txt file from https://github.com/Xethron/Hangman/blob/master/words.txt

use std::io;
//Guess class which has 2 fields
//Var1: guess a char
//Var2: bool true if hits; false if misses
struct Guess {
    guess : char,
    hit : bool
}


//Picks the secret words from a file
//args: None
fn pick_word(){

}

//Print out the hangman based on the number of guesses
//args guessList
fn build_hangman(){

}

//TODO: Build the text below the character
//args: guessList; takes in the secret;
//return none
fn build_text() {

}

//TODO:
// Takes in the secret; Takes in character
// return bool True if inside 
fn letter_in_word(input : , left : Vec<char>, Vec<Guess>guesses ){
}

// https://www.reddit.com/r/rust/comments/17mseuc/how_would_i_read_a_single_char_at_a_time_from/?rdt=65387
fn read_one_char()  -> char {
        println!("Choose a letter to guess");
        let mut input_handler = io::stdin();
        let mut byte = [0_u8];
        return input_handler.read_exact(&mut byte).unwrap();

    }

fn main() {
    println!("Welcome to Hangman!");
    // String secret = pick_word()
    let mut secret = "DaveHasBigJuicyLips".to_string();
    secret =  secret.to_lowercase();
    //make a vector of guessses
    let mut guesses: Vec<Guess> = Vec::new();
    let mut left : Vec<char> = secret.chars().collect();
    //main game loop
    while left.len() != 0    {
        let input = read_one_char()
        println!("{}", input);
        letter_in_word(input,left,&guesses);
    }
    //TODO: 
    //Ask user for input
    //if(letter_in_word == true)
    //fill out guess list
    //call build_hangman()
    //call build_text()
          
}
