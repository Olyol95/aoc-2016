use std::process::exit;
use std::error::Error;
use std::collections::LinkedList;

#[derive(Clone, Debug)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

#[derive(Clone, Debug)]
struct Location {
    x: i32,
    y: i32,
    direction: Direction,
}

pub fn main( input: &str ) {
    let mut path: LinkedList<Location> = LinkedList::new();
    path.push_back( Location { x: 0, y: 0, direction: Direction::NORTH } );
    for instruction in input.trim().split( ", " ) {
        parse_instruction( &instruction, &mut path );
    }
    let destination = path.back().unwrap().clone();
    println!( 
        "final distance travelled: {}, ({},{})", 
        destination.x.abs() + destination.y.abs(),
        destination.x, destination.y
    );
    'outer: while !path.is_empty() {
        let location1 = path.pop_front().unwrap();
        for location2 in path.iter() {
            if location1.x == location2.x && location1.y == location2.y {
                println!(
                    "distance travelled to first point visited twice: {}, ({},{})",
                    location1.x.abs() + location1.y.abs(),
                    location1.x, location1.y
                );
                break 'outer;
            }
        }
    }
}

fn parse_instruction( instruction: &str, path: &mut LinkedList<Location> ) {
    let direction = instruction.chars().nth( 0 );
    let distance_vec: String = instruction.chars().skip( 1 ).collect();
    let distance: i32 = match distance_vec.parse::<i32>() {
        Ok ( x ) => { x },
        Err( x ) => {
            println!( "Non-numeric distance specified {}: {}", distance_vec, x.description() );
            exit( 1 );
        },
    };
    let step_operation = match direction.unwrap() {
        'L' => {
            match path.back().unwrap().direction {
                Direction::NORTH => { go_west  },
                Direction::SOUTH => { go_east  },
                Direction::EAST  => { go_north },
                Direction::WEST  => { go_south },
            }
        },
        'R' => {
            match path.back().unwrap().direction {
                Direction::NORTH => { go_east  },
                Direction::SOUTH => { go_west  },
                Direction::EAST  => { go_south },
                Direction::WEST  => { go_north },
            }
        },
        x   => {
            println!( "Encountered unknown direction {:?} in instruction {:?}",
                      x,
                      instruction );
            exit( 1 );
        },
    };
    for _ in 0..distance {
        let last_loc = path.back().unwrap().clone();
        path.push_back( step_operation( &last_loc ) );
    }
}

fn go_north( loc: &Location ) -> Location {
    Location { x: loc.x, y: loc.y + 1, direction: Direction::NORTH }
}

fn go_south( loc: &Location ) -> Location {
    Location { x: loc.x, y: loc.y - 1, direction: Direction::SOUTH }
}

fn go_east( loc: &Location ) -> Location {
    Location { x: loc.x + 1, y: loc.y, direction: Direction::EAST  }
}

fn go_west( loc: &Location ) -> Location {
    Location { x: loc.x - 1, y: loc.y, direction: Direction::WEST  }
}
