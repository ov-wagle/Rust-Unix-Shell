pub fn validate_command(user_command : String) -> bool {

    let command_history:String = String::from("cmd_history");
    let command_quit: String = String::from("quit");
    let command_ls: String = String::from("listDir"); //listDir
    let command_rev_search:String = String::from("rev_search");
    let command_cd:String = String::from("cd");
    let command_sort:String = String::from("sort");
    
     let mut command = user_command.trim().to_string();
     let command_out = user_command.trim().to_string();

     if command.contains(" >") {
        let cmd = &command_out;
        let vec: Vec<&str> = cmd.split(">").collect();

        if vec.len() == 2 {
            let path_to_output = vec[1].clone();

            let mut cmd1 = command_out.clone();
            cmd1 = cmd1.clone().replace(">", "");
            cmd1 = cmd1.clone().replace(path_to_output, "");

            if path_to_output.contains(">") {
                return false;
            }
            else if !path_to_output.contains(">") && path_to_output!="" {
                command = cmd1.clone().trim().to_string();
            }
            else {
                return false;
            }
        }
        else {
            return false;
        }
    }

    
    else if command.contains("<"){

        if command.starts_with("<") {

            return false;

        }
        else if command.ends_with("<"){
            return false;
        }
        else{
            let chunks: Vec<_> = command.split("<").collect();
            if chunks.len() != 2{
                return false;
            }
            else{
                return true;
            }
        }        
    }
    
     if command == command_history {
        return true;
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
        
        let path = vec[vec.len()-1];

        if !path.starts_with("-") && vec[vec.len()-1]!=&command_ls {
            vec.pop();
        }

        match vec[..] {
            [_ls] => { return true; },
            [_ls, a] => {
                match a {
                    "-tree" => { return true; }
                    "-a" => { return true; }
                    "-l" => { return true; }
                    _=> { return false; }
                }
            },
            [_ls, a,b] => {
                if (a == "-color" && b == "-l") || (b == "-color" && a == "-l") {
                    return true;
                }
                else  {
                    return false;
                }
            },
            _=> {
                return false;
            }
        }
        
     }
     //check if command starts with rmallexn
     else if command.starts_with("rmallexn") {
        return true;
     } 
     else if command == command_quit {
        return true;
     } 
     else if command == command_rev_search {
        return true;
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
        if vec_path.len() == 2 {
            return true;
        }
        else {
            return false;
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
            if option == "-n" {
                return true;
            }
            else if option == "-t" {
                if ext_name.starts_with(".") {
                    return true;
                } else {
                    return false;
                }
            }
            else {
                return false;
            }
        }
        else {
            return false;
        }
     }
     else {
        return false;
     }
}