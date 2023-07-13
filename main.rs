use std::fs::{File, OpenOptions};

use std::io::{self, prelude::*, BufReader};

use std::io::Write;
// use std::fs::File;
use std::process::Command;

use std::collections::HashMap;

fn fake_main() -> i32 {
    // Let's say our program fails
    // So, we return one to the OS
    0
}

fn open_link(url: &str) -> std::io::Result<()> {
    let result = match std::env::consts::OS {
        "macos" => Command::new("open").arg(url).output(),
        "windows" => Command::new("cmd").arg("/C").arg("start").arg(url).output(),
        "linux" => Command::new("xdg-open").arg(url).output(),
        _ => panic!("Unsupported operating system!"),
    };

    match result {
        Ok(output) => {
            if output.status.success() {
                Ok(())
            } else {
                let error_msg = String::from_utf8_lossy(&output.stderr);
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    error_msg.to_string(),
                ))
            }
        }
        Err(err) => Err(err),
    }
}

fn main() {
    println!("___________        .___               .__  .__          __    ");
    println!(r"\__    ___/___   __| _/____           |  | |__| _______/  |_  ");
    println!(r"  |    | /  _ \ / __ |/  _ \   ______ |  | |  |/  ___/\   __\ ");
    println!(r"  |    |(  <_> ) /_/ (  <_> ) /_____/ |  |_|  |\___ \  |  |   ");
    println!(r"  |____| \____/\____ |\____/          |____/__/____  > |__|   ");
    println!(r"                    \/                             \/        ");

    println!(
        " Available Commands: \n\n -add [task]        -ls \n\n -quit              -check [task] \n"
    );

    loop {
        let (command, parameter) = get_user_input();
        match_command(&command, &parameter);
    }
}

fn get_user_input() -> (String, String) {
    print!(">> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut words = input.trim().split_whitespace();
    let first_word = words.next().unwrap_or("").to_string();
    let second_word = words.next().unwrap_or("").to_string();

    (first_word, second_word)
}

fn match_command(command: &str, parameter: &str) {
    match command {
        "quit" => {
            let exit_code = fake_main();
            std::process::exit(exit_code);
        }

        "add" => {
            add(&parameter);

            if &parameter != &"" {
                println!(" new element added! -> {parameter}")
            }
        }

        "ls" => {
            let tasks = cache();
            readall(tasks);
        }

        "check" => {
            check(parameter);
        }

        "" => print!(""),

        "bastava-chiedere" => {
            let url = "https://www.youtube.com/watch?v=xvFZjo5PgG0&pp=ygUIcmlja3JvbGw%3D";
            match open_link(url) {
                Ok(_) => println!("Link opened successfully."),
                Err(err) => eprintln!("Failed to open link: {}", err),
            }
        }
        _ => println!(" command not found"),
    }
}

fn add(name: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("output.txt")
        .expect("Failed to open file");

    if name != "" {
        // Write the line to the file.
        file.write_all(name.as_bytes())
            .expect("Failed to write to file");

        // Optional: Add a newline after writing the line.
        file.write_all(b"\n")
            .expect("Failed to write newline to file");
    }
}

fn cache() -> HashMap<String, bool> {
    let file = File::open("output.txt").expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut tasks: HashMap<String, bool> = HashMap::new();

    for line in reader.lines() {
        match line {
            Ok(line) => {
                tasks.insert(line.clone(), false);
            }
            Err(err) => {
                eprintln!("Error reading line: {}", err);
            }
        }
    }
    tasks
}

fn readall(tasks: HashMap<String, bool>) {
    let done = emojis::get("✅").unwrap();
    let undone = emojis::get("❌").unwrap();

    for (key, value) in tasks {
        if value {
            println!(" ~ {}  {}", key, done);
        } else {
            println!(" ~ {}  {}", key, undone);
        }
    }
}

fn check(_name: &str) {
   println!("Yet to be implemented :) 🦄");
}
