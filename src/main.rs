use std::io;

use crate::logic::{init, mode};

mod logic;

fn main() {
    loop {
        let file = init();
        println!();
        println!("Todo List:");
        if file.content.is_empty() {
            println!("No ToDo list yet");
        } else {
            println!("{}", file.content);
        };
        println!();
        println!("Option:");
        println!("1. add todo list");
        println!("2. remove todo list");
        println!("3. clean todo list");
        println!("4. to exit");
        eprint!("input: ");

        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Invalid input");
            let input: Result<u32, _> = input.trim().parse();
            match input {
                Ok(_) => (),
                Err(_) => {
                    println!("Enter valid option");
                    eprint!("input: ");
                    continue;
                }
            }
            let _ = mode(input.unwrap(), file);
            break;
        }
    }
}
