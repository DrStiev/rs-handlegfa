/// The point of this project is to implement the rs-handlegraph interface to handling
/// genomic graphs that are being represented via the library rs-gfa
/// Both of the libraries are developed by Christian Fischer and are available on GitHub

/// library for handling the CLI input
#[macro_use]
extern crate clap;

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
    // REMEMBER! ./target/debug/gfa_to_handlegraph --help
    // USAGE: gfa_to_handlegraph.exe [OPTIONS] <INPUT> [SUBCOMMAND]

    // define a default operation
    static DEFAULT_OPERATION: &str = "rf";

    // use clap as a macro to handle the argument passed as parameters via command line
    // (october 5, 2020)
    let matches = clap_app!(gfa_to_handlegraph =>
        (version: "1.0")
        (author: "Stievano Matteo <m.stievano1@campus.unimib.it>")
        (about: "The point of this project is to implement the rs-handlegraph interface to handling\n\
                genomic graphs that are being represented via the library rs-gfa.\n\
                Both of the libraries are developed by Christian Fischer and are available on GitHub\n")
        (@arg OPERATION: -o --operation +takes_value "Sets a custom operation to do with the file\n\
                > rf: read a file\n> rdf: read all files from a directory")
        (@arg INPUT: +required "Set the input file (or the directory) to read")
        (@arg OUTPUT: +required "Set the output file where write the result")
        (@subcommand test => 
            (about: "Controls testing features")
            (version: "1.0")
            (author: "Stievano Matteo <m.stievano1@campus.unimib.it>")
            (@arg verbose: -v --verbose "Print test information verbosely"))).get_matches();

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    let input_file = matches.value_of("INPUT").unwrap();

    let output_file = matches.value_of("OUTPUT").unwrap();

    // Gets a value for operation if supplied by user, or use the default one
    match matches.value_of("OPERATION").unwrap_or(DEFAULT_OPERATION) {
        "rf" => print_file_result(write_file(output_file, &read_file(input_file).unwrap())),

        "rdf" => {
            // take the last 2 characters of the string to check if they are EXACTLY "/*" (or "\*")
            // otherwise the function read_file_directory could not work properly
            // (october 6, 2020)
            if let Some((i, _)) = input_file.char_indices().rev().nth(1) {
                let last_two_char = &input_file[i..];

                match last_two_char {

                    "/*" | "\\*" => {
                        let dir_content = read_directory_files(&input_file);

                        let mut content_to_write: String = "".to_string();

                        // write the content of each file read in the directory passed as input 
                        // (october 6, 2020)
                        for files in dir_content {
                            for file in files {
                                // each file will be write as "filename\ncontent\n\n"
                                content_to_write.push_str(&file);
                                content_to_write.push_str("\n");
                                content_to_write.push_str(&read_file(&file).unwrap().clone());
                                content_to_write.push_str("\n\n");
                            }
                        }

                        print_file_result(write_file(output_file, &content_to_write))
                    },

                    _ => {
                        let dir_path = format!("{}{}", input_file.clone(), "/*");
                        let dir_content = read_directory_files(&dir_path);

                        let mut content_to_write: String = "".to_string();
                        
                        // write the content of each file read in the directory passed as input 
                        // (october 6, 2020)
                        for files in dir_content {
                            for file in files {
                                // each file will be write as "filename\ncontent\n\n"
                                content_to_write.push_str(&file);
                                content_to_write.push_str("\n");
                                content_to_write.push_str(&read_file(&file).unwrap().clone());
                                content_to_write.push_str("\n\n");
                            }
                        }
                        print_file_result(write_file(output_file, &content_to_write))
                    }
                }
            }
        },

        _ => eprintln!("Error! the argument passed as OPERATION is not valid\n\
                        Run application.exe --help or application.exe -h to show how to use \
                        the application properly\n"),
    };
}
