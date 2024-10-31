//words.txt file from https://github.com/Xethron/Hangman/blob/master/words.txt
use std::fs;
extern crate rand;
extern crate console;
use rand::{thread_rng, Rng};
use console::Term;

struct Game {
    secret: String,
    discovered_word: Vec<char>,
    game_on : bool,
    guessed_letters: Vec<char>,
    stage: i32
}

impl Game {
    pub fn new(secret: String, discovered_word: Vec<char>, game_on: bool, guessed_letters: Vec<char>, stage: i32) -> Self {
        Self {
            secret,
            discovered_word,
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

    pub fn print_discovered_word(&mut self) -> (){
        for i in 0..self.secret.len() {
            if self.guessed_letters.contains(&(self.secret.as_bytes()[i] as char)) && !self.discovered_word.contains(&(self.secret.as_bytes()[i] as char)) {
                self.discovered_word.push(self.secret.as_bytes()[i] as char);
            } 

        }
        println!("{:#?}", self.discovered_word);
    }

    pub fn get_stage(&self) -> i32 {
        return self.stage;
    }

    pub fn check_hit(&mut self, guess: char) -> () {
        if self.secret.contains(&guess.to_string()) {
            println!("The secret word contains: {}", &guess.to_string());
        }else{
            self.stage += 1;
        }
    }

    pub fn won(&self) -> bool {
        if self.secret.len() == self.discovered_word.len(){
            return true;
        }else{
            return false;
        }
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

fn main() {
    let _rng = rand::thread_rng();
    //Dynamic Array of Words
    let words = build_word_array();

    println!("Welcome to Hangman!");
    let secret: String = pick_word(&words).to_lowercase();
    //println!("{secret}");

    let mut hangman = Game::new(secret.clone(), Vec::<char>::new(), true, Vec::<char>::new(), 0);
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

        hangman.check_hit(guess);

        if hangman.get_stage() == 7 {
            println!("You died!\nThe secret word was {secret}");
            hangman.end_game();
        }else if hangman.won() {
            println!("Congrats, you won!");
            hangman.end_game();
        }
    }  

    println!("Would you like to play again? (y/n)");
    let again: char = term.read_char().expect("Unable to Read");
    if again == 'y' {
        main();
    }else{
        println!("Thanks for playing!");
    }
}
