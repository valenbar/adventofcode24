#![allow(dead_code)]
#![allow(unused_variables)]
use std::fs;
use eyre::Result;

static RESULT1_DEMO: isize = 0;
static RESULT1_REAL: isize = 0;
static RESULT2_DEMO: isize = 0;
static RESULT2_REAL: isize = 0;

fn solve_task_1(input: &str) -> Result<isize> {
    let input = fs::read_to_string(input)?;

    // SOLVE TASK 1

    let result = 0;

    Ok(result)
}

fn solve_task_2(input: &str) -> Result<isize> {
    let input = fs::read_to_string(input)?;

    // SOLVE TASK 2

    let result = 0;

    Ok(result)
}

fn main() {
    println!("Task1: {:?}", solve_task_1("input_demo.txt"));
    println!("Task2: {:?}", solve_task_2("input_demo.txt"));
}

#[test]
fn test_task_1_demo() -> Result<()> {
    let result = solve_task_1("input_demo.txt")?;
    assert_eq!(result as isize, RESULT1_DEMO);
    Ok(())
}

#[test]
fn test_task_1_real() -> Result<()> {
    let result = solve_task_1("input.txt")?;
    assert_eq!(result as isize, RESULT1_REAL);
    Ok(())
}

#[test]
fn test_task_2_demo() -> Result<()> {
    let result = solve_task_2("input_demo.txt")?;
    assert_eq!(result as isize, RESULT2_DEMO);
    Ok(())
}

#[test]
fn test_task_2_real() -> Result<()> {
    let result = solve_task_2("input.txt")?;
    assert_eq!(result as isize, RESULT2_REAL);
    Ok(())
}
