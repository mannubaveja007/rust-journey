enum Direction {
    North,
    South,
    East,
    West,
}

fn move_player(dir: Direction) {
    match dir {
        Direction::North => println!("Going up"),
        Direction::South => println!("Going down"),
        Direction::East  => println!("Going right"),
        Direction::West  => println!("Going left"),
    }
}

fn main(){
    print!("Mutthh maar");
    move_player(Direction::South);
}
