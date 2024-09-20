use rand::Rng;
use std::io;
use prime_factorization::Factorization;
use std::{thread, time};
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

trait Lyrics {
    //Normal Bottles
    fn bottles(&self, bool: bool) -> Self;
    //Bottles for factors
    fn bottles_fact(&self, bool: bool) -> Self;
    //Bottles for Files
    fn bottles_file(&self, bool: bool, file: &mut File) -> Self;
    //Normal Take
    fn take(&self) -> Self;
    //Normal File
    fn take_file(&self,file: &mut File) -> Self;
    //Normal Wall
    fn wall(&self) -> Self;
    //Wall File
    fn wall_file(&self,file:&mut File) -> Self;
    //Normal Mid
    fn mid(&self) -> Self;
    //Mid File
    fn mid_file(&self,file:&mut File) -> Self;
    
    fn end(&self);
    fn end_file(&self, file:&mut File);
    // fn long_num(&self ,num:u32, File: File) -> String;
    fn long_num(&self, num:u32) -> String;
}

impl Lyrics for u32 {
    // Part G: Breaks all the printed text into prime factorization
    fn long_num(&self, num: u32) -> String {
        //Runs the factor repr
        let factor_repr = Factorization::run(num);
        //If number is 1 or prime than there are no factors.
        if num == 1 {
            return "()".to_string();
        }
        if factor_repr.is_prime {
            return "()".to_string();
        }
        //mut = mutable, or changeable.
        //Rust Factorization crate takes a number and produces a Vector, 
        //which is somewhat like an array, that contains the number's 
        //factors. This loop just pulls the factor out of the vector 
        //and builds the string in the (x*x*x) format.
        let mut out : String = "(".to_string();
        for i in 0..factor_repr.factors.len() {
            if i == factor_repr.factors.len() - 1   {

                out += &(factor_repr.factors[i].to_string() + &(")"));
            }
            else {
                let temp :String = "*".to_string();
                out += &(factor_repr.factors[i].to_string() + &temp);
            } 
        }

        return out;
    } 

    //Normal Bottles
    fn bottles(&self, cap:bool) -> u32 {
        match *self {
            0 => print!("{}o more lines of text", if cap {"N"} else {"n"}),
            1 => print!("{} line of text", *self),
            _ => print!("{} lines of text", *self)
        }
        *self
    }

    //Bottles Factors that uses the long_num function
    //for part G
    fn bottles_fact(&self, cap:bool) -> u32 {
        match *self {
            0 => print!("{}o more lines of text", if cap {"N"} else {"n"}),
            1 => print!("{} line of text", self.long_num(*self)),
            _ => print!("{} lines of text", self.long_num(*self))
        }
        *self
    }

    //Normal Take
    fn take(&self) -> u32 {
        match *self {
            _ => { print!("Print it out, stand up and shout, "); *self + 1 }
        }
    }

    //Normal Wall
    fn wall(&self) -> u32 {
        print!(" on the screen");
        *self
    }

    //Normal Mid
    fn mid(&self) -> u32 {
        print!(", ");
        *self
    }

    //Normal End
    fn end(&self) {
        println!(".");
    }

    //All the file functions take in a File Object as an argument.

    //Bottles File: Uses the write_all function to write directly to 
    //file.
    fn bottles_file(&self, cap: bool, file:&mut File) -> Self {
        let first_letter = if cap {"N"} else {"n"}.to_string();
        let _ = match *self {
            0 => file.write_all((first_letter + "o more lines of text").as_bytes()),
            1 => file.write_all(((self.long_num(*self)).to_string() + " line of text").as_bytes()),
            _ => file.write_all(((self.long_num(*self)).to_string() + " lines of text").as_bytes()),
        };
        *self
    }

    //Take File
    fn take_file(&self, file:&mut File) -> u32 {
        match *self {
            _ => { let _ = file.write_all("Print it out, stand up and shout, ".as_bytes()); *self + 1 }
        }
    }

    //Wall File
    fn wall_file(&self, file:&mut File) -> u32 {
        let _ = file.write_all(" on the screen".as_bytes());
        *self
    }

    //Mid File
    fn mid_file(&self, file:&mut File) -> u32 {
        let _ = file.write_all(", ".as_bytes());
        *self
    }

    //End file
    fn end_file(&self, file:&mut File) {
       let _ = file.write_all(".\n".as_bytes()); 
    }
}


