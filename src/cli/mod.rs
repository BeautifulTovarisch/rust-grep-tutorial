use std::fs;
use std::io;
use std::error::Error;

pub struct Param {
    pub query: String,
    pub filename: String
}

pub fn search<'a>( query: &str, contents: &'a str ) -> Vec<&'a str> {
    contents.lines()
        .filter( |line| line.contains( query ) )
        .collect()
}

pub fn get_args( args: &[String] ) -> Result<Param, &'static str> {
    if args.len() < 3 {
        return Err( "Invalid number of arguments!" );
    }

    Ok( Param { query: args[1].clone(), filename: args[2].clone() } )
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
