use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

pub enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    pub fn right(&self) -> Direction {
        match *self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
    pub fn left(&self) -> Direction {
        match *self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
    pub fn to_strint(&self) -> &'static str {
        match *self {
            Direction::East => "East",
            Direction::West => "West",
            Direction::North => "North",
            Direction::South => "South",
        }
    }
}

pub struct Position {
    pub x : usize,
    pub y : usize,
}

impl Position {
    pub fn new(x : usize, y : usize) -> Position {
        Position { x : x, y : y }
    }
    
    pub fn move_to(&self, dir : &Direction) -> Position {
        match *dir {
            Direction::North => Position::new(self.x, self.y - 1),
            Direction::South => Position::new(self.x, self.y + 1),
            Direction::East => Position::new(self.x + 1, self.y),
            Direction::West => Position::new(self.x - 1, self.y),
        }
    }
}

pub enum TileKind {
    Start,
    Goal,
    Isle,
    Wall,
}

pub struct Tile {
    kind: TileKind,
}

#[allow(dead_code)]
impl Tile {
    pub fn new(kind : TileKind) -> Tile {
        Tile {
            kind : kind,
        }
    }
    pub fn is_enterable(&self) -> bool {
        match self.kind {
            TileKind::Wall => false,
            _ => true,
        }
    }
    pub fn is_start(&self) -> bool {
        match self.kind {
            TileKind::Start => true,
            _ => false,
        }
    }
    pub fn is_goal(&self) -> bool {
        match self.kind {
            TileKind::Goal => true,
            _ => false,
        }
    }
}

pub type Maze = Vec<Vec<Tile>>;

pub trait Matrix<T> {
    fn at(&self, pos : &Position) -> &T;
}

impl Matrix<Tile> for Maze {
    fn at<'a, 'b>(&'a self, pos : &'b Position) -> &'a Tile {
        &self[pos.y][pos.x]
    }
}

pub fn read_file(file_name : &String)
                 -> Result<(Maze, Option<Position>), io::Error> {
    let f = try!(File::open(file_name));

    let reader = BufReader::new(f);
    
    let mut start : Option<Position> = None;
    let mut maze : Maze = Vec::new();
    for (j,line) in reader.lines().enumerate() {
        let line = try!(line);
        let mut row : Vec<Tile> = Vec::new();
        for (i,c) in line.chars().enumerate() {
            row.push(Tile::new(match c {
                ' ' => TileKind::Isle,
                'S' => {
                    start = Some(Position::new(i, j));
                    TileKind::Start
                },
                'G' => TileKind::Goal,
                _ => TileKind::Wall
            }));
        }
        maze.push(row);
    }
    Ok((maze, start))
}
