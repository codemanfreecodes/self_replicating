use std::fs::File;
use std::io::prelude::*;
use std::env;

fn main() {
    // Get the name of the current executable.
    let executable_name = env::current_exe().expect("Failed to get executable name");

    print!("of type PathBuf:{:?} ",executable_name);

    

    // Open the current executable file.
    let mut file = File::open(&executable_name).expect("Failed to open file");

    // Read the entire content of the executable into a Vec<u8>.
    let mut code = Vec::new();
    file.read_to_end(&mut code).expect("Failed to read file");
    
    


    // Define a new filename for the copied executable.
    let new_executable_name = "copy_of_executable";

    // Create a new file with the new name.
    let mut new_file = File::create(new_executable_name).expect("Failed to create file");

    // Write the content of the Vec<u8> (code) to the new file.
    new_file.write_all(&code).expect("Failed to write file");

    println!("Successfully copied executable to {}", new_executable_name);

    
}
