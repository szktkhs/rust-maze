enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    fn right(&self) -> Direction {
        match *self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
    fn left(&self) -> Direction {
        match *self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
    fn to_strint(&self) -> &'static str {
        match *self {
            Direction::East => "East",
            Direction::West => "West",
            Direction::North => "North",
            Direction::South => "South",
        }
    }
}

struct Position {
    x : usize,
    y : usize,
}

impl Position {
    fn new(x : usize, y : usize) -> Position {
        Position { x : x, y : y }
    }
    
    fn move_to(&self, dir : &Direction) -> Position {
        match *dir {
            Direction::North => Position::new(self.x, self.y - 1),
            Direction::South => Position::new(self.x, self.y + 1),
            Direction::East => Position::new(self.x + 1, self.y),
            Direction::West => Position::new(self.x - 1, self.y),
        }
    }
}

enum TileKind {
    Start,
    Goal,
    Isle,
    Wall,
}

struct Tile {
    kind: TileKind,
}

impl Tile {
    fn new(kind : TileKind) -> Tile {
        Tile {
            kind : kind,
        }
    }
    fn is_enterable(&self) -> bool {
        match self.kind {
            TileKind::Wall => false,
            _ => true,
        }
    }
    fn is_start(&self) -> bool {
        match self.kind {
            TileKind::Start => true,
            _ => false,
        }
    }
    fn is_goal(&self) -> bool {
        match self.kind {
            TileKind::Goal => true,
            _ => false,
        }
    }
}

type Maze = Vec<Vec<Tile>>;

trait Matrix<T> {
    fn at(&self, pos : &Position) -> &T;
}

impl Matrix<Tile> for Maze {
    fn at<'a, 'b>(&'a self, pos : &'b Position) -> &'a Tile {
        &self[pos.y][pos.x]
    }
}

fn find_start(maze : &Maze) -> Position {
    for j in 0..maze.len() {
        for i in 0..maze[j].len() {
            if maze[j][i].is_start() {
                return Position::new(i, j);
            }
        }
    }
    panic!("No START tile");
}

fn main() {
    let maze : Maze = vec![
        vec![
            Tile::new(TileKind::Wall),
            Tile::new(TileKind::Wall),
            Tile::new(TileKind::Wall),
            Tile::new(TileKind::Wall),
            Tile::new(TileKind::Wall),
            Tile::new(TileKind::Wall),
            Tile::new(TileKind::Wall),
            Tile::new(TileKind::Wall),
        ],
        vec![
            Tile::new(TileKind::Wall),
            Tile::new(TileKind::Start),
            Tile::new(TileKind::Wall),
            Tile::new(TileKind::Wall),
            Tile::new(TileKind::Isle),
            Tile::new(TileKind::Isle),
            Tile::new(TileKind::Isle),
            Tile::new(TileKind::Wall),
        ],
        vec![
            Tile::new(TileKind::Wall),
            Tile::new(TileKind::Isle),
            Tile::new(TileKind::Wall),
            Tile::new(TileKind::Wall),
            Tile::new(TileKind::Wall),
            Tile::new(TileKind::Wall),
            Tile::new(TileKind::Isle),
            Tile::new(TileKind::Wall),
        ],
        vec![
            Tile::new(TileKind::Wall),
            Tile::new(TileKind::Isle),
            Tile::new(TileKind::Isle),
            Tile::new(TileKind::Isle),
            Tile::new(TileKind::Isle),
            Tile::new(TileKind::Wall),
            Tile::new(TileKind::Isle),
            Tile::new(TileKind::Wall),
        ],
        vec![
            Tile::new(TileKind::Wall),
            Tile::new(TileKind::Wall),
            Tile::new(TileKind::Wall),
            Tile::new(TileKind::Wall),
            Tile::new(TileKind::Isle),
            Tile::new(TileKind::Isle),
            Tile::new(TileKind::Isle),
            Tile::new(TileKind::Wall),
        ],
        vec![
            Tile::new(TileKind::Wall),
            Tile::new(TileKind::Wall),
            Tile::new(TileKind::Isle),
            Tile::new(TileKind::Isle),
            Tile::new(TileKind::Isle),
            Tile::new(TileKind::Wall),
            Tile::new(TileKind::Wall),
            Tile::new(TileKind::Wall),
        ],
        vec![
            Tile::new(TileKind::Wall),
            Tile::new(TileKind::Isle),
            Tile::new(TileKind::Isle),
            Tile::new(TileKind::Wall),
            Tile::new(TileKind::Isle),
            Tile::new(TileKind::Isle),
            Tile::new(TileKind::Goal),
            Tile::new(TileKind::Wall),
        ],
        vec![
            Tile::new(TileKind::Wall),
            Tile::new(TileKind::Wall),
            Tile::new(TileKind::Wall),
            Tile::new(TileKind::Wall),
            Tile::new(TileKind::Wall),
            Tile::new(TileKind::Wall),
            Tile::new(TileKind::Wall),
            Tile::new(TileKind::Wall),
        ],
    ];
    let mut pos = find_start(&maze);
    let mut dir = Direction::North;
    loop {
        if maze.at(&pos).is_goal() {
            println!("Goal!");
            break;
        }
        println!("Position {},{}", pos.x, pos.y);
        dir = dir.right();
        loop {
            let tmp_pos = pos.move_to(&dir);
            if maze.at(&tmp_pos).is_enterable() {
                println!("Move to {}", dir.to_strint());
                pos = tmp_pos;
                break;
            } else {
                dir = dir.left();
            }
        }
    }
}
