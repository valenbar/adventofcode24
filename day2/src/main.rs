#![allow(dead_code)]
#![allow(unused_variables)]
use std::fs;
use eyre::Result;
use itertools::Itertools;

static RESULT1_DEMO: isize = 2;
static RESULT1_REAL: isize = 379;
static RESULT2_DEMO: isize = 4;
static RESULT2_REAL: isize = 430;

fn is_safe(vec: &[usize]) -> bool {
    let windows = vec.windows(2).collect_vec();
    let x = match vec[..] {
        [a,b,..] if a == b => false,
        [a,b,..] if a < b => {
            windows.iter().all(|&w| w[1].abs_diff(w[0]) <= 3 && (w[1] as isize) - (w[0] as isize) > 0)
        },
        [a,b,..] if a > b => {
            windows.iter().all(|&w| w[0].abs_diff(w[1]) <= 3 && (w[0] as isize) - (w[1] as isize) > 0)
        },
        _ => false,
    };
    x
}

fn solve_task_1(input: &str) -> Result<isize> {
    let input = fs::read_to_string(input)?;

    // SOLVE TASK 1
    let result = input
        .lines()
        .map(|x| 
            x.split_whitespace()
            .map(|y| y.parse::<usize>())
            .map(Result::unwrap)
            .collect::<Vec<usize>>()
        )
        .filter(|vec| is_safe(vec)).count();

    println!("{:?}", result);

    Ok(result as isize)
}

fn solve_task_2(input: &str) -> Result<isize> {
    let input = fs::read_to_string(input)?;

    // SOLVE TASK 1
    let result = input
        .lines()
        .map(|x| 
            x.split_whitespace()
            .map(|y| y.parse::<usize>())
            .map(Result::unwrap)
            .collect::<Vec<usize>>()
        )
        .filter(|vec| {
                match is_safe(vec) {
                    true => true,
                    false => { 
                        for i in 0..vec.len() {
                            let mut vec_ = vec.clone();
                            vec_.remove(i);
                            if is_safe(&vec_) {
                                return true
                            }
                        }
                        false
                    }
                }
            }
        ).count();

    println!("{:?}", result);

    Ok(result as isize)
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
