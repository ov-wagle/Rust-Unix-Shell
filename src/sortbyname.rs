use std::fs::{self};
use std::path::{PathBuf, Path};
use std::io::ErrorKind;
use pbr::ProgressBar;
use std::{thread, time};

pub fn sort_by_name_main(file_type : &str, dest : String) {

    // Assigning basic information
    let user_path = String::from("./");
    let paths = fs::read_dir(user_path).unwrap();
    let mut count = 0;
    let mut flag = true;

    // Check for director exist or not
    let check: bool = Path::new(&dest.clone()).is_dir();
    if !check {
        fs::create_dir(&dest).unwrap_or_else(|error| {
            if error.kind() == ErrorKind::AlreadyExists {
                println!("Folder already exists! 😐");
                flag = false;
            } else {
                println!("Something went wrong! Please try again ...");
            }
        });
    }

    // Iteration over all files and folders in given directory
    paths.for_each(|initial| {
        let initial = initial.unwrap();
        let path = initial.path();
        if flag == true {
            count = move_files_with_name(path.clone(), &dest, &file_type, count);
        }
    });

    // Check for no matching condition
    if count == 0 && flag == true {
        println!("There are no matching files starting with given string! 😐");
        println!("Quitting... 🚫");

        // Removing a empty directory
        let test = fs_utils::check::is_folder_empty(&dest).unwrap();
        if test {
            fs::remove_dir(&dest).unwrap();
        }
        return;
    }
}

// Moving files
pub fn move_files_with_name(fn_path: PathBuf, dest: &str, file_type: &str, count: i32) -> i32{

        // Collecting metadata and file name
        let metadata = &fn_path.metadata().unwrap();
        let f_name = &fn_path.file_name().unwrap();
        let f_name_str = f_name.to_str().unwrap();
        let file_name = String::from(f_name_str);
        let mut temp = count;

        // Check if file starts with given string and it is a file
        if file_name.starts_with(&file_type) && metadata.is_file() {

            // Creating a destination path
            let destination = dest.to_owned() + &String::from("/") + &String::from(&file_name);

            // File copy
            match fs::copy(&file_name, &destination) {
                Ok(_) => {},
                Err(why) => {panic!("Error : Sort by Name {}", why)}
            };

            // File remove
            match fs::remove_file(&file_name) {
                Ok(_) => {},
                Err(why) =>{ panic!("Error : Sort by Name {}", why)}
            };  
            temp = temp + 1;

            // Progressbar implementation
            let t_count = 100;
            let mut pb = ProgressBar::new(t_count);
            pb.format("=>");
            for _ in 0..t_count {
                pb.inc();
                thread::sleep(time::Duration::from_millis(1));
            }
            pb.finish_print("Successfully Moved!");
        } 
    temp
}