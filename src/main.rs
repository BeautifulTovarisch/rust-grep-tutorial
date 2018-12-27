use std::env;
use std::process;

mod cli;

use crate::cli::{ run, get_args, Param };

fn main() {
    let Param { query, filename } = get_args( env::args() ).unwrap_or_else( |err| {
        eprintln!( "Error parsing arguments: {}", err );
        process::exit( 1 )
    });

    if let Err( e ) = run( query, filename ) {
        eprintln!( "Error {}", e );
        process::exit( 1 );
    }
}
