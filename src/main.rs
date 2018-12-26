use std::env;
use std::process;

mod cli;

use crate::cli::{ run, get_args, Param };

fn main() {
    let args: Vec<String> = env::args().collect();

    let Param { query, filename } = get_args( &args ).unwrap_or_else( |err| {
        println!( "Error parsing arguments: {}", err );
        process::exit( 1 )
    });

    if let Err( e ) = run( query, filename ) {
        println!( "Error {}", e );
        process::exit( 1 );
    } else {
        process::exit( 0 );
    }
}