use std::io;

enum Direction {
    North,
    West,
    East,
}

fn main() {
    println!("Enter direction: ");
    let mut dir = String::new();
    io::stdin().read_line(&mut dir).expect("can't understand");
    let dir = dir.trim();

    let direction = match dir.to_lowercase().as_str() {
        "north" => Some(Direction::North),
        "west" => Some(Direction::West),
        "east" => Some(Direction::East),
        _ => None,
    };

    let result = match direction {
        Some(Direction::North) => "The direction is North",
        Some(Direction::West) => "The direction is West",
        Some(Direction::East) => "The direction is East",
        None => "Nahh that direction don't exist anymore!",
    };

    println!("The Found Direction is: {}", result);
}