fn main() {
    //part A and B: Counts upwards to 100 inc 1. Lyrics changed. 
    //Simple for loop
    for i in 1..100 {
        i.bottles(true).wall().mid().bottles(false).end();
        i.take().bottles(false).wall().end();
        println!();
    }

//Part C: count up to 500 but increment by a random number
    //Use Rand::rng crate to generate a number between 1 and 10 
    //inclusive.
    let mut rng = rand::thread_rng();
    let r_num: usize = rng.gen_range(1..=10);

    //Change end to 500. Use step_by function to increment by r_num
    for i in (1..520).step_by(r_num) {
        if i > 500  {
            // i = 499;
            499.bottles(true).wall().mid().bottles(false).end();
            499.take().bottles(false).wall().end();
            println!();
            break
        }
        else {
            i.bottles(true).wall().mid().bottles(false).end();
            i.take().bottles(false).wall().end();
            println!();
        }
    }

//Part D: Display a prompt like "How many lines? " and read an 
    //integer from the terminal. Stop at that number
    //Uses io::stdin() to take in user input
    let mut _input = String::new();
    println!("How many lines?");
    let _ = io::stdin().read_line(&mut _input);
    let num: u32 = _input.trim().parse().expect("Bad");

    for i in 1..num  {
        i.bottles(true).wall().mid().bottles(false).end();
        i.take().bottles(false).wall().end();
        println!();
    }

//Part F: Wait 1 second between each
    let mut _input = String::new();
    println!("How many lines?");
    let _ = io::stdin().read_line(&mut _input);
    let num: u32 = _input.trim().parse().expect("Bad");

    for i in 1..num {
        i.bottles(true).wall().mid().bottles(false).end();
        i.take().bottles(false).wall().end();
        println!();

        //Time and Sleep to wait one second
        let one_sec = time::Duration::from_secs(1);
        thread::sleep(one_sec);
    }

//Part G: Factors (explained in long_num())
    let mut _input = String::new();
    println!("How many lines?");
    let _ = io::stdin().read_line(&mut _input);
    let num: u32 = _input.trim().parse().expect("Bad");

    //Uses bottles_fact that we made
    for i in 1..num {
        i.bottles_fact(true).wall().mid().bottles_fact(false).end();
        i.take().bottles_fact(false).wall().end();
        println!();

        //Time and Sleep
        let one_sec = time::Duration::from_secs(1);
        thread::sleep(one_sec);
    }

// Part H: Writing to file
    let mut _input = String::new();
    println!("How many lines?");
    let _ = io::stdin().read_line(&mut _input);
    let num: u32 = _input.trim().parse().expect("Bad");

    //Creates File named out.txt
    let path = Path::new("out.txt");
    let mut file = match File::create(&path) {
        Err(_) => panic!("couldn't create"),
        Ok(file) => file,
    };

    //Writes to File
    for i in 1..num {
        i.bottles_file(true,&mut file).wall_file(&mut file).mid_file(&mut file).bottles_file(false,&mut file).end_file(&mut file);
        i.take_file(&mut file).bottles_file(false,&mut file).wall_file(&mut file).end_file(&mut file);
        let _ = file.write_all(b"\n");
    }

    //Prints to stdout
    for i in 1..num {
        i.bottles_fact(true).wall().mid().bottles_fact(false).end();
        i.take().bottles_fact(false).wall().end();
        println!();
    }

    // Part I: Writing to file and stdout with Threads (also to stderr)
    let mut _input = String::new();
    println!("How many lines?");
    let _ = io::stdin().read_line(&mut _input);
    let num: u32 = _input.trim().parse().expect("Bad");
    let path = Path::new("out.txt");

    let mut file = match File::create(&path) {
        Err(_) => panic!("couldn't create"),
        Ok(file) => file,
    };

    //Spawned thread to run the File functions
    //move: Transfers ownership of things from main to the spawned thread
    //Basically allows the num variable to be used by the spawned
    //thread
    let handle = thread::spawn(move ||{
        for i in 1..num {
            i.bottles_file(true,&mut file).wall_file(&mut file).mid_file(&mut file).bottles_file(false,&mut file).end_file(&mut file);
            i.take_file(&mut file).bottles_file(false,&mut file).wall_file(&mut file).end_file(&mut file);
            let _ = file.write_all(b"\n");
        }
        
        let _ = std::io::stderr().write(b"Haha I'm a thread!\n");
    });

    //Main Thread to run normal Bottles Factor
    for i in 1..num {
        i.bottles_fact(true).wall().mid().bottles_fact(false).end();
        i.take().bottles_fact(false).wall().end();
        println!();
    }
    std::io::stderr().write(b"It's me the main!\n").unwrap();

    //Joined is called on the main thread to wait to terminate until the 
    //spawned thread is finished
    handle.join().unwrap();
}