use std::process::exit;
use std::error::Error;

pub fn main( input: &str ) {
    part_one( &input );
    part_two( &input );
}

fn is_valid( triangle: &(i32, i32, i32) ) -> bool {
    triangle.0 + triangle.1 > triangle.2 &&
        triangle.1 + triangle.2 > triangle.0 &&
        triangle.0 + triangle.2 > triangle.1
}

fn part_one( input: &str ) {
    let mut valid_triangles = 0;
    for line in input.trim().split( "\n" ) {
        let triangle: Vec<i32> = line.split_whitespace().map( 
            |s| match s.parse::<i32>() {
                Ok ( x ) => { x },
                Err( f ) => { 
                    println!( 
                        "Encountered illegal character {} in line {}: {}", 
                        s, line, f.description() );
                    exit( 1 );
                },
            }
        ).collect();
        if is_valid( &(triangle[0], triangle[1], triangle[2]) ) {
            valid_triangles += 1;
        }
    }
    println!( "{} valid triangles", valid_triangles );
}

fn part_two( input: &str ) {
    let mut valid_triangles = 0;
    let mut rows: Vec<i32> = Vec::new();
    for line in input.trim().split( "\n" ) {
        let mut row: Vec<i32> = line.split_whitespace().map(
            |s| match s.parse::<i32>() {
                Ok ( x ) => { x },
                Err( f ) => { 
                    println!( 
                        "Encountered illegal character {}: {}", 
                        s, f.description() );
                    exit( 1 );
                },
            }
        ).collect();
        rows.append( &mut row );
    }
    let mut i = 0;
    while i < rows.len() - 6 {
        if is_valid( &(rows[i], rows[i + 3], rows[i + 6]) ) {
            valid_triangles += 1;
        }
        i += 1;
        if i % 3 == 0 {
            i += 6;
        }
    }
    println!( "{} valid triangles", valid_triangles );
}
