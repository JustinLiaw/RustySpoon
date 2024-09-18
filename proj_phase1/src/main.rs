use rand::Rng;
use std::io;
use prime_factorization::Factorization;
use std::{thread, time};

trait Lyrics {
    fn bottles(&self, bool: bool) -> Self;
    fn take(&self) -> Self;
    fn wall(&self) -> Self;
    fn mid(&self) -> Self;
    fn end(&self);
    fn long_num(&self ,num:u32) -> String;
}

impl Lyrics for u32 {
    // Part G: Breaks all the printed text into prime factorization

    fn long_num(&self, num: u32) -> String {
        //Runs the factor repr
        let factor_repr = Factorization::run(num);
        if num == 1 {
            return "()".to_string();
        }
        if factor_repr.is_prime {
            return "()".to_string();
        }
        // println!("{} <- The factorization of the number of bottles", factor_repr);
        // assert_eq!(factor_repr.factors, vec![2, 3, 5]);
        // *self
        // *self
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
        // println!("{}" ,factor_repr.factors[0]);
        //  return factor_repr;
        // return factor_repr.factors.pop()
        
        

        return out;
    } 

    fn bottles(&self, cap:bool) -> u32 {
        match *self {
            0 => print!("{}o more lines of text", if cap {"N"} else {"n"}),
            1 => print!("{} line of text", self.long_num(*self)),
            _ => print!("{} lines of text", self.long_num(*self))
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
    let mut _input = String::new();
    println!("How many lines?");
    let _ = io::stdin().read_line(&mut _input);
    let num: u32 = _input.trim().parse().expect("Bad");

    for i in 1..num {
        i.bottles(true).wall().mid().bottles(false).end();
        i.take().bottles(false).wall().end();
        println!();

        //Time and Sleep
        let one_sec = time::Duration::from_secs(1);
        thread::sleep(one_sec);
    }
}