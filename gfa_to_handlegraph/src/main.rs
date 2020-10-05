/// The point of this project is to implement the rs-handlegraph interface to handling
/// genomic graphs that are being represented via the library rs-gfa
/// Both of the libraries are developed by Christian Fischer and are available on GitHub

/// external libraries from github
use handlegraph;
use gfa;

/// library for handling the CLI input
extern crate clap;
use clap::{App, Arg};

/// libraries for handling files (R/W) and directories (relative and absolute path)
/// (october 3, 2020)
use std::fs::File;
use std::io::prelude::*;
/// library to access the arguments passed as command line parameters
/// (october 4, 2020)
use std::{io, env};
extern crate glob;
use glob::glob;
/// library to access the path of a file
use std::path::Path;

// (october 4, 2020)
// TODO: find a way to display either the content (or the error) and the name of the file
/// function to print the result of the function read_file
/// the result could be either the content of a file or an error message
/// (october 3, 2020)
fn print(result: Result<String, io::Error>) {
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
fn read_file(filename: &str) -> Result<String, io::Error> {
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
fn check_file_exist(result: Result<bool, io::Error>) {
    match result {
        Ok(_) => println!("File created successfully!"),
        Err(why) => println!("Error: {}", why),
    }
}

/// a function that create a file and then write a string of information in it
/// (october 5, 2020)
fn write_file(filename: &str, content: &str) -> Result<bool, io::Error> {
    let path = Path::new(filename);

    // try to open a file in write-only mode, returns io::Result<File>
    let mut file = File::create(&path)?; 

    // write in file the string content passed as input parameter
    file.write_all(content.as_bytes())?;

    Ok(file.metadata().unwrap().is_file())
}

/// a function to print the result of the function read_directory_files
/// (october 4, 2020)
fn print_dir(dircontent: Result<Vec<String>, io::Error>) {
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
fn read_directory_files(dirname: &str) -> Result<Vec<String>, io::Error> {
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

/// function help that inform the user on how the program works and how to use it in the right way
/// (october 4, 2020)
fn help(otpional_parameters: Option<usize>) {
    // the only way to implement optional parameters and default parameter in a Rust program
    // is to define an Option<usize> parameter in the target function
    // this because Rust DOES NOT support default function argument
    // (october 5, 2020)
    if let Some(20) = otpional_parameters {
        // print the help message relative to the error of BAD_OPERATION
        // (october 5, 2020)
        println!("The operation you can perform now are:\n\
        1) with the command rf: read the content of a file \n\
        2) with the command rd: display the content of a directory\n\
        3) with the command rdf: display the content of a directory and read the content of each valid file in it\n");    
    } else {
        // print a generic help message
        // (october 5, 2020)
        println!("To run properly this application via command line you should do as follow:\n\
        1) run the application with ./application_name argument1 argument2\n\
        2) argument1 is referred to what kind of operation you are intended to do\n\
        3) argument2 is referred to what kind of input, based on the operation you wanna perform, you \
        wanna pass to the program\n");

        println!("The operation you can perform now are:\n\
        1) with the command rf: read the content of a file \n\
        2) with the command rd: display the content of a directory\n\
        3) with the command rdf: display the content of a directory and read the content of each valid file in it\n");

        println!("The input you can pass to the application is either:\n\
        1) a path to a file\n\
        2) a path to a directory\n")
    }
}

/// separated the test.rs file from the main.rs file so the code can remain clean and readable
/// (october 5, 2020)
#[cfg(test)]
mod test;

fn main() {

    // REMEMBER! from path/project_folder cargo build to build a rust project
    //          from path/project_folder cargo run with_optional_argument to run a rust application

    // read the arguments passed as command line parameters
    // (october 4, 2020)
    let args: Vec<String> = env::args().collect();

    // check the length of the argument passed as command line parameters
    match args.len() {
        // if passed one command and one argument then the application can starts smoothly
        3 => {
            // save the arguments extract from the command line in 2 different categories
            // command and argument
            let command = &args[1];
            let argument = &args[2];

            // parse the command
            match &command[..] {
                "rf" => print(read_file(argument)),

                "rd" => {
                    // warning!
                    // use unwrap() only if sure that argument has at least 1 character
                    let last_char = argument.chars().last().unwrap();

                    // check the last character of the directory path is *
                    // otherwise the read_directory_file will not run properly and instead of
                    // display the entire body of the directory, it will return only the path
                    // insert as input
                    let mut arg = argument.clone();

                    // changed control statement from if-else to match
                    // (october 5, 2020) 
                    match last_char {
                        '/' => arg.push('*'),
                        _ => arg.push_str("/*"),
                    }
                  
                    print_dir(read_directory_files(&*arg))
                },

                "rdf" => {

                    let last_char = argument.chars().last().unwrap();
                    let mut arg = argument.clone();

                    // changed control statement from if-else to match
                    // (october 5, 2020) 
                    match last_char {
                        '/' => arg.push('*'),
                        _ => arg.push_str("/*"),
                    }

                    print_dir(read_directory_files(&*arg));

                    let dir = read_directory_files(&*arg);
                    for files in dir {
                        for file in files {
                            print(read_file(&file));
                        }
                    }
                },

                _ => {
                    eprintln!("\nError! Invalid command\n");
                    help(Some(20));
                },
            }
        },
        // if the number of argument passed as command line parameters is not proper
        // the program will display an error message to inform the user
        _ => {
            eprintln!("\nError! The number of argument passed is not correct to run the program properly!\n");
            help(Some(0));
            return;
        },
    }
}
