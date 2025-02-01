use std::io::{self, Read};
extern crate  rand;
use rand::Rng;
use std::num::ParseIntError;
use std::cmp::Ordering;

#[derive(Debug)]
struct Player{
    heart: u8,
    name: String,
    loan: bool,
}

impl Player{
    fn new(heart: u8, name: String, loan: bool) -> Self{
      Self{
        heart,
        name,
        loan
            }
        }
    
    fn add(&mut self, val: u8){
        self.heart += val
    }
    fn sub(&mut self, val:u8){
        if self.heart <= 0{
            println!("Consider borrowing...");
        }
        else {    
        self.heart = self.heart.saturating_sub(val);
        }
    }
}

fn get_guess() -> u8{
    let mut input = String::new();
    loop{
        input.clear();
    println!("What did you choose: ");
    io::stdin().read_line(&mut input).expect("Error Reading your input");
    match input.trim().parse::<u8>(){
        Ok(num) => return num,
        Err(err) => match err.kind(){
            std::num::IntErrorKind::Empty => println!("Error: Input Is Empty."),
            std::num::IntErrorKind::InvalidDigit => println!("Error: Input is invalid."),
            std::num::IntErrorKind::PosOverflow => println!("Error: Your input long o."),
            std::num::IntErrorKind::NegOverflow => println!("Error: No negatives here."),
            _ => println!("Error: Unknown parsing error."),
        },
    };
}
    }


fn get_name() -> String{
   let mut name: String = String::new();
   loop {
         println!("Hol on:)\n++++ What's your name?: ");
   io::stdin().read_line(&mut name).expect("Error while reading your name");

   if (name.trim().is_empty()){
      println!("\nYour name Cannot be empty!\n");
      continue;
   }
   break
   }
   name.trim().to_string()
 }

fn main() {
    // Display message
    let name = get_name();
    let mut player = Player::new(5 as u8, name.clone(), false);
   println!("++++ Hello This is a guess-game\n++++ Works 100% with your connection to the universe
       and how lucky you are at guessing\n++++ Let's see your skills");    
   let sys_guess = rand::thread_rng().gen_range(1..=70) as u8;
   println!("++++ Gotcha {}!\n++++ Welcome to the guess Zone you have just 5 hearts at start:)\n 
       Each failed attempt is minus one heart and each successful guess is plus 2 hearts:)
       You can take a loan of 3 hearts from our hearts bank after been broke of heart\n 
       We will take back the hearts from your successful guesses and you get to take the loan just once\n 
       if after the loan you make no successful guess, we will kick your ass out of the game:)", name);
   println!("Press Enter to continue...");
   let mut input: String = String::new();
   io::stdin().read_line(&mut input).expect("An error occured");
   println!("Bye...");

   loop {   
    let guess = get_guess();
    match guess{
           num if num > 100 || num < 1 => println!("Sorry Enter a number between 1 and 101\n{} is not valid", num),
           _ => match guess.cmp(&sys_guess) {
               Ordering::Less => println!("Sorry that is too small\nTry again"),
               Ordering::Greater => println!("Sorry that was too big"),
               Ordering::Equal => {
                   println!("That's correct...You just earned +2 hearts");
                   &player.add(2);
                   break;
               },
           }
       }
    if player.heart >0 {
        player.sub(1);
    }
    else {
        
    }
   }   
}
