extern crate  rand;
use rand::Rng;
// use std::num::ParseIntError;
use std::cmp::Ordering;
use std::process::Command;
use std::io;
use colored::*;
use std::fmt::{format, Display};

fn print_banner() {
    let banner = r#"
    ██████╗ ██╗  ██╗██████╗  ██╗███████╗
    ██╔═══██╗╚██╗██╔╝██╔══██╗███║╚════██║
    ██║   ██║ ╚███╔╝ ██████╔╝╚██║    ██╔╝
    ██║   ██║ ██╔██╗ ██╔══██╗ ██║   ██╔╝ 
    ╚██████╔╝██╔╝ ██╗██████╔╝ ██║   ██║  
     ╚═════╝ ╚═╝  ╚═╝╚═════╝  ╚═╝   ╚═╝  
                                by peters
    "#;
    println!("{}", banner.bright_cyan());
}
#[macro_export]
macro_rules! print {
    ($msg:expr, $status_type:expr) => {{
        let prefix = match $status_type {
            "info"=> "[+]".bright_green(),
            "success"=> "[✓]".bright_blue(),
            "error"=> "[!]".bright_red(),
            "wait"=> "[*]".bright_yellow(),
            _ => "[+]".bright_green(),
        };
       let msg = $msg;
        for line in msg.lines(){
            println!("{} {}", prefix, line);
        }
    }};
    ($msg:expr, $status_type:expr, $extra:expr) => {{
        let prefix = match $status_type {
            "info"=> "[+]".bright_green(),
            "success"=> "[✓]".bright_blue(),
            "error"=> "[!]".bright_red(),
            "wait"=> "[*]".bright_yellow(),
            _ => "[+]".bright_green(),
        };
        let formatted_msg = format!($msg, $extra);
        for line in formatted_msg.lines(){
            println!("{} {}", prefix, line);
        }
    }};
}

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
            print!("Consider borrowing...","error");
        }
        else {    
        self.heart = self.heart.saturating_sub(val);
        }
    }
    fn lend(&mut self){
        self.heart = 3;
        self.loan = true;
    }
    fn pay(&mut self){
        self.sub(3);
        self.loan = false;
    }


}

fn clear_screen(){
    if cfg!(target_os = "windows"){
        Command::new("cmd")
        .args(["/c", "cls"])
        .status()
        .expect("Error: Failed to clear screen");
    }
    else {
        Command::new("clear")
        .status()
        .expect("Error: Unable to clear screen");
    }
}

fn get_guess(hearts: u8) -> u8{
    let mut input = String::new();
    loop{
        input.clear();
    println!("You currently have {}", "\u{2764} ".repeat(hearts.into()).green());
    print!("What did you choose: ", "info");
    io::stdin().read_line(&mut input).expect("Error Reading your input");
    match input.trim().parse::<u8>(){
        Ok(num) => return num,
        Err(err) => match err.kind(){
            std::num::IntErrorKind::Empty => print!("Error: Input Is Empty.", "info"),
            std::num::IntErrorKind::InvalidDigit => print!("Error: Input is invalid.", "info"),
            std::num::IntErrorKind::PosOverflow => print!("Error: Overflow?\nLow expecting something with u8.", "info"),
            std::num::IntErrorKind::NegOverflow => print!("Error: No negatives here.", "info"),
            _ => print!("Error: Unknown parsing error.", "info"),
        },
    };
}
    }
fn get_name() -> String{
   let mut name: String = String::new();
   loop {
         print!("Hol on:)\nWhat's your name?: ", "info");
   io::stdin().read_line(&mut name).expect("Error while reading your name");

   if name.trim().is_empty(){
      print!("\nYour name Cannot be empty!\n", "info");
      continue;
   }
   break
   }
   name.trim().to_string()
 }

fn get_input() -> Result<u8, String>{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading your input..");
    
    match input.trim().parse() {
        Ok(num) => return Ok(num),
        Err(_) => return Err(input)
    }
}

fn rand_guess() -> u8 {
    rand::thread_rng().gen_range(1..=70) as u8
}

fn main() {
    // Get Name and Display message
    print_banner();
    let name = get_name();
    let mut player = Player::new(5 as u8, name.clone(), false);
   print!("Hello This is a guess-game\nWorks 100% with your connection to the universe
       and how lucky you are at guessing\nLet's see your skills", "info");    
    let mut sys_guess = rand_guess();
   print!("Gotcha {}!\nWelcome to the guess Zone you have just 5 hearts at start:)\nEach failed attempt is minus one heart and each successful guess is plus 2 hearts:)\nYou can take a loan of 3 hearts from our hearts bank after been broke of heart\nWe will take back the hearts from your successful guesses and you get to take the loan just once\nif after the loan you make no successful guess, we will kick your ass out of the game:", "info", name);

   print!("Press Enter to continue...", "info");
   let _=  get_input();
   clear_screen();

   loop {   
    let guess = get_guess(player.heart);
    match guess{
           num if  !(1..=100).contains(&num) => print!("Sorry Enter a number between 1 and 101\n{} is not valid", "info", num),
           _ => match guess.cmp(&sys_guess) {
               Ordering::Less => print!("Sorry that is too small\nTry again", "error"),
               Ordering::Greater => print!("Sorry that was too big\nTry again", "error"),
               Ordering::Equal => {
                   print!("That's correct...You just earned +2 hearts", "info");
                   player.add(2);
                   if player.loan && player.heart >= 3{
                    print!("hehe Time to payback your loan", "info");
                    player.pay();
                   }
                   print!("Press y to proceed or any other key to quit", "info");
                   let proceed = get_input();
                   match proceed {
                     Ok(_) => {
                         print!("Bye", "info");
                         break;
                     },
                     Err(err) if err.trim().to_lowercase() == 'y'.to_string() => {
                        sys_guess = rand_guess();
                     },
                     _ => {
                         print!("Bye", "info");
                     }
                   }
               },
           }
       }
    if player.heart > 0 {
        player.sub(1);
    }
    else {
        print!("Consider borrowing as you are out of hearts", "error");
        print!("Enter yes to borrow or any other key to quit", "wait");
        let choice = get_input();
        match choice{
            Err(err) if err.trim().to_lowercase() == "yes"=> {
               if player.loan{
                print!("Sorry you already borrowed", "error");
                break;
               }
               else{
                player.lend();
                print!("You have been added 3 hearts to keep trying", "success");
                continue;
               }
            }
            _ => {
                print!("Bye", "info");
                break;
            }
        }
    }
   }   
}
