use std::env;
use std::process;

mod maze_utils;
use maze_utils::Matrix;

fn print_usage(name : &String) {
    println!("Usage > {} <Maze File>", name);
}

fn main() {
    let file_name = env::args().nth(1)
        .unwrap_or_else(|| {
            print_usage(&env::args().nth(0).unwrap());
            process::exit(1);
        });

    let (maze, start) = maze_utils::read_file(&file_name)
        .expect("read_file failed");
    
    if let Some(mut pos) = start {
        let mut dir = maze_utils::Direction::North;
        println!("Start from {},{}", pos.x, pos.y);
        loop {
            if maze.at(&pos).is_goal() {
                println!("Goal!");
                break;
            }
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
    } else {
        println!("No sart tile");
    }
}
