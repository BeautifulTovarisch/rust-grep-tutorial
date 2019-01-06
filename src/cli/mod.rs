use std::fs;
use std::io::{ Error as IOError };
use std::path::{ PathBuf };
use std::error::Error;

/// Search a collection of strings given a pattern
///
/// # Examples
///
/// let pattern = "Test"
///
/// let contents = "\
/// This Line Doesn't Return
/// Test. This one should.
///
/// assert_eq!( search( pattern, contents ), vec![ "Test.",
///                                              "This",
///                                              "one",
///                                              "should." ] );

fn search<'a>( pattern: &str, contents: &'a str ) -> Vec<&'a str> {
    contents.lines()
        .filter( |line| line.contains( pattern ) )
        .collect()
}

fn read_file( path: &PathBuf ) -> Result<String, IOError> {
    fs::read_to_string( path )
}

pub fn run( pattern: String, path: PathBuf ) -> Result<(), Box<dyn Error>> {
    let contents = read_file( &path )?;
    for line in search( &pattern, &contents ) {
        println!( "{}", line );
    }

    Ok( () )
}

#[cfg(test)]
mod test;
