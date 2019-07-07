extern crate termion;
use std::env;
use std::path;
use std::process::Command;
use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::cursor;
use std::io::{Write, stdout, stdin};
use std::char;


fn dir_print(is_prompt : bool){
    let cwd = env::current_dir().expect("Error obtaining current directory!");
    let mut new_path = path::PathBuf::new();
    if cwd.starts_with("/home/"){
        let cwd: path::PathBuf = cwd.iter().skip(2).collect();
        new_path.push("~");
        new_path.push(&cwd);
    }
    if is_prompt {
        print!("{} > ", new_path.display());
    } else {
        println!("{}", new_path.display());
    }
    std::io::Write::flush(&mut std::io::stdout()).expect("Failed to flush values!");
}
fn get_input(history: &mut Vec<String>) -> String {
    let mut data_read = String::new();

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    
    history[1] = String::new();
    let mut history_index: i32 = 1;

    stdout.flush().unwrap();

    let l = char::from_digit(9, 10);
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => {
                data_read = data_read + "q";
                print!("q");
            },
            Key::Char(' ') {

            },
            Key::Char(l) => {

            },
            Key::Char('\n') => { 
                break;
            },
            Key::Char(c) => {
                data_read = data_read + &c.to_string();
                print!("{}", c);
            },
            Key::Backspace => {
                if data_read.len() > 0{
                    data_read = data_read[..(data_read.len()-1)].to_string();
                    print!("{} {}", cursor::Left(1), cursor::Left(1));
                }
            },
            Key::Up => {
                if history_index < ((history.len()-1) as i32) || history_index == 0{
                    if history_index == 1 && data_read != ""{
                        history[1] = data_read.clone();
                    }
                    history_index = history_index + 1; 
                    if data_read.len() != 0 {
                        let spaces = " ".repeat(data_read.len());
                        print!("{}{}{}", cursor::Left(data_read.len() as u16), spaces, cursor::Left(data_read.len() as u16));
                    }
                    
                    if history.len() > 0{
                        let d = history_index.clone();
                        data_read = (&history[d as usize]).to_string();
                        print!("{}", data_read);
                    }
                }
            },
            Key::Down => {
                if history_index > 0 {
                    if history_index == 1{
                        history[1] = data_read.clone();
                    }
                    if !(history[0] == data_read && history_index == 1) {
                        history_index = history_index - 1;
                    }
                    if data_read.len() != 0 {
                        let spaces = " ".repeat(data_read.len());
                        print!("{}{}{}", cursor::Left(data_read.len() as u16), spaces, cursor::Left(data_read.len() as u16));
                    }
                    if history.len() > 0{
                        let d = history_index.clone();
                        data_read = (&history[d as usize]).to_string();
                        print!("{}", data_read);
                    }
                }
            }
            _ => {
                //print!("Other");
            },
        }
        stdout.flush().unwrap();
    }
    if history.len() > 2 {
        if history[2] != data_read {
            history.insert(2, data_read.clone());
        }
    } else {
        history.insert(2, data_read.clone());
    }
    println!("\r");

    return data_read;
}
fn process_input(input : &str, args : &mut Vec<String>) -> i32{
    split_space(input, args);
    
    //Check if piped

    //Check if own command
    if handle_own_commands(input.to_string(), args.to_vec()) {
        return 0;
    }
    else {
        return 1;
    };
    //Otherwise return and allow running of process; If piped return 2 and allow running of pipethrough

}
fn split_space(input : &str, args : &mut Vec<String>){
    let temp = input.split(" ");
    let mut i = 0;
    for arg in temp {
        if i == 0 {
            i = i + 1;
        }
        else if arg != ""{
            args.push(arg.trim().to_string());
        }
    }
}
fn handle_own_commands(input : String, args : Vec<String>) -> bool{
    let mut commands : [String; 4] = Default::default();
    let mut run_val = 0;
    let mut i : i32 = 0;

    let mut success = false;

    let cmd = input.split(" ").next().unwrap().trim().to_string();

    commands[0] = String::from("exit");
    commands[1] = String::from("help");
    commands[2] = String::from("cd");
    commands[3] = String::from("cwd");
    for command in commands.iter() {
        if cmd == command.to_string() {
            run_val = i + 1;
            success = true;
            break;
        }
        i = i + 1;
    }
    match run_val {
        1 => {::std::process::exit(0);},
        2 => {},
        3 => {change_dir(args);},
        4 => {dir_print(false);},
        _ => {}
    }
    return success;
}
fn change_dir(args : Vec<String>){
    let p;
    if args.len() > 0 {
        p = path::Path::new(&args[0]);
        env::set_current_dir(p).expect("No such path");
    }
}
fn run_command(input : String, args : Vec<String>){
    let process = Command::new(input.trim()).args(args).spawn();
    match process {
        Ok(mut child) => {
             match child.wait() {
                 Ok(val) => {
                     println!("{}", val);
                 },
                 Err(err) => {
                    println!("{}", err);
                 }
             };
        },
        Err(_err) => {
            println!("Uh oh, Invalid Command");
        }
    }
}

fn main() {

    let mut history : Vec<String> = vec!["".to_string(), "".to_string()];

    loop {
        let mut args : Vec<String> = vec![String::new(); 0];

        dir_print(true);
        let mut input = get_input(&mut history);    

        let exec_flag = process_input(&input, &mut args);

        if input.trim().contains(" ") {
            input = input.split(" ").next().unwrap().to_string();
        } else { input = input.trim().to_string()};

        if exec_flag == 1 {
            run_command(input, args);
        }
    }
}
