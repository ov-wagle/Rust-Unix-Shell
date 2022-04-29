
use std::fs;
use std::path::Path;
use std::error::Error;
use std::fs::metadata;
use std::fs::File;
use std::io::Write;
use std::fs::OpenOptions;

pub fn tree_display(path : String ,save_output: bool, output_path : &str) {
    let mut dir_path = path;
    if dir_path == "" {
        dir_path = String::from("./");
    }
    if save_output == false  {
        println!("{}",dir_path);
    }

    if save_output == true {
        let _file = match File::create(output_path) {
            Err(why) => panic!("couldn't create  {}", why),
            Ok(file) => file,
        };
    }

	if let Err(ref e) = run(Path::new(&dir_path),0,Vec::new(),&dir_path, save_output , &output_path) {
		println!("{:?}", e);
        return;
	}
    
    if save_output == false  {
        println!("");
    }
}

fn run(dir: &Path, mut level : usize,  mut vec : Vec<String>, dir_path:&String, save_output: bool, output_path : &str) -> Result<(), Box<dyn Error>> {

	if dir.is_dir() {
        level = level + 1;

        // Source Url https://endler.dev/2018/ls/
		for entry in fs::read_dir(dir)? {
				let entry = entry?;
				let file_name = entry.file_name().into_string().or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;
				if file_name.chars().nth(0).unwrap() != '.' {

                    // level = depth from directory

                    if save_output == true {
                        let mut file = OpenOptions::new()
                            .write(true)
                            .append(true)
                            .open(output_path)
                            .unwrap();
                        for _i in 1..level {
                            if let Err(e) = write!(file, "    ") {
                                eprintln!("Couldn't write to file: {}", e);
                            }
                        }
                        if let Err(e) = write!(file, "|") {
                            eprintln!("Couldn't write to file: {}", e);
                        }

                        if let Err(e) = writeln!(file, "__{}", &file_name.clone()) {
                            eprintln!("Couldn't write to file: {}", e);
                        }

                    }

                    else {

                        for _i in 1..level {
                            print!("    ");
                        }
                        print!("|");
                    
                        println!("__{}", file_name);    
                    }

                    let s1 = "/".to_string();
                    let s2 = file_name.clone();
                    let mut s3 = String::new();
                    let mut s4 = String::new();
                    s3 += &s2;
                    s3 += &s1;
                    vec.push(s3.clone());
                    s4+=dir_path;
                    s4+="/";
                    for i in vec.iter() {
                        s4 += i;
                    }

                    if let Err(ref e) = run(Path::new(&s4),level,vec.clone(),dir_path, save_output, &output_path) {
                            println!("{:?}", e);
                            break;
                    }
                    vec.pop();
                }
		}
	}

	Ok(())
    
}

pub fn list_all(path : String ,save_output: bool, output_path : &str) {
    let mut dir_path = path;
    if dir_path == "" {
        dir_path = String::from("./");
    }


    let md = metadata(&dir_path).unwrap();
    if md.is_file() {
        println!("{}",dir_path);
    }
    
    else {
        
        let mut disp_vec : Vec<String> = Vec::new();

        disp_vec.push(".".to_string());
        disp_vec.push("..".to_string());

        if let Err(ref e) = run_all(Path::new(&dir_path),&mut disp_vec) {
            println!("{}", e);
            return;
        }
        let mut max = 0;
        let mut screen_max = 5;
        for i in disp_vec.iter() {
            if i.len() > max {
                max = i.len();
            }
        }
        if max>17 {
            screen_max = 3;
        }

        let mut data : String = "".to_owned();
        let mut count = 0;
        for i in disp_vec.iter() {
            if save_output == false  {
                print!("{}",i);
            }
            data.push_str(i);
            count+=1;
            if i.len() < max {
                for _k in i.len()..max {
                    if save_output == false  {
                        print!(" ");
                    }
                    data.push_str(" ");
                }
            }
            if save_output == false  {
                print!("\t");
            }
            data.push_str("\t");

            if count%screen_max == 0 {
                if save_output == false  {
                    println!("");
                }
                data.push_str("\n");
            }
        }

        if save_output == false  {
            println!("");
        }
        
        if save_output == true {
            let mut file = match File::create(output_path) {
                Err(why) => panic!("couldn't create  {}", why),
                Ok(file) => file,
            };
            match file.write_all(data.as_bytes()) {
                Err(why) => panic!("couldn't write to  {}", why),
                Ok(_) => {},
            }
        }
        
    }
}

fn run_all(dir: &Path, vec : &mut Vec<String>) -> Result<(), Box<dyn Error>> {
	if dir.is_dir() {
        // Source Url https://endler.dev/2018/ls/
		for entry in fs::read_dir(dir)? {
				let entry = entry?;
				let file_name = entry.file_name().into_string().or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;
                vec.push(file_name);
		}
	}
	Ok(())
}

pub fn list_no_param(path : String ,save_output: bool, output_path : &str) {
    let mut dir_path = path;
    if dir_path == "" {
        dir_path = String::from("./");
    }

    let md = metadata(&dir_path).unwrap();
    if md.is_file() {
        println!("{}",dir_path);
    }

    else {
        let mut disp_vec : Vec<String> = Vec::new();

        // Source Url https://endler.dev/2018/ls/
        if let Err(ref e) = run_no_params(Path::new(&dir_path),&mut disp_vec) {
            println!("{}", e);
            return;
        }
        let mut max = 0;
        let mut screen_max = 5;
        for i in disp_vec.iter() {
            if i.len() > max {
                max = i.len();
            }
        }
        if max>17 {
            screen_max = 3;
        }
        let mut count = 0;
        let mut data : String = "".to_owned();

        for i in disp_vec.iter() {
            if save_output == false  {
                print!("{}",i);
            }

            data.push_str(i);

            count+=1;
            if i.len() < max {
                for _k in i.len()..max {
                    if save_output == false  {
                        print!(" ");
                    }
                    data.push_str(" ");
                }
            }
            if save_output == false  {
                print!("\t");
            }
            data.push_str("\t");
            if count%screen_max == 0 {
                if save_output == false  {
                    println!("");
                }
                data.push_str("\n");
            }
        }

        if save_output == true {
            let mut file = match File::create(output_path) {
                Err(why) => panic!("couldn't create  {}", why),
                Ok(file) => file,
            };
            match file.write_all(data.as_bytes()) {
                Err(why) => panic!("couldn't write to  {}", why),
                Ok(_) => {},
            }
        }
        if save_output == false  {
            println!("");
        }
    }
}

fn run_no_params(dir: &Path, vec : &mut Vec<String>) -> Result<(), Box<dyn Error>> {
	if dir.is_dir() {
        // Source Url https://endler.dev/2018/ls/
		for entry in fs::read_dir(dir)? {
				let entry = entry?;
				let file_name = entry.file_name().into_string().or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;
                if !file_name.starts_with(".") {
                    vec.push(file_name);
                }
		}
	}
	Ok(())
}