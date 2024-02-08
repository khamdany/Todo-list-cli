use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::process;

#[derive(Clone)]
pub struct SaveFile {
    pub path: PathBuf,
    pub content: String,
}

pub fn init() -> SaveFile {
    let mut file = env::current_exe().unwrap();
    file.pop();
    file.push("todo.txt");
    let file_string = file.clone();
    let mut content = String::new();
    let path = Path::new(&file);

    if path.exists() {
        content = open(file);
    } else {
        let _ = fs::write(path, "");
    }
    let file = SaveFile {
        path: file_string,
        content,
    };
    file
}

fn open(file: PathBuf) -> String {
    let content = fs::read_to_string(file);
    content.unwrap()
}

pub fn mode(i: u32, s: SaveFile) {
    match i {
        1 => add(s),
        2 => remove(s),
        3 => clean(s),
        4 => process::exit(1),
        _ => println!("Select from option"),
    }
}

fn add(s: SaveFile) {
    println!("What to do?");
    let todo = input();
    let mut line: Vec<&str> = s.content.lines().collect();
    let new = format!("- {}", todo);
    line.push(&new);
    let new = line.join("\n");
    let _ = fs::write(s.path, new);
}

fn remove(s: SaveFile) {
    loop {
        let mut line: Vec<&str> = s.content.lines().collect();
        if line.is_empty() {
            println!("There nothing todo");
            break;
        }
        eprint!("what line you want to delete? ");
        let number = input();
        let number: Result<usize, _> = number.trim().parse();
        let mut real_number: usize = 0;
        match number {
            Ok(parsed) => real_number = parsed,
            Err(_) => {
                println!("Enter number ");
                continue;
            }
        }
        if number.unwrap().clone() > line.len() {
            println!("Not valid");
            continue;
        }
        if real_number > line.len() {
            println!("Print not valid");
            continue;
        }
        line.remove(real_number - 1);
        let new = line.join("\n");
        let _ = fs::write(s.path, new);
        break;
    }
}

fn clean(s: SaveFile) {
    let _ = fs::write(s.path, "");
}

fn input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid input");
    input
}
