use std::process;

use structopt::StructOpt;

mod cli;
use crate::cli::{ run };

#[derive(StructOpt)]
struct Cli {
    pattern: String,

    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf
}

fn main() {
    let Cli { pattern, path } = Cli::from_args();

    if let Err( e ) = run( pattern, path ) {
        eprintln!( "Error {}", e );
        process::exit( 1 );
    }
}
