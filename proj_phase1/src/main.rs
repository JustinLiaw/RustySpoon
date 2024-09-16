use rand::Rng;
use std::io;

trait Lyrics {
    fn bottles(&self, bool: bool) -> Self;
    fn take(&self) -> Self;
    fn wall(&self) -> Self;
    fn mid(&self) -> Self;
    fn end(&self);
}

impl Lyrics for u32 {
    fn bottles(&self, cap:bool) -> u32 {
        match *self {
            0 => print!("{}o more lines of text", if cap {"N"} else {"n"}),
            1 => print!("{} line of text", self),
            _ => print!("{} lines of text", self)
        }
        *self
    }

    fn take(&self) -> u32 {
        match *self {
            // '_' means else
            _ => { print!("Print it out, stand up and shout, "); *self + 1 }
            //_ => { print!("Go to the lab and type some more, "); *self + 1 }
        }
    }

    fn wall(&self) -> u32 {
        print!(" on the screen");
        *self
    }

    fn mid(&self) -> u32 {
        print!(", ");
        *self
    }

    fn end(&self) {
        println!(".");
    }
}

fn main() {
    //part A and B: Counts upwards to 100 inc 1. Lyrics changed. 
    // for i in 1..100 {
    //     i.bottles(true).wall().mid().bottles(false).end();
    //     i.take().bottles(false).wall().end();
    //     println!();
    // }

    //Part C: count up to 500 but increment by a random number
    // let mut rng = rand::thread_rng();
    // let r_num: usize = rng.gen_range(1..=10);

    // for i in (1..500).step_by(r_num) {
    //     i.bottles(true).wall().mid().bottles(false).end();
    //     i.take().bottles(false).wall().end();
    //     println!();
    // }

    //Part D: Display a prompt like "How many lines? " and read an integer from the terminal. Stop at that number
    // let mut _input = String::new();
    // println!("How many lines?");
    // let _ = io::stdin().read_line(&mut _input);
    // let num: u32 = _input.trim().parse().expect("Bad");

    // for i in 1..num {
    //     i.bottles(true).wall().mid().bottles(false).end();
    //     i.take().bottles(false).wall().end();
    //     println!();
    // }

    //Part F: Wait 1 second between each
    
}