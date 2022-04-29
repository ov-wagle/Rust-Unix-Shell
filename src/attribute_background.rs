use crate::util;
use std::thread;

pub fn background_execution(history:Vec<String>, command: String) {
    let background_command: Vec<&str> = command.split("&").collect();
    
    if background_command.len() != 2 {
        println!("Invalid format");
        return;
    }

    if background_command[0].len() == 0 {
        println!("command cannot have a length of 0");
        return;
    }

    if background_command[1].len() != 0 {
        println!("& cannot be followed by a command");
        return;
    }

    let command_to_execute = String::from(background_command[0].trim());

    thread::spawn(move || {
        util::dispatch_function_helper(history.clone(), command_to_execute);
    });
}