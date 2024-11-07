//words.txt file from https://github.com/Xethron/Hangman/blob/master/words.txt
use std::*;
extern crate rand;
extern crate console;
use rand::{thread_rng, Rng};
use console::Term;


//Struct that contains the fields of the Game implementation
//secret is the desired words
//discovered_word contains the correct guesses of the word and has '_' for placeholders
//game_on determines if there the game is ready to stop or not
//guessed_letters contains all the guesses the user has made
//stage contains which file number stage to print next 

struct Game {
    secret: String,
    discovered_word: Vec<char>,
    game_on : bool,
    guessed_letters: Vec <char>,
    stage: i32
}


//impl for the Game
    //Constructs the a Game instance
    //Contains methods: stage, get_get_status, end_game, add_to_guess, print_guesses, print_discovered_word, get_stage
    //check_hit, and won  

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
    //Reads in a stage file and prints it to the screen
    pub fn stage(&self) -> (){
        let stage = format!("hangman/src/stage{}.txt", self.stage);
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
        if !self.guessed_letters.contains(&guess) {
            self.guessed_letters.push(guess);
        }
    }

    pub fn print_guesses(&self) -> (){
        println!("{:?}", self.guessed_letters);
    }

    pub fn print_discovered_word(&mut self) -> (){
        // for i in 0..self.secret.len() {
        //     if self.guessed_letters.contains(&(self.secret.as_bytes()[i] as char)) && !self.discovered_word.contains(&(self.secret.as_bytes()[i] as char)) {
        //         println!("yes");
        //         self.discovered_word[i] = self.secret.as_bytes()[i] as char;
        //     }
        // }
        println!("{:?}", self.discovered_word);
    }

    pub fn get_stage(&self) -> i32 {
        return self.stage;
    }

    pub fn check_hit(&mut self, guess: char) -> () {
        // println!("------------------------------");
        // if self.guessed_letters.contains(&guess) {
        //     println!("Already Tried this one you silly little sea cookie, try again!");
        // }
        if self.secret.contains(&guess.to_string()) {
            for i in 0..self.secret.len() {
                if self.guessed_letters.contains(&(self.secret.as_bytes()[i] as char)) { //&& !self.discovered_word.contains(&(self.secret.as_bytes()[i] as char)) {
                    self.discovered_word[i] = self.secret.as_bytes()[i] as char;
                }
            }
            println!("The secret word contains: {}", &guess.to_string());
        }
        else{
            println!("MISS! The secret word does not contain: {}", &guess.to_string());
            self.stage += 1;
        }
        
    }


    pub fn won(&self) -> bool {
        if self.discovered_word.contains(&'_') == false {
            return true;
        }
        else    {
            return false;
        }
    }
}

//picks a random word from a sting set

fn pick_word(words: &[String]) -> &str{
    let index = thread_rng().gen_range(0..=words.len());
    return &words[index];
}

//reads in a word file

fn build_word_array() -> Vec<String>{
    let file = fs::read_to_string("hangman/src/words.txt").expect("File Read");
    let mut words = Vec::new();

    for line in file.lines() {
        words.push(line.to_string());
    }

    return words;
}



fn main() {
    //init the game standards

    let _rng = rand::thread_rng();
    //Dynamic Array of Words
    let words = build_word_array();

    println!("Welcome to Hangman!");
    let secret: String = pick_word(&words).to_lowercase();
    //println!("{secret}");

    //Game new(secret, discovered_word, game_on, guessed_letters, stage)
    let mut hangman = Game::new(secret.clone(),vec!['_'; secret.len()],  true,Vec::<char>::new(), 0);
    let term = Term::stdout();

    //While game is running
    //1. Print guesses that have been made
    //2. Print all the discovered_word
    //3. Print the stage
    //4. ASk for user input
    //6. Add Guess to guess list
    //7. Check to see if there was a hit 
    //8. Check winning
    
    while hangman.get_game_status(){
        
        print!("Guesses: ");
        hangman.print_guesses();

        //Prints the blanks and the correct guesses

        println!("Secret Word: ");
        hangman.print_discovered_word();

        hangman.stage();

        let guess: char = term.read_char().expect("Unable to Read");
        println!("{guess}");
        
        
        hangman.add_to_guesses(guess);

        hangman.check_hit(guess);
        

        if hangman.get_stage() == 7 {
            println!("You died!\nThe secret word was {secret}!");
            hangman.end_game();
        }else if hangman.won() {
            println!("-------------------------------");
            println!("\n\nCongrats, you won! The word was {secret}!");
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