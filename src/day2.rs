use std::process::exit;
use std::collections::HashMap;

pub fn main( input: &str ) {
    find_combination( &input, &keypad_one(), (1, 1) );
    find_combination( &input, &keypad_two(), (0, 2) );
}

fn find_combination( input: &str, keypad: &HashMap<(i32,i32),String>, start: (i32, i32) ) {
    let mut point = (start.0, start.1);
    for line in input.trim().split( "\n" ) {
        for instruction in line.chars() {
            match instruction {
                'U' => { if keypad.contains_key( &(point.0, point.1 - 1) ) {
                            point.1 = point.1 - 1; } },
                'D' => { if keypad.contains_key( &(point.0, point.1 + 1) ) { 
                            point.1 = point.1 + 1; } },
                'L' => { if keypad.contains_key( &(point.0 - 1, point.1) ) { 
                            point.0 = point.0 - 1; } },
                'R' => { if keypad.contains_key( &(point.0 + 1, point.1) ) { 
                            point.0 = point.0 + 1; } },
                _   => {
                    println!( "Unknown instruction encountered {:?} in line {:?}", instruction, line );
                    exit( 1 );
                },
            }
        }
        print!( "{}", keypad.get( &point ).unwrap() );
    }
    println!( "" );
}

fn keypad_one() -> HashMap<(i32,i32),String> {
    let mut keypad = HashMap::new();
    let mut number = 0; 
    for y in 0..3 {
        for x in 0..3 {
            number += 1;
            keypad.insert( (x, y), number.to_string() );
        }
    }
    keypad
}

fn keypad_two() -> HashMap<(i32,i32),String> {
    let mut keypad = HashMap::new();
    keypad.insert( (2, 0), format!( "1" ) );
    keypad.insert( (1, 1), format!( "2" ) );
    keypad.insert( (2, 1), format!( "3" ) );
    keypad.insert( (3, 1), format!( "4" ) );
    keypad.insert( (0, 2), format!( "5" ) );
    keypad.insert( (1, 2), format!( "6" ) );
    keypad.insert( (2, 2), format!( "7" ) );
    keypad.insert( (3, 2), format!( "8" ) );
    keypad.insert( (4, 2), format!( "9" ) );
    keypad.insert( (1, 3), format!( "A" ) );
    keypad.insert( (2, 3), format!( "B" ) );
    keypad.insert( (3, 3), format!( "C" ) );
    keypad.insert( (2, 4), format!( "D" ) );
    keypad
}
