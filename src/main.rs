mod util;
mod ls_tree;
mod ls;
mod ls_color;
mod rmallexn;
mod rev_search;
mod sortbyname;
mod sortbytype;
mod pipe_operator;
mod attribute_background;
mod validator;
mod input_redirection;
use std::io::Write;

fn main() {
    let mut history = util::initialize_vector();
    let mut command:String = String::new();

    // Retrieve the history commands if any before starting the shell
    history = util::retrieve_history(history);

    loop {

        let cur_dir = std::env::current_dir();
        let mut cur_dir_path : String = "".to_string();
        match cur_dir {
            Ok(_) => {
                cur_dir_path = cur_dir.unwrap().into_os_string().into_string().unwrap();
            },
            Err(why)=> println!("Error : In getting path {}",why),
        }

        let mut vec_path: Vec<&str> = cur_dir_path.split("Rust-Unix-Shell").collect();
        if !cur_dir_path.contains("Rust-Unix-Shell") {
            vec_path = cur_dir_path.split("/home").collect();
        }
        print!("rustshell@rustshell:~");
        if vec_path.len() > 1 {
            if vec_path[1] != "" {
                print!("{}", vec_path[1]);
            }
        }
        print!{"$ "}

        // Flushes the output to stdout as prints without new line are buffered and we have 
        // to explicitly flush the buffer.
        std::io::stdout().flush().unwrap();

        // Reads the input from the command line
        std::io::stdin().read_line(&mut command).unwrap();

        // Reading input from command line adds a new line character at the end.
        // Copying everything back to command except the new line character to match 
        // the strings later with the commands.
        command = String::from(&command[..command.len() - 1]);

        // If no input is entered and only return is pressed, it does not track those inputs
        if command.len() == 0 {
            command.clear();
            continue;
        }

        // Every command whether valid or invalid is added to the history list
        history = util::add_command_to_history(history.clone(), command.clone());

        history = util::dispatch_function_helper(history.clone(), command.clone());

        if history.len() == 0 {
            break;
        }

        command.clear();
    }
}