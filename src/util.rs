use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use crate::ls;
use crate::ls_tree;
use crate::ls_color;
use crate::rev_search;
use crate::rmallexn;
use crate::sortbytype;
use crate::sortbyname;

use crate::pipe_operator;
use crate::attribute_background;
use crate::input_redirection;
extern crate json;

// Initialize the vector to store the list of commands entered on the shell
pub fn initialize_vector() -> Vec<String> {
    let vec = Vec::new();

    vec
}

// Add command to the history vector
pub fn add_command_to_history(mut history:Vec<String>, command:String) -> Vec<String> {
    history.push(command);

    history
}

// Print the commands present in the history vector
pub fn list_history(history:Vec<String>, save_output: bool, output_path : &str) -> Vec<String> {
    let mut data:String = "".to_owned();
    for i in 0..history.len() {
        println!("\t{0} {1}", i + 1, history[i]);
        data.push_str("\n");
        data.push_str(&(i+1).to_string());
        data.push_str("    ");
        data.push_str(&history[i]);
    }

    if save_output == true {
        let mut file = match File::create(output_path) {
            Err(why) => panic!("couldn't create  {}", why),
            Ok(file) => file,
        };
        match file.write_all(data.as_bytes()) {
            Err(why) => panic!("couldn't write to  {}", why),
            Ok(_) => println!("successfully wrote to {}",output_path),
        }
    }

    history
}

// Write the cmd_history to a file
pub fn write_results_in_file(key: String, value: Vec<String>) {
    let file_name = "log.txt";
    let path = Path::new(file_name);
    let display = path.display();

    // Check the existence of the file in the current directory. If exists, read the file, extract the 
    // JSON from the file and create a file with write only permission and write the updated data
    // into the file. If file does not exists, create the file and write the data into it.
    let exists = Path::new(file_name).exists();

    // https://doc.rust-lang.org/rust-by-example/std_misc/file/open.html
    if exists {
        let mut file = match File::options()
            .read(true)
            .write(true)
            .open(&path) {
                Err(why) => panic!("couldn't open {}: {}", display, why),
                Ok(file) => file,
            };

        let mut data = String::new();
        match file.read_to_string(&mut data) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => print!("")
        }

        let mut parsed_data = json::parse(&data).unwrap();
        parsed_data[key] = value.into();

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        match file.write_all(json::stringify(parsed_data).as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
    } else {
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        // https://docs.rs/json/latest/json/
        let mut data = json::JsonValue::new_object();
        data[key] = value.into();

        match file.write_all(json::stringify(data).as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => {println!("successfully wrote to {}", display)},
        }
    }
}

pub fn retrieve_history(mut history:Vec<String>) -> Vec<String> {
    let file_name = "log.txt";
    let path = Path::new(file_name);
    let display = path.display();

    // Check for file existence
    let exists = Path::new(file_name).exists();

    if exists {
        // Open the file with read permission
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        // Read the contents of the file and convert it into the String format
        let mut data = String::new();
        match file.read_to_string(&mut data) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => print!("")
        }

        // Convert the extracted data from String to JSON type and then extract the value
        // of key "cmd_history"
        let data = json::parse(&data).unwrap();
        let cmd_history = &data["cmd_history"];

        // Add every entry into the JSON array into the history vector using stringify.
        for i in 0..cmd_history.len() {
            history = add_command_to_history(history, json::stringify(cmd_history[i].clone()));
            // Rewrite every entries into the history vector such that double quotes ("") are not
            // copied into the history vector. e.g. "ls" is copied as ls into the vector
            history[i] = String::from(&history[i][1..history[i].len() - 1]);
        }
    }

    history
}

