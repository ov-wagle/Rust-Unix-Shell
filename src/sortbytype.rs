use std::fs::{self};
use std::path::{PathBuf, Path};
use std::io::ErrorKind;
use pbr::ProgressBar;
use std::{thread, time};

pub fn sort_by_type_main(file_type : &str, dest : String) {

    // Assigning variables for basic functionalities
    let user_path = String::from("./");
    let paths = fs::read_dir(user_path).unwrap();
    let mut count = 0;
    let mut flag = true;

    // Checking if folder already exists or not, if not then create a new folder
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


    // Iterating over all files and folders in given directory
    paths.for_each(|initial| {
        let initial = initial.unwrap();
        let path = initial.path();

        // Calling move_files function only if file folder exists or it's successfully created
        if flag == true {
            count = move_files(path.clone(), &dest, &file_type, count);
        }
    });

    // Check if there are no files matching with given extension
    if count == 0 && flag == true {
        println!("There are no matching files with given extension 😐");
        println!("Quitting... 🚫");

        // Checking whether it is a empty directory and if yes then deleting it
        let test = fs_utils::check::is_folder_empty(&dest).unwrap();
        if test {
            fs::remove_dir(&dest).unwrap();
        }
        return;
    }
}

// Actual move files function
pub fn move_files(fn_path: PathBuf, dest: &str, file_type: &str, count: i32) -> i32{

        // Collecting metadata and file name
        let metadata = &fn_path.metadata().unwrap();
        let f_name = fn_path.file_name().unwrap();
        let f_name_str = f_name.to_str().unwrap();
        let file_name = String::from(f_name_str);
        let mut temp = count;

        // If file name ends with extension and making sure it is a file
        if file_name.ends_with(&file_type) && metadata.is_file() {

            // Preparing a destination path
            let destination = dest.to_owned() + &String::from("/") + &String::from(&file_name);

            // File copy operation
            match fs::copy(&file_name, &destination) {
                Ok(_) => {},
                Err(why) => {panic!("Error : Sort by Name {}", why)}
            };

            // File remove operation i.e (copy + remove = move)
            match fs::remove_file(&file_name) {
                Ok(_) => {},
                Err(why) =>{ panic!("Error : Sort by Name {}", why)}
            };

            temp = temp + 1;

            // For printing progress bar
            let t_count = 100;
            let mut pb = ProgressBar::new(t_count);
            pb.format("#");
            for _ in 0..t_count {
                pb.inc();
                thread::sleep(time::Duration::from_millis(1));
            }
            pb.finish_print("Successfully Moved!");
        } 
    temp
}