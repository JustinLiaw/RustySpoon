//words.txt file from https://github.com/Xethron/Hangman/blob/master/words.txt


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
fn letter_in_word(){

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
        
    }
    //TODO: 
    //Ask user for input
    //if(letter_in_word == true)
    //fill out guess list
    //call build_hangman()
    //call build_text()
          
}
