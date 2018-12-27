use std::fs;
use std::io;
use std::error::Error;

pub struct Param {
    pub query: String,
    pub filename: String
}

/// Search a collection of strings given a query
///
/// # Examples
///
/// let query = "Test"
///
/// let contents = "\
/// This Line Doesn't Return
/// Test. This one should.
///
/// assert_eq!( search( query, contents ), vec![ "Test.",
///                                              "This",
///                                              "one",
///                                              "should." ] );

pub fn search<'a>( query: &str, contents: &'a str ) -> Vec<&'a str> {
    contents.lines()
        .filter( |line| line.contains( query ) )
        .collect()
}

pub fn get_args( mut args: std::env::Args ) -> Result<Param, &'static str> {
    if args.len() < 3 {
        return Err( "Invalid number of arguments!" );
    }

    let query = match args.nth( 1 ) {
        Some( arg ) => arg,
        None => return Err( "Query not provided." )
    };

    let filename = match args.nth( 2 ) {
        Some( arg ) => arg,
        None => return Err( "Filename, not provided." )
    };

    Ok( Param { query, filename } )
}

pub fn read_file( filename: &String ) -> Result<String, io::Error> {
    fs::read_to_string( filename )
}

pub fn run( query: String, filename: String ) -> Result<(), Box<dyn Error>> {
    let contents = read_file( &filename )?;
    for line in search( &query, &contents ) {
        println!( "{}", line );
    }

    Ok( () )
}

#[cfg(test)]
mod test;
