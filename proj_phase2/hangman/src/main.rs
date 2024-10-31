//words.txt file from https://github.com/Xethron/Hangman/blob/master/words.txt
use std::fs;
extern crate rand;
extern crate console;
use rand::{thread_rng, Rng};
use std::io;
use console::Term;
//Guess class which has 2 fields
//Var1: guess a char
//Var2: bool true if hits; false if misses
struct Game {
    secret: String,
    game_on : bool,
    guessed_letters: Vec<char>,
    stage: i32
}

impl Game {
    pub fn new(secret: String, game_on: bool, guessed_letters: Vec<char>, stage: i32) -> Self {
        Self {
            secret,
            game_on,
            guessed_letters,
            stage
        }
    }

    pub fn stage(&self) -> (){
        let stage = format!("stage{}.txt", self.stage);
        let file = fs::read_to_string(stage).expect("File Read");
        println!("{file}\n");
    }

    pub fn get_game_status(&self) -> bool{
        return self.game_on;
    }

    pub fn end_game(&mut self) -> (){
        self.game_on = false;
    }

    pub fn add_to_guesses(&mut self, guess: char) -> (){
        self.guessed_letters.push(guess);
    }

    pub fn print_guesses(&self) -> (){
        println!("{:?}", self.guessed_letters);
    }

    pub fn print_discovered_word(&self) -> (){
        let mut word = Vec::<char>::new();
        for i in 0..self.secret.len() {
            if self.guessed_letters.contains(&(self.secret.as_bytes()[i] as char)) {
                word.push(self.secret.as_bytes()[i] as char);
            } 

        }
        println!("{:#?}", word);
    }
}


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

// https://www.reddit.com/r/rust/comments/17mseuc/how_would_i_read_a_single_char_at_a_time_from/?rdt=65387
// fn read_one_char()  -> char {
//         println!("Choose a letter to guess");
//         let mut input_handler = io::stdin();
//         let mut byte = [0_u8];
//         return input_handler.read_exact(&mut byte).unwrap();
//     }


fn main() {
    let mut rng = rand::thread_rng();
    //Dynamic Array of Words
    let words = build_word_array();

    println!("Welcome to Hangman!");
    let secret: String = pick_word(&words).to_lowercase();
    //println!("{secret}");

    let mut hangman = Game::new(secret,true, Vec::<char>::new(), 0);
    let term = Term::stdout();

    while hangman.get_game_status(){
        print!("Guesses: ");
        hangman.print_guesses();

        println!("Secret Word: ");
        hangman.print_discovered_word();

        hangman.stage();

        let guess: char = term.read_char().expect("Unable to Read");
        println!("{guess}");
        
        hangman.add_to_guesses(guess);



        // hangman.end_game();
    }     
}
