/// libraries for handling files (R/W) and directories (relative and absolute path)
/// (october 3, 2020)
use std::fs::File;
use std::io::prelude::*;
/// library to access the arguments passed as command line parameters
/// (october 4, 2020)
use std::io;
extern crate glob;
use glob::glob;
/// library to access the path of a file
use std::path::Path;

// (october 4, 2020)
// TODO: find a way to display either the content (or the error) and the name of the file
/// function to print the result of the function read_file
/// the result could be either the content of a file or an error message
/// (october 3, 2020)
pub fn print(result: Result<String, io::Error>) {
    // based on the input variable 'result', println! will print a different message
    match result {
        Ok(file) => {
            if file.is_empty(){
                println!("The file is empty")
            } else {
                println!("File content:\n{}\n", file)
            }
        },
        Err(why) => println!("Error: {}\n", why),
    }
}

/// a function that read the content of a given file passed as input
/// (october 3, 2020)
pub fn read_file(filename: &str) -> Result<String, io::Error> {
    // try to open a file from its path passed as input 'filename'
    let mut file = File::open(filename)?;

    // if the file exists and can be open, try to read its content
    // and save it as a String
    let mut content = String::new();

    // check if the size of a file is smaller than 1MB (1_000_000)
    // if it is then I read the content of the file and save it as a string
    // otherwise I skip it and display a message
    // (october 4, 2020)
    if file.metadata().unwrap().len() >= 1_000_000u64 {
        content = "The file you are about to read is too big to display (>= 1MB)".parse().unwrap();
    } else {
        file.read_to_string(&mut content)?;
    }

    // return the content of the file
    Ok(content)
}

/// a function to control if a the file created by write_file exist or not
/// (october 5, 2020) 
pub fn print_file_result(result: Result<bool, io::Error>) {
    match result {
        Ok(_) => println!("File created and wrote successfully!"),
        Err(why) => println!("Error: {}", why),
    }
}

/// a function that create a file and then write a string of information in it
/// (october 5, 2020)
pub fn write_file(filename: &str, content: &str) -> Result<bool, io::Error> {
    let path = Path::new(filename);

    // try to open a file in write-only mode, returns io::Result<File>
    let mut file = File::create(&path)?; 

    // write in file the string content passed as input parameter
    file.write_all(content.as_bytes())?;

    Ok(file.metadata().unwrap().is_file())
}

/// a function to print the result of the function read_directory_files
/// (october 4, 2020)
pub fn print_dir(dircontent: Result<Vec<String>, io::Error>) {
    match dircontent{
        Ok(dir) => {
            if dir.is_empty() {
                println!("The directory is empty")
            } else {
                for file in dir {
                    println!("{}", file)
                }
            }
        },
        Err(why) => println!("Error: {}\n", why),
    }  
}

//  (october 4, 2020)
// TODO: handle and display the errors properly!
/// function that read and save in a Vec<String> the content of a directory passed as input
/// example: foo -
///            |- fooFile.txt
///            |- fooFile2.txt
/// the function will return: result = ["fooFile.txt", "fooFile2.txt"]
/// (october 3, 2020)
pub fn read_directory_files(dirname: &str) -> Result<Vec<String>, io::Error> {
    // declare a Vec<String> variable to store the results
    let mut files:Vec<String> = vec![];

    // Search through the entire directory with a loop
    // nested directory will be returned as if they are plain file
    for e in glob(dirname).expect("Failed to read glob pattern") {
        match e {
            // if match gives a positive result, that means the directory exist,
            // go and store the name of the file (or sub-directory) found in it
            // insert the name of the file (or sub-directory) into the Vec<String> variable "files"
            Ok(path) => files.push(path.display().to_string()),
                 
            // this branch of the match statement seems to not working properly.
            // It should catch the error if a non-existing directory is passed as input and then
            // display an error message, but the error message it's never displayed
            // idkw
            Err(why) => println!("Error: {}", why),
        }
    }
    // return the vector containing the result produced by the function
    // a return expression DO NOT want the character ";" at the end of it
    // otherwise the function will return its default value ()
    Ok(files)
}