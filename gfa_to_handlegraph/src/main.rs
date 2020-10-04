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

/// a function to print the result of the function read_directory_files
/// (october 4, 2020)
fn print_dir(dircontent: Vec<String>) {
    if dircontent.is_empty() {
        println!("The directory is empty");
    } else {
        for file in dircontent {
            println!("{}", file);
        }
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
fn read_directory_files(dirname: &str) -> Vec<String> {
    // declare a Vec<String> variable to store the results
    let mut files:Vec<String> = vec![];

    // Search through the entire directory with a loop
    // nested directory will be returned as if they are plain file
    for e in glob(dirname).unwrap() {
        match e {
            // if match gives a positive result, that means the directory exist,
            // go and store the name of the file found in it
            Ok(path) => {
                // declare a variable to store the name of the file found over each iteration
                let filename = path.display().to_string();
                // insert the name of the file into the Vec<String> variable "files"
                files.push(filename)
            },
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
    files
}

/// function help that inform the user on how the program works and how to use it in the right way
/// (october 4, 2020)
fn help() {
    println!("The point of this project is to implement the rs-handlegraph interface to handling \
    genomic graphs that are being represented via the library rs-gfa.\n\
    Both of the libraries are developed by Christian Fischer and are available on GitHub\n");

    println!("To run properly this application via command line you should do as follow:\n\
    1) run the application with ./application_name argument1 argument2\n\
    2) argument1 is referred to what kind of operation you are intended to do\n\
    3) argument2 is referred to what kind of input, based on the operation you wanna perform, you \
    wanna pass to the program\n");

    println!("The operation you can perform now are:\
    1) with the command rf: read the content of a file \n\
    2) with the command rd: display the content of a directory\n\
    3) with the command rdf: display the content of a directory and read the content of each valid file in it\n");

    println!("The input you can pass to the application is either:\n\
    1) a path to a file\n\
    2) a path to a directory\n")
}

/// implement various test function
/// (october 4, 2020)
#[cfg(test)]
mod tests{
    use super::*;

    /// implement test function for help
    #[test]
    fn test_help() {
        help();
    }

    /// implement test function for read_file passing an existing file as input
    #[test]
    fn read_existing_file() {
        println!("Test function \"read_file\" passing an existing file as input\n");
        let temp_filepath = "test/gfas/gfa1_files/example1.gfa";
        print(read_file(temp_filepath));
    }

    /// implement test function for read_file passing an existing empty file as input
    #[test]
    fn read_empty_file() {
        println!("Test function \"read_file\" passing an existing empty file as input\n");
        let temp_filepath = "test/gfas/gfa2_files/blankDocument.gfa";
        print(read_file(temp_filepath));
    }

    /// implement test function for read_file passing an existing file >= 1MB as input
    #[test]
    fn read_big_file() {
        println!("Test function \"read_file\" passing an existing file >= 1MB as input\n");
        let temp_filepath = "test/gfas/big_file/very_big_file.gfa";
        print(read_file(temp_filepath));
    }

    /// implement test function for read_file passing a non-existing file as input
    #[test]
    fn read_non_existing_file() {
        println!("Test function \"read_file\" passing a non-existing file as input\n");
        let temp_filepath = "test/gfas/i_dont_exist.gfa";
        print(read_file(temp_filepath));
    }

    /// implement test function for read_file passing a directory instead of a file as input
    #[test]
    fn read_directory_as_file() {
        println!("Test function \"read_file\" passing a directory instead of a file as input\n");
        let temp_dirpath = "test/gfas";
        print(read_file(temp_dirpath));
    }

    /// implement test function for read_directory_files passing an existing directory as input
    #[test]
    fn scan_existing_directory() {
        println!("Test function \"read_directory_files\" passing an existing directory as input");
        let temp_dir = "test/gfas/*";
        println!("The directory {} contains the files:\n ", temp_dir);
        print_dir(read_directory_files(temp_dir));
    }

    /// implement test function for read_directory_files passing an existing directory as input
    /// testing the search for .gfa files in all the directory and subdirectories
    #[test]
    fn scan_existing_directory_recursively() {
        println!("Test function \"read_directory_files\" passing an existing directory as input");
        let temp_dir = "test/gfas/**/*.gfa";
        println!("The directory {} and all its subdirectories, contain the files:\n ", temp_dir);
        print_dir(read_directory_files(temp_dir));
    }

    /// implement test function for read_directory_files passing a non-existing directory as input
    #[test]
    fn scan_non_existing_directory() {
        println!("Test function \"read_directory_files\" passing a non-existing directory as input");
        let temp_dir = "test/i_dont_exist/*";
        println!("The directory {} contains the files:\n ", temp_dir);
        print_dir(read_directory_files(temp_dir));
    }
}

fn main() {

    // REMEMBER! from path/project_folder cargo build to build a rust project
    //          from path/project_folder cargo run with_optional_argument to run a rust application

    /// read the arguments passed as command line parameters
    /// (october 4, 2020)
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

                    if last_char == '/' {
                        arg.push('*');
                    } else {
                        arg.push_str("/*");
                    }

                    print_dir(read_directory_files(&*arg))
                },

                "rdf" => {

                    let last_char = argument.chars().last().unwrap();
                    let mut arg = argument.clone();

                    if last_char == '/' {
                        arg.push('*');
                    } else{
                        arg.push_str("/*");
                    }

                    print_dir(read_directory_files(&*arg));

                    let files = read_directory_files(&*arg);
                    for file in files {
                        print(read_file(&*file));
                    }
                },

                _ => {
                    eprintln!("Error! Invalid command\n");
                    help();
                },
            }
        },
        // if the number of argument passed as command line parameters is not proper
        // the program will display an error message to inform the user
        _ => {
            eprintln!("Error! The number of argument passed is not correct to run the program properly!\n");
            help();
            return;
        },
    }
}
