extern crate getopts;
use getopts::Options;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::exit;

mod day1;
mod day2;

fn main() {
    let args: Vec<String> = env::args().collect();
    parse_opt( &args );
}

fn parse_opt( args: &Vec<String> ) {
    let mut options = Options::new();
    options.optopt( "d", "day", "specify the day number", "<day_number>" );
    options.optopt( "i", "input", "specify an input file", "<file_name>" );
    options.optflag( "h", "help", "print this help menu" );
    
    let matches = match options.parse( &args[1..] ) {
        Ok ( m ) => { m },
        Err( f ) => { 
            println!( "{}", f.to_string() );
            exit( 1 );
        },
    };

    if matches.opt_present( "h" ) {
        print_usage( &args[0], options );
        exit( 0 );
    }

    let input_file = matches.opt_str( "i" );
    let input = match input_file {
        Some( x ) => { read_input( &x ) },
        None      => {
            println!( "No input file specified!" );
            exit( 1 );
        },
    };
    
    let day = matches.opt_str( "d" );
    match day {
        Some( x ) => {
            match x.as_ref() {
                "1"  => day1::main( &input ),
                "2"  => day2::main( &input ),
                _    => { 
                    println!( "Unknown day number {}", x );
                    exit( 1 );
                },
            }
        },
        None => {
            println!( "No day specified!" );
            exit( 1 );
        },
    }
}

fn print_usage( prog_name: &str, options: Options ) {
    let brief = format!( "Usage: {} -d <day_number> -i <input_file> [-h]", prog_name );
    print!( "{}", options.usage( &brief ) );
}

fn read_input( input: &str ) -> String {
    let path = Path::new( input );
    let mut file = match File::open( &path ) {
        Ok ( f ) => { f },
        Err( f ) => {
            println!( "Error opening file {}: {}", path.display(), f.description() );
            exit( 1 );
        },
    };
    let mut data = String::new();
    match file.read_to_string( &mut data ) {
        Ok ( _ ) => { }
        Err( f ) => {
            println!( "Error reading from file {}: {}", path.display(), f.description() );
        },
    }
    data
}
