use crate::util;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use rustc_serialize::json::Json;
extern crate json;


pub fn redirection(history:Vec<String>, command: String) {
    
    let command = command;

    if command.contains("<"){

        if command.starts_with("<") {

            println!("Invalid command: command starts with <, try modifying the command");

        }
        else if command.ends_with("<"){

            println!("Invalid command: command ends with <, try modifying the command");
        }
        else{
            
            let chunks: Vec<_> = command.split("<").collect();
            
            if chunks.len() != 2{

                println!("Invalid command");

            }
            else{

                let mut input_command = chunks[0].trim().to_string();
                let input_file_name = chunks[1].trim().to_string();
                
                if input_file_name.ends_with(".json"){


                    if Path::new(&input_file_name).exists(){

                        let mut file = File::open(input_file_name).unwrap();
                        let mut data = String::new();
                        file.read_to_string(&mut data).unwrap();

                        let json = Json::from_str(&data).unwrap();
                        
                        let mut flag = false;

                        match json.find_path(&[&input_command]) {
                            Some(_value) => {
                                flag = true
                            },
                            None =>{}
                        };

                        if flag == true{
                            let value = json.find_path(&[&input_command]).unwrap();

                            let space: &str = " ";
                            input_command.push_str(space);
                            input_command.push_str(value.as_string().unwrap());

                            util::dispatch_function_helper(history.clone(), input_command.clone());
                        }

                        else{

                            println!("command input doesn't exist in file");

                        }

                    }
                    else{

                        println!("file doesn't exist");
                    }
                }
                else{

                    println!("file is not a json type");

                }

            }
        
        }        
    
    }

}
