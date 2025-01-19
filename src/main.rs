use std::env;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::process::Command;

fn init() -> io::Result<()>{
    let mut exit = String::from(" ");

    
    while  exit != "exit".to_string() {
        print!("> ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let token: Vec<&str> = input.trim().split(' ').collect();
        write_history(input.clone());        
        match token[0] {
            "exit" => { exit = token[0].to_string()}, 
            "pwd" => { show_pwd() }
            "cd" => { set_current_directory(token[1]) },
            "echo" => { print_echo(token[1])},
            "history" => { history() },
            "clear" => { clear_console()}
            _ => { extern_command(token)}
        }
    }
    Ok(())
}

fn show_pwd() {

    match env::current_dir() {
        Ok(dir) => {
            if let Some(pwd) = dir.to_str() {
                println!("{}", pwd);
            }
        },
        Err(e) => {
            eprintln!("Error getting the path: {}", e);
        }
    }
}

fn set_current_directory(path: &str) {
    let _ = env::set_current_dir(path);
}

fn print_echo(text: &str) {
    if text.starts_with('"') && text.starts_with('"') {
        println!("{}", text.trim_matches('"'));
    } else if text.ends_with('\'') && text.ends_with('\'') {
        println!("{}", text.trim_matches('\''));
    }else {
        println!("{}", text);
    }
}

fn clear_console() {
    print!("\x1B[2J\x1B[H")
}

fn history () {
    if let Ok(file) = File::open(".gsh_history") {
        let reader = BufReader::new(file);
        for (index, line )in reader.lines().enumerate() {
            match line {
               Ok(line)  => println!("{}", line),
               Err(e) => eprintln!("Error in line {}: {}", index+1, e)
            }
        }
    } else {
        println!("file not open");
    }
}

fn write_history(history: String) {
    if Path::new(".gsh_history").exists(){
        let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(".gsh_history")
        .unwrap();

        let mut writer = BufWriter::new(file);
        writer.write_all(history.as_bytes()).unwrap();
        writer.flush().unwrap();
    } else {
        let file = File::create(".gsh_history");
        let mut writer = BufWriter::new(file.unwrap());
        writer.write_all(history.as_bytes()).unwrap();
        writer.flush().unwrap();
    }
}

fn extern_command(commands: Vec<&str>) {
    if commands.len() == 1  {
        let output = Command::new(commands[0]).output().expect("Error when executing command");
        println!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        let output = Command::new(commands[0]).arg(commands[1]).output().expect("Error when execute command");
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
}

fn main() -> io::Result<()>{
    let _ = init();    
    Ok(())
}
