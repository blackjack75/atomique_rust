use inquire::{Text, validator::{StringValidator, Validation} , error::InquireError, Select};
use termion::{color, clear, cursor};

//use std::time::Duration;
//use std::thread;

mod todo;

fn main() {

    header();
    menu();
}


fn header() {

    println!("{}", clear::All);
    println!("{}", cursor::Goto(1, 1));


    println!("{cyan}Atomique Rust 1.0.0{reset}",
     cyan = color::Fg(color::Cyan),
     reset = color::Fg(color::Reset));
       println!("\r");
}

fn menu() {

    'menuloop: loop {

    let options: Vec<&str> = 
        vec!["Exit","Todo","Note","Read list", "Watch list"];

    let ans: Result<&str, InquireError> =
        Select::new("Select Module", options).prompt();

    match ans {

     Ok(choice) => {

        if choice == "Exit" {
           println!("ktxbye");

           break 'menuloop;

        } else if choice == "Todo" {
           header();
           todo::show();
        } else {
            header();
            println!("{} Module not implemented yet", choice);
           }        
      },

    Err(_) =>  { 
        header();
        println!("There was an error, please try again");
       },

     }

  }

}
