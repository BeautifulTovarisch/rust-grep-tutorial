use std::fs::{
    write,
    remove_file
};

use super::{
    Param,
    run,
    search,
    get_args,
    read_file,
};

#[test]
fn test_run() {
    let filename = "test_run.test";

    write( filename, b"Some Content" ).unwrap();

    let query = String::from( "some regex" );
    let file = String::from( filename );

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

// #[test]
// fn test_get_args_ok() {
//     let args: std::env::args(
//         String::from( "" ),
//         String::from( "query" ),
//         String::from( "filename" )
//     );

//     assert!( get_args( args ).is_ok() );

//     if let Ok( Param { query, filename } ) = get_args( args ) {
//         assert_eq!( query, "query" );
//         assert_eq!( filename, "filename" );
//     }
// }

// #[test]
// fn test_get_args_err() {
//     let args: Vec<String> = vec![];
//     assert!( get_args( args ).is_err() );
// }

#[test]
fn test_read_file_ok() {
    let filename = "test_file.test";

    write( filename, b"Test Content!" ).unwrap();

    if let Ok( contents ) = read_file( &String::from( "test_file.test" ) ) {
        assert_eq!( contents, "Test Content!" );
    } else {
        panic!( "Test Failed" );
    }

    remove_file( filename ).unwrap();
}

#[test]
fn test_read_file_err() {
    let does_not_exist = String::from( "does_not_exist.test" );

    assert!( read_file( &does_not_exist ).is_err() );
}
