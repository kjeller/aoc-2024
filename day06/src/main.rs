use std::{collections::HashSet, hash::Hash};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position(i32, i32);

enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

impl Orientation {
    fn get_char(&self) -> char {
        match self {
            Orientation::Up => '^',
            Orientation::Down => 'v',
            Orientation::Left => '<',
            Orientation::Right => '>',
        }
    }
}

struct Map {
    width: i32,
    height: i32,
    obstacles: Vec<Position>,
    player: Player,
}

impl Map {
    fn new(map_row: Vec<&str>) -> Map {
        let width = map_row[0].len() as i32;
        let height = map_row.len() as i32;
        let mut obstacles: Vec<Position> = Vec::new();
        let mut player: Player = Player::new();

        for (row, map_line) in map_row.iter().enumerate() {
            //let mut row: Vec<Option<(u8, u8)>> = Vec::new();
            for (col, c) in map_line.chars().enumerate() {
                match c {
                    '#' => {
                        obstacles.push(Position(col as i32, row as i32));
                    }
                    '^' => {
                        let start_position = Position(col as i32, row as i32);
                        player.current = start_position;
                        player.start = start_position;
                        player.orientation = Orientation::Up;
                    }
                    _ => {}
                }
            }
        }
        Map {
            width,
            height,
            obstacles,
            player,
        }
    }

    fn update_player_position(&mut self) {
        let next_position = self.player.next_position();
        if self.obstacles.contains(&next_position) {
            self.player.rotate_clockwise();
        } else {
            self.player.update_position(next_position);
        }
    }

    fn is_player_in_map(&self) -> bool {
        self.player.current.0 > 0
            && self.player.current.0 < self.height
            && self.player.current.1 > 0
            && self.player.current.1 < self.width
    }

    fn print_map(&self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let position = Position(col as i32, row as i32);
                let c: char = if self.obstacles.contains(&position) {
                    '#'
                } else if position == self.player.current {
                    self.player.orientation.get_char()
                } else {
                    '.'
                };
                print!("{}", c);
            }
            println!();
        }
        println!();
    }
}

struct Player {
    current: Position,
    orientation: Orientation,
    history: HashSet<Position>,
    start: Position,
}

impl Player {
    fn new() -> Player {
        Player {
            current: Position(0, 0),
            orientation: Orientation::Up,
            start: Position(0, 0),
            history: HashSet::new(),
        }
    }
    fn next_position(&mut self) -> Position {
        match self.orientation {
            Orientation::Up => Position(self.current.0, self.current.1 - 1),
            Orientation::Down => Position(self.current.0, self.current.1 + 1),
            Orientation::Left => Position(self.current.0 - 1, self.current.1),
            Orientation::Right => Position(self.current.0 + 1, self.current.1),
        }
    }
    fn update_position(&mut self, new: Position) {
        self.history.insert(self.current);
        self.current = new;
    }
    fn rotate_clockwise(&mut self) {
        match self.orientation {
            Orientation::Up => self.orientation = Orientation::Right,
            Orientation::Down => self.orientation = Orientation::Left,
            Orientation::Left => self.orientation = Orientation::Up,
            Orientation::Right => self.orientation = Orientation::Down,
        }
    }
}

fn part_1() -> i32 {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut map = Map::new(input.lines().into_iter().map(|line| line).collect());
    while map.is_player_in_map() {
        // map.print_map();
        map.update_player_position();
    }
    map.player.history.len() as i32
}

fn main() {
    println!("{}", part_1());
}