pub fn dispatch_function_helper(mut history:Vec<String>, user_command:String) -> Vec<String> {
    let command_history:String = String::from("cmd_history");
    let command_quit: String = String::from("quit");
    let command_ls: String = String::from("listDir"); //listDir
    let command_rev_search:String = String::from("rev_search");
    let command_cd:String = String::from("cd");
    let command_sort:String = String::from("sort");
    
     let mut command = user_command.trim().to_string();
     let command_out = user_command.trim().to_string();
     let mut save_output = false;
     let mut output_path = "";

    //  let is_valid = validator::validate_command(command.clone()); // implementation for validator 
    //  println!("Valid hai kya ? = {} ",is_valid); // returns true or false

     if command.contains("&") {
        attribute_background::background_execution(history.clone(), command.clone());
     } else if command.contains("|") {
        pipe_operator::pipe(history.clone(), command.clone());
     } else if command.contains("<") {
        input_redirection::redirection(history.clone(),command.clone());
     }
     else {
        if command.contains(" >") {
            let cmd = &command_out;
            let vec: Vec<&str> = cmd.split(">").collect();
            
            if vec.len() == 2 {
                let path_to_output = vec[1].clone();

                let mut cmd1 = command_out.clone();
                cmd1 = cmd1.clone().replace(">", "");
                cmd1 = cmd1.clone().replace(path_to_output, "");
                
                command = cmd1.clone().trim().to_string();

                if path_to_output.contains(">") {
                    command = "".to_string();
                }
                else if !path_to_output.contains(">") && path_to_output!="" {
                    save_output = true;
                    output_path = path_to_output.trim();
                }
                else {
                    command = "".to_string();
                }
            }
            else {
                command = "".to_string();
            }
        }

        if command == command_history {
            history = list_history(history,save_output,&output_path);
        }
        
        else if command.starts_with(&command_ls) {

            let g = command.split(" ");
            let mut vec: Vec<&str> = g.collect();
            let mut vec_indices = Vec::new(); // for removing extra spaces
            for i in 0..vec.len() { 
                if vec[i] == "" {
                    vec_indices.push(i);
                }
            }
            for i in vec_indices.iter().rev() {
                vec.remove(*i);
            }

            let mut path = vec[vec.len()-1];

            if path.starts_with("-") || path == &command_ls {
                path = "";
            }

            else if !path.starts_with("-") && vec[vec.len()-1]!=&command_ls {
                vec.pop();
            }
            
            if !Path::new(path).exists() && path != "" {
                println!("Error : Path does not exist -> {}", path);
            }
            else {
                match vec[..] {
                    [_ls] => {ls_tree::list_no_param(path.to_string() ,save_output,&output_path)},
                    [_ls, a] => {
                        match a {
                            "-tree" => { ls_tree::tree_display(path.to_string(),save_output,&output_path); }
                            "-a" => { ls_tree::list_all(path.to_string(),save_output,&output_path); }
                            "-l" => { ls::ls_main(path.to_string() ,save_output,&output_path); }
                            // "-color" => { println!(" color {}", a); }
                            _=> { println!("Error : Invalid Option {} ",a); }
                        }
                    },
                    [_ls, a,b] => {
                        if (a == "-color" && b == "-l") || (b == "-color" && a == "-l") {
                            ls_color::ls_color_main(path.to_string(),save_output,&output_path);
                        }
                        else  {
                            println!("Error : Invalid Option");
                            println!("Correct Usage : listDir [-l] [-a] [-tree] [-color] <directory>");
                        }
                    },
                    _=> {
                        println!("Error : Invalid number of arguments");
                    }
                }
            }
            
        }
        //check if command starts with rmallexn
        else if command.starts_with("rmallexn") {
            rmallexn::rmxn(command.clone());
        } 

        else if command == command_quit {
            println!("Quitting");
            write_results_in_file(command_history, history.clone());
            return Vec::<String>::new();
        } 
        
        else if command == command_rev_search {
            history = rev_search::rev_search(history);
        } 
        
        else if command.starts_with(&command_cd) {
            let mut vec_path: Vec<&str> = command.split(" ").collect();

            let mut vec_indices = Vec::new(); // for removing extra spaces
            for i in 0..vec_path.len() { 
                if vec_path[i] == "" {
                    vec_indices.push(i);
                }
            }
            for i in vec_indices.iter().rev() {
                vec_path.remove(*i);
            }

            if vec_path.len() > 1 {
                let path = vec_path[1];  
                let cur_dir = std::env::current_dir();
                let mut cur_dir_path : String = "".to_string();
                match cur_dir {
                    Ok(_) => {
                        cur_dir_path = cur_dir.unwrap().into_os_string().into_string().unwrap();
                    },
                    Err(why)=> println!("Error : In getting path {}",why),
                }

                let mut cur_path: Vec<&str> = cur_dir_path.split("Rust-Unix-Shell").collect();
                if !cur_dir_path.contains("Rust-Unix-Shell") {
                    cur_path = cur_dir_path.split("/home").collect();
                }
                
                if cur_path.len() > 1 && path.contains("..") { // Check if cuurent directory is Rust-Unix-Shell prevent back
                
                    if cur_path[1] != "" {  
                        if Path::new(path.trim()).exists() {
                            match std::env::set_current_dir(path) {
                                Ok(_) => {},
                                Err(why) => println!("Error in cd {}", why),
                            }
                        }
                        else {
                            println!("Error : No such directory")
                        }
                    }
                }
                else if cur_path.len() == 2 {
                    if Path::new(path.trim()).exists() {
                        match std::env::set_current_dir(path) {
                            Ok(_) => {},
                            Err(why) => println!("Error in cd {}", why),
                        }
                    }
                    else {
                        println!("Error : No such directory")
                    }
                }
            }
        }

        else if command.starts_with(&command_sort) {
            let g = command.split(" ");
            let mut vec: Vec<&str> = g.collect();

            let mut vec_indices = Vec::new(); // for removing extra spaces
            for i in 0..vec.len() { 
                if vec[i] == "" {
                    vec_indices.push(i);
                }
            }
            for i in vec_indices.iter().rev() {
                vec.remove(*i);
            }

            if vec.len() == 4 {
                let option = vec[1];
                let ext_name = vec[2];
                let directory = vec[3].clone();
                if option == "-n" {
                    sortbyname::sort_by_name_main(ext_name, directory.to_string());
                }
                else if option == "-t" {
                    if ext_name.starts_with(".") {
                        sortbytype::sort_by_type_main(ext_name, directory.to_string());
                    } else {
                        println!("Error : Invalid extention");
                        println!("Correct Usage : sort -t .ext <directory_name>");
                    }
                }
                else {
                    println!("Error : Invalid Option");
                    println!("Correct Usage : sort [-t] [-n] .ext/name <directory_name>");
                }
            }
            else {
                println!("Error : Invalid Number or Arguments in Sort");
                println!("Correct Usage : sort [-t] [-n] .ext/name <directory_name>");
            }

        }


        else {
            println!("Invalid command");
        }
    
    }

     history
}
