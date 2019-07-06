extern crate termion;
use std::env;
use std::path;
use std::process::Command;
use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use std::io::{Write, stdout, stdin};

fn init(){
    println!("{}[2J", 27 as char);

}
fn dir_print(isPrompt : bool){
    let cwd = env::current_dir().expect("Error obtaining current directory!");
    let mut new_path = path::PathBuf::new();
    if cwd.starts_with("/home/"){
        let cwd: path::PathBuf = cwd.iter().skip(2).collect();
        new_path.push("~");
        new_path.push(&cwd);
    }
    if isPrompt {
        print!("{} > ", new_path.display());
    } else {
        println!("{}", new_path.display());
    }
    std::io::Write::flush(&mut std::io::stdout()).expect("Failed to flush values!");
}
fn get_input() -> String {
    let mut data_read = String::new();

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    stdout.flush().unwrap();

    for c in stdin.keys() {
        //write!(stdout, "{}{}", termion::cursor::Goto(1,1), termion::clear::CurrentLine).unwrap();
        match c.unwrap() {
            Key::Char('q') => {
                data_read = data_read + "q";
                print!("q");
            },
            Key::Char('\n') => { 
                break;
            }
            Key::Char(c) => {
                data_read = data_read + &c.to_string();
                print!("{}", c);
            },                
            _ => {
                //print!("Other");
            },
        }
        stdout.flush().unwrap();
    }
    println!("\r");

   //let termios = Termios::from_fd(stdin).unwrap();
    //std::io::stdin().read_line(&mut data_read).expect("Couldn't read a valid string!");

    //TODO add_history

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
    let mut temp = input.split(" ");
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
    let mut p = path::Path::new("./");
    if args.len() > 0 {
        p = path::Path::new(&args[0]);
        env::set_current_dir(p).expect("No such path");
    }
}
fn run_command(input : String, args : Vec<String>){
    //let mut out = Command::new(input.trim()).args(args).output().expect("Failed to start");
    let process = Command::new(input.trim()).args(args).spawn();
    match process {
        Ok(mut child) => {
            child.wait();
        },
        Err(err) => {
            println!("Uh Oh, Invalid Command");
        }
    }

    /*let cwd = env::current_dir().expect("Error obtaining current directory!");
    cmd.current_dir("/");
    */
}

fn main() {

    while true  {
        let mut input = String::new();
        let mut args : Vec<String> = vec![String::new(); 0];
        let mut exec_flag : i32 = -1;

        dir_print(true);
        input = get_input();    /*let cwd = env::current_dir().expect("Error obtaining current directory!");
    cmd.current_dir("/");
    */
        exec_flag = process_input(&input, &mut args);

        if input.trim().contains(" ") {
            input = input.split(" ").next().unwrap().to_string();
        } else { input = input.trim().to_string()};

        if exec_flag == 1 {
            run_command(input, args);
        }
    }
}
