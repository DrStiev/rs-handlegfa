/// The point of this project is to implement the rs-handlegraph interface to handling
/// genomic graphs that are being represented via the library rs-gfa
/// Both of the libraries are developed by Christian Fischer and are available on GitHub

/// external libraries from github
use handlegraph;
use gfa;

/// library for handling the CLI input
#[macro_use]
extern crate clap;
/// library to access the arguments passed as command line parameters
/// (october 4, 2020)
use std::env;

/// separated the main.rs file from the file that handles the operation over a file
/// this improves the readability of the code (imho)
/// (october 5, 2020)
/// import the file where the functions are defined
mod file_operation;
use file_operation::*;

/// separated the test.rs file from the main.rs file so the code can remain clean and readable
/// (october 5, 2020)
#[cfg(test)]
mod test;

fn main() {

    // define a default operation
    static DEFAULT_OPERATION: &str = "rf";
    // REMEMBER! ./target/debug/gfa_to_handlegraph --help
    // USAGE: gfa_to_handlegraph.exe [OPTIONS] <INPUT> [SUBCOMMAND]

    // use clap as a macro to handle the argument passed as parameters via command line
    // (october 5, 2020)
    let matches = clap_app!(gfa_to_handlegraph =>
        (version: "1.0")
        (author: "Stievano Matteo <m.stievano1@campus.unimib.it>")
        (about: "The point of this project is to implement the rs-handlegraph interface to handling\n\
                genomic graphs that are being represented via the library rs-gfa.\n\
                Both of the libraries are developed by Christian Fischer and are available on GitHub\n")
        (@arg INPUT: +required "Set the input file to use")
        (@arg OPERATION: -o --operation +takes_value "Sets a custom operation to do with the file\n\
                > rf: read file\n> rd: read directory\n> rdf: read all files from directory")
        (@subcommand test => 
            (about: "Controls testing features")
            (version: "1.0")
            (author: "Stievano Matteo <m.stievano1@campus.unimib.it>")
            (@arg verbose: -v --verbose "Print test information verbosely"))).get_matches();

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    let input_file = matches.value_of("INPUT").unwrap();

    // Gets a value for operation if supplied by user, or use the default one
    match matches.value_of("OPERATION").unwrap_or(DEFAULT_OPERATION) {
        "rf" => print(read_file(input_file)),

        "rd" => {
            // warning!
            // use unwrap() only if sure that argument has at least 1 character
            let last_char = input_file.chars().last().unwrap();
            
            // check the last character of the directory path is *
            // otherwise the read_directory_file will not run properly and instead of
            // display the entire body of the directory, it will return only the path
            // insert as input
            let dir: String;
            // changed control statement from if-else to match
            // (october 5, 2020) 
            match last_char {
                '/' => dir = format!("{}{}",input_file.clone(), "*"),
                _ => dir = format!("{}{}",input_file.clone(), "/*"),
            }
            
            print_dir(read_directory_files(&dir))
        },

        "rdf" => {
            // warning!
            // use unwrap() only if sure that argument has at least 1 character
            let last_char = input_file.chars().last().unwrap();
            
            // check the last character of the directory path is *
            // otherwise the read_directory_file will not run properly and instead of
            // display the entire body of the directory, it will return only the path
            // insert as input
            let dir: String;
            // changed control statement from if-else to match
            // (october 5, 2020) 
            match last_char {
                '/' => dir = format!("{}{}",input_file.clone(), "*"),
                _ => dir = format!("{}{}",input_file.clone(), "/*"),
            }
            
            print_dir(read_directory_files(&dir));

            let dirpath = read_directory_files(&dir);
            for files in dirpath {
                for file in files {
                    print(read_file(&file))
                }
            }
        },

        _ => eprintln!("Error! the argument passed as OPTION is not valid\n\
                        Run application.exe --help or application.exe -h to show the instruction to use \
                        the application properly\n"),
    };
}
