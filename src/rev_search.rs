extern crate termion;
use std::io::{Write, stdout, stdin};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use crate::rev_search::termion::cursor::DetectCursorPos;
use crate::util;

fn search_substring(history: Vec<String>, command:String, mut index:usize) -> usize {
    while index < history.len() {
        if !history[history.len() - 1 - index].contains(command.as_str()) {
            index += 1;
            continue;
        }

        print!("\t -> {}", history[history.len() - 1 - index]);
        break;
    }

    index 
}


pub fn rev_search(mut history:Vec<String>) -> Vec<String> {
    // https://github.com/redox-os/termion/blob/master/examples/keys.rsm
    let stdin = stdin();
    let mut command = String::new();
    let mut index:usize = 0;
    let current = String::from("rev_search");

    if history.contains(&current) && history.len() == 1 {
        return history;
    }

    // Bring the console into raw mode where key detection happens
    let mut stdout = stdout().into_raw_mode().unwrap();
    stdout.flush().unwrap();

    // Fetch the current cursor position and set the same cursor position into the raw mode
    let (mut x, y) = stdout.cursor_pos().unwrap_or_else(|_error| {
        (0, 0)
    });

    if x == 0 && y == 0 {
        println!("Cannot run rev_search in background");
        return history;
    }

    write!(stdout, "{}{}", termion::cursor::Goto(x,y), 
           termion::clear::CurrentLine).unwrap();

    // Fetch the detection of every key on the keyboard. Detect every character on the keyboard
    // Detect backspace and ctrl key
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char(command_line) => {
                if command_line == '\n' {
                    // if enter is pressed, bring the cursor back to the dimension (1, y)
                    // and fetch the command at the given index.
                    write!(stdout, "{}{}", termion::cursor::Goto(1,y), 
                           termion::clear::CurrentLine).unwrap();
                    println!("{}", history[history.len() - 1 - index]);
                    write!(stdout, "{}{}", termion::cursor::Goto(1, y), 
                       termion::clear::AfterCursor).unwrap();
                    break;
                }

                // When new keys are pressed, every key press will be displayed based on the cursor
                // position. Hence  we increment x after successul printing of character on console
                write!(stdout, "{}{}", termion::cursor::Goto(x + 1, y), 
                       termion::clear::AfterCursor).unwrap(); 
                print!("{}", command_line);
                command.push(command_line);
                index = search_substring(history.clone(), command.clone(), index);
                x += 1;
            },
            Key::Backspace => {
                // Bring the cursor back to the original position, this will remove one character 
                // from the screen. The same is replicted by removing a character from the command variable
                write!(stdout, "{}{}", termion::cursor::Goto(x, y), 
                       termion::clear::AfterCursor).unwrap();
                if x > 1 {
                    x -= 1;
                    command = command[0..command.len() - 1].to_string();
                }
                
                // Index is brought back to 0 as the we start iterating the list from the tail again
                index = 0;
            },
            Key::Ctrl('c') => break,
            Key::Ctrl('r') => {
                // When we press ctrl 'r', rev_Search increments the index and let us check the further 
                // entries in the history list having the same substring
                write!(stdout, "{}{}", termion::cursor::Goto(x + 1, y), 
                       termion::clear::AfterCursor).unwrap();
                index += 1;
                index = search_substring(history.clone(), command.clone(), index);
            }
            _ => {
                
            }
        }
        stdout.flush().unwrap();
    }

    let (new_x, _new_y) = stdout.cursor_pos().unwrap();
    stdout.suspend_raw_mode().unwrap();

    if new_x == 1 {
        // When enter is pressed, we execute the command present in the list at the given index.
        // Before executing the command, we remove the console from the raw mode and then execute the
        // requested command.
        let command_to_execute = history[history.len() - 1 - index].clone();
        stdout.suspend_raw_mode().unwrap();
        history = util::add_command_to_history(history.clone(), 
                                               command_to_execute.clone());
        util::dispatch_function_helper(history.clone(), command_to_execute);
    }
    write!(stdout, "{}", termion::cursor::Show).unwrap();

    history
}