//use std::rand;
use rand::Rng;

trait Lyrics {
    fn bottles(&self, bool) -> Self;
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
    //part a and b: Counts upwards to 100 inc 1. Lyrics changed. 
    // for i in 1..100 {
    //     i.bottles(true).wall().mid().bottles(false).end();
    //     i.take().bottles(false).wall().end();
    //     println!();
    // }

    for i in 1..500 {
        i.bottles(true).wall().mid().bottles(false).end();
        i.take().bottles(false).wall().end();
        println!();
        i = i + 1;
    }
}
