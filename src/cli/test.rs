use std::fs::{
    write,
    remove_file
};

use std::path::PathBuf;

use super::{
    run,
    search,
    read_file,
};

#[test]
fn test_run() {
    let filename = "test_run.test";

    write( filename, b"Some Content" ).unwrap();

    let query = String::from( "some regex" );
    let file = PathBuf::from( filename );

    assert!( run( query, file ).is_ok() );

    remove_file( filename ).unwrap();
}

#[test]
fn test_search() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!( search( query, contents ), vec![ "safe, fast, productive." ] );
}

#[test]
fn test_read_file_ok() {
    let filename = "test_file.test";

    write( filename, b"Test Content!" ).unwrap();

    if let Ok( contents ) = read_file( &PathBuf::from( "test_file.test" ) ) {
        assert_eq!( contents, "Test Content!" );
    } else {
        panic!( "Test Failed" );
    }

    remove_file( filename ).unwrap();
}

#[test]
fn test_read_file_err() {
    let does_not_exist = PathBuf::from( "does_not_exist.test" );

    assert!( read_file( &does_not_exist ).is_err() );
}
