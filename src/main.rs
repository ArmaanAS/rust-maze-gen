use crate::maze_opt::Maze;

mod maze_opt;
mod maze;

fn main() {
    // let mut maze = maze::Maze::new(69, 37);
    let args: Vec<String> = std::env::args().collect();

    let width: u32;
    let height: u32;
    if args.len() == 1 {
        width = 16;
        height = 16
    } else if args.len() == 2 {
        if args[1] == "--help" || args[1] == "?" {
            println!(
                "Maze generation algorithm. Print an WxH sized maze to stdout.                

USAGE:
    maze [width=16] [height=16]

OPTIONS:
    [width]  = 16          Columns of the inner-maze and height if height is unspecified
    [height] = 16          Rows of the inner-maze.
            "
            );
            return;
        }

        let temp = args[1].parse::<u32>().unwrap_or(16);
        width = temp;
        height = temp;
    } else {
        width = args[1].parse::<u32>().unwrap_or(16);
        height = args[2].parse::<u32>().unwrap_or(16);
    }

    let mut maze = Maze::new(width, height);
    maze.generate();
    println!("{}", maze.to_string());
}
