use crate::util;
use crate::validator;

pub fn pipe(history:Vec<String>, command: String) {
    
    let command = command;

    if command.contains("|"){

        if command.starts_with("|") {

            println!("Invalid command: command starts with pipe, try modifying the command");

        }
        else if command.ends_with("|"){

            println!("Invalid command: command ends with pipe, try modifying the command");
        }
        else{
            
            let chunks: Vec<_> = command.split("|").collect();
            
            let mut flag = 0;

            for substring in chunks.iter(){

                let substring = substring.trim().to_string();

                if substring.chars().count() == 0{
                    
                    flag = 1;
                    break

                }

            }

            if flag == 1{

                println!("Invalid command");

            }
            else{

                let mut new_flag = true;
                //let validate = true;
                for check_command in chunks.iter(){

                    let check = check_command.trim().to_string();
                    
                    new_flag = validator::validate_command(check);

                    if new_flag == false{

                        break
                    }

                } 

                if new_flag == true{


                    for items in chunks.iter(){

                        let cmd = items.trim().to_string();
                        
                        util::dispatch_function_helper(history.clone(), cmd.clone());
                        
                    }
                }
                else{

                    println!("Invalid command");

                }

            }
        
        }        
    
    }

}
