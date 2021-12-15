use std::{
    cell::RefCell,
    fmt::{Display, Formatter, Result},
};

use rand::random;

pub struct Maze {
    width: u32,
    height: u32,
    w: u32,
    h: u32,
    walls: Vec<bool>,
}

type Point = (u32, u32);

impl Maze {
    pub fn new(width: u32, height: u32) -> Maze {
        let w = width * 2 - 1;
        let h = height * 2 - 1;
        let walls = vec![true; (w * h) as usize];

        // for y in (0..=h).step_by(2) {
        //     for x in (0..=w).step_by(2) {
        //         let idx = y * w + x;
        //         walls[idx as usize] = false;
        //     }
        // }

        Maze {
            width,
            height,
            w,
            h,
            walls,
        }
    }

    pub fn generate(&mut self) {
        let explored = RefCell::<Vec<bool>>::new(vec![false; (self.width * self.height) as usize]);
        let mut stack: Vec<Point> = vec![(0, 0)];
        explored.borrow_mut()[0] = true;

        let index = |x: u32, y: u32| (y * self.width + x) as usize;

        let unexplored_neighbours = |(x, y): Point| {
            let ex = explored.borrow();
            let mut unexplored: Vec<Point> = Vec::new();
            if x > 0 && !ex[index(x - 1, y)] {
                unexplored.push((x - 1, y));
            }

            if y > 0 && !ex[index(x, y - 1)] {
                unexplored.push((x, y - 1));
            }

            if x < self.width - 1 && !ex[index(x + 1, y)] {
                unexplored.push((x + 1, y));
            }

            if y < self.height - 1 && !ex[index(x, y + 1)] {
                unexplored.push((x, y + 1));
            }

            unexplored
        };

        // let mut break_wall_between = |(x, y): Point, (a, b): Point| {
        //     let wx: u32;
        //     let wy: u32;
        //     if x == a {
        //         wx = x * 2;
        //         wy = if y > b { b } else { y } * 2 + 1;
        //     } else {
        //         wx = if x > a { a } else { x } * 2 + 1;
        //         wy = y * 2;
        //     }
        //     let idx = wy * self.w + wx;

        //     self.walls[idx as usize] = false;
        // };

        let mut count = 0;
        while stack.len() > 0 {
            let pos = stack[stack.len() - 1];
            let neighbours = unexplored_neighbours(pos);
            let idx = pos.1 * 2 * self.w + pos.0 * 2;
            self.walls[idx as usize] = false;

            if count == 5 || neighbours.len() == 0 {
                count = 0;
                stack.pop();
            } else if neighbours.len() == 1 {
                count += 1;
                stack.pop();
                stack.push(neighbours[0]);

                let idx = index(neighbours[0].0, neighbours[0].1);
                explored.borrow_mut()[idx] = true;

                // break_wall_between(pos, neighbours[0]);
                let (a, b) = pos;
                let (x, y) = neighbours[0];
                let wx: u32;
                let wy: u32;
                if x == a {
                    wx = x * 2;
                    wy = if y > b { b } else { y } * 2 + 1;
                } else {
                    wx = if x > a { a } else { x } * 2 + 1;
                    wy = y * 2;
                }
                let idx = wy * self.w + wx;

                self.walls[idx as usize] = false;
            } else {
                count += 1;
                let rand: usize = random();
                let i = rand % neighbours.len();
                let neighbour = neighbours[i];

                stack.push(neighbour);

                let idx = index(neighbour.0, neighbour.1);
                explored.borrow_mut()[idx] = true;

                // break_wall_between(pos, neighbour);
                let (a, b) = pos;
                let (x, y) = neighbour;
                let wx: u32;
                let wy: u32;
                if x == a {
                    wx = x * 2;
                    wy = if y > b { b } else { y } * 2 + 1;
                } else {
                    wx = if x > a { a } else { x } * 2 + 1;
                    wy = y * 2;
                }
                let idx = wy * self.w + wx;

                self.walls[idx as usize] = false;
            }
        }
    }

    pub fn furthest_point_from(&self, x: u32, y: u32) -> Point {
        let explored = RefCell::<Vec<bool>>::new(vec![false; (self.width * self.height) as usize]);

        let index = |x: u32, y: u32| (y * self.width + x) as usize;
        let pindex = |(x, y): Point| (y * self.width + x) as usize;
        let windex = |x: u32, y: u32| (y * self.w + x) as usize;

        explored.borrow_mut()[index(x, y)] = true;
        let mut points: Vec<Point> = vec![(x, y)];

        let unexplored_neighbours = |(x, y): Point| {
            let ex = explored.borrow();
            let mut unexplored: Vec<Point> = Vec::new();
            if x > 0 && !ex[index(x - 1, y)] && !self.walls[windex(x * 2 - 1, y * 2)] {
                unexplored.push((x - 1, y));
            }

            if y > 0 && !ex[index(x, y - 1)] && !self.walls[windex(x * 2, y * 2 - 1)] {
                unexplored.push((x, y - 1));
            }

            if x < self.width - 1 && !ex[index(x + 1, y)] && !self.walls[windex(x * 2 + 1, y * 2)] {
                unexplored.push((x + 1, y));
            }

            if y < self.height - 1 && !ex[index(x, y + 1)] && !self.walls[windex(x * 2, y * 2 + 1)]
            {
                unexplored.push((x, y + 1));
            }

            unexplored
        };

        let mut furthest = (x, y);
        while points.len() > 0 {
            furthest = points[0];
            let mut new_points = Vec::<Point>::new();

            for point in points {
                let neighbours = unexplored_neighbours(point);

                for neighbour in neighbours {
                    let idx = pindex(neighbour);
                    explored.borrow_mut()[idx] = true;
                    new_points.push(neighbour);
                }
            }

            points = new_points;
        }

        (furthest.0 * 2, furthest.1 * 2)
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // let start = (0, 0);
        let end = self.furthest_point_from(0, 0);
        let start = self.furthest_point_from(end.0 / 2, end.1 / 2);

        write!(f, "\n  {}", "██".repeat((self.w + 2) as usize))?;
        for y in 0..self.h {
            write!(f, "\n  ██")?;
            for x in 0..self.w {
                let idx = (y * self.w + x) as usize;
                let is_wall = self.walls[idx];
                if end == (x, y) {
                    write!(f, "\x1b[31;41m  \x1b[0m")?;
                } else if start == (x, y) {
                    write!(f, "\x1b[32;42m  \x1b[0m")?;
                } else {
                    write!(f, "{}", if is_wall { "██" } else { "  " })?;
                }
            }
            write!(f, "██")?;
        }
        write!(f, "\n  {}\n", "██".repeat((self.w + 2) as usize))?;

        Ok(())
    }
}
