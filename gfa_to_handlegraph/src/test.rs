/// implement various test function
/// (october 4, 2020)
use super::*;

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

/// implement test function for write_file passing an existing file and path as input
#[test]
fn write_content_on_valid_file() {
    println!("Test function \"write_file\" passing a valid file and a valid content\n");
    let tempfile = "test/test_output_file/test_file.gfa";
    let tempcontent = read_file("test/gfas/gfa2_files/example2.gfa").unwrap();
    check_file_exist(write_file(tempfile, &tempcontent)); 
}

/// implement test function for write_file passing an existing file and path as input
#[test]
fn write_on_unnamed_file() {
    println!("Test function \"write_file\" passing an unnamed file and a valid content\n");
    let tempfile = "";
    let tempcontent = read_file("test/gfas/gfa2_files/example2.gfa").unwrap();
    check_file_exist(write_file(tempfile, &tempcontent)); 
}

/// implement test function for write_file passing an existing file and path as input
#[test]
fn write_empty_file() {
    println!("Test function \"write_file\" passing a valid file and an empty content\n");
    let tempfile = "test/test_output_file/empty_file.gfa";
    check_file_exist(write_file(tempfile, "")); 
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
fn scan_directory_recursively() {
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