#![allow(dead_code)]
#![allow(unused_variables)]
use std::{fmt::Debug, fs};
use eyre::Result;

static RESULT1_DEMO: usize = 41;
static RESULT1_REAL: usize = 5153;
static RESULT2_DEMO: usize = 6;
static RESULT2_REAL: usize = 0;


enum Direction {
    North,
    South,
    East,
    West
}

impl Direction {
    fn rotate(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

impl Debug for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::North => write!(f, "North"),
            Self::South => write!(f, "South"),
            Self::East => write!(f, "East"),
            Self::West => write!(f, "West"),
        }
    }
}

enum Location {
    Nothing,
    Obstacle,
    Outside,
}

struct Watcher {
    x: isize,
    y: isize,
    direction: Direction,
    map: Vec<char>,
    h: usize,
    w: usize,
    count: usize
}

impl Debug for Watcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Watcher").field("x", &self.x).field("y", &self.y).field("direction", &self.direction).field("map", &self.map).field("h", &self.h).field("w", &self.w).field("count", &self.count).finish()
    }
}

impl Watcher {
    fn new(x: isize, y: isize, h: usize, w: usize, mut map: Vec<char>) -> Watcher {
        map[(x as usize) * h + y as usize] = 'X';
        Watcher {
            x, 
            y,
            direction: Direction::North,
            map,
            h,
            w,
            count: 0,
        }
    }

    fn go(&mut self) -> Option<(isize, isize)> {
        let (x_, y_) = match self.direction {
            Direction::North => (self.x-1, self.y),
            Direction::South => (self.x+1, self.y),
            Direction::East => (self.x, self.y+1),
            Direction::West => (self.x, self.y-1),
        };
        match self.check_location(x_, y_) {
            Location::Nothing => {
                self.x = x_;
                self.y = y_;
                Some((x_, y_))
            },
            Location::Obstacle => {
                self.direction = self.direction.rotate();
                Some((self.x, self.y))
            },
            Location::Outside => None
        }
    }

    fn check_location(& mut self, x: isize, y: isize) -> Location {
        if x >= self.h as isize || x < 0 || y >= self.w as isize || y < 0 {
            return Location::Outside
        }
        match self.map[(x as usize) * self.h + (y as usize)] {
            '#' => Location::Obstacle,
            '.' => {
                self.map[(x as usize) * self.h + (y as usize)] = 'X';
                Location::Nothing
            },
            'X' => Location::Nothing,
            c => panic!("weird character {c} recognized"),
        }
    }

    fn count_visited(& self) -> usize {
        self.map.iter().filter(|&c| *c == 'X').count()
    }
}

fn solve_task_1(input: &str) -> Result<usize> {
    let input = fs::read_to_string(input)?;

    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();

    let grid: Vec<char> = input.lines().flat_map(|l| l.chars()).collect();

    let (pos, _) = grid.iter().enumerate().find(|(i, &x)| x == '^').unwrap();

    let mut watcher = Watcher::new((pos / height).try_into().unwrap(), (pos % width).try_into().unwrap(), height, width, grid);

    while let Some((new_x, new_y)) = watcher.go() {
        // println!("new_x: {new_x}, new_y: {new_y}");
    }


    let result = watcher.count_visited();

    Ok(result)
}

fn solve_task_2(input: &str) -> Result<usize> {
    let input = fs::read_to_string(input)?;

    // SOLVE TASK 2

    let result = 0;

    Ok(result)
}

fn main() {
    println!("Task1: {:?}", solve_task_1("input_demo.txt"));
    println!("Task1: {:?}", solve_task_1("input.txt"));
    println!("Task2: {:?}", solve_task_2("input_demo.txt"));
    println!("Task2: {:?}", solve_task_2("input.txt"));
}

#[test]
fn test_task_1_demo() -> Result<()> {
    let result = solve_task_1("input_demo.txt")?;
    assert_eq!(result, RESULT1_DEMO);
    Ok(())
}

#[test]
fn test_task_1_real() -> Result<()> {
    let result = solve_task_1("input.txt")?;
    assert_eq!(result, RESULT1_REAL);
    Ok(())
}

#[test]
fn test_task_2_demo() -> Result<()> {
    let result = solve_task_2("input_demo.txt")?;
    assert_eq!(result, RESULT2_DEMO);
    Ok(())
}

#[test]
fn test_task_2_real() -> Result<()> {
    let result = solve_task_2("input.txt")?;
    assert_eq!(result, RESULT2_REAL);
    Ok(())
}
