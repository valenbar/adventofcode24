#![allow(dead_code)]
#![allow(unused_variables)]
use std::fs;
use eyre::{Ok, Result};

static RESULT1_DEMO: isize = 161;
static RESULT1_REAL: isize = 188192787;
static RESULT2_DEMO: isize = 48;
static RESULT2_REAL: isize = 113965544;

fn solve_task_1(input: &str) -> Result<usize> {
    let input = fs::read_to_string(input)?;

    // parse all "mul(x" get index of x
    let mut candidates: Vec<String> = Vec::new();
    let mut start = 0;

    // collects all mul(*) strings
    while let Some(candidate) = input[start..].find("mul(") {
        start += candidate;
        if let Some(end) = input[start..].find(')') {
            candidates.push(input[start..=start + end].to_string());
        } else {
            break;
        }
        start += 4;
    }

    // go through candidates remove any not containing only numbers and a comma
    candidates = candidates.iter()
        .map(|x| x[4..x.len()-1].to_string()) // get the part enclosed in ()
        .filter(|x| x.len() >= 3)
        .filter(|x| x.chars().all(|c| "0123456789,".contains(c)))
        .filter(|x| x.chars().filter(|y| *y == ',').count() == 1)
        .filter(|x| { 
            let comma_pos = x.find(',').unwrap();
            comma_pos < x.len() - 1 && comma_pos > 0 
        })
        .collect();

    // split at comma and apply multiplication
    let result = candidates.iter()
        .map(|x| x.split(',').flat_map(|y| y.parse::<usize>()).collect::<Vec<usize>>())
        .map(|x| x[0] * x[1])
        .sum::<usize>();

    Ok(result)
}

fn solve_task_2(input: &str) -> Result<usize> {
    let input = fs::read_to_string(input)?;

    // remove everything between don't() and do()
    let mut input = input;
    while let Some(dont) = input.find("don't()") {
        if let Some(doo) = input[dont..].find("do()") {
            input = format!("{}{}", &input[..dont], &input[dont+doo+4..]);
        } else {
            input = input[..dont].to_string();
        }
    }

    // parse all "mul(x" get index of x
    let mut candidates: Vec<String> = Vec::new();
    let mut start = 0;

    // collects all mul(*) strings
    while let Some(candidate) = input[start..].find("mul(") {
        start += candidate;
        if let Some(end) = input[start..].find(')') {
            candidates.push(input[start..=start + end].to_string());
        } else {
            break;
        }
        start += 4;
    }

    // go through candidates remove any not containing only numbers and a comma
    candidates = candidates.iter()
        .map(|x| x[4..x.len()-1].to_string()) // get the part enclosed in ()
        .filter(|x| x.len() >= 3)
        .filter(|x| x.chars().all(|c| "0123456789,".contains(c)))
        .filter(|x| x.chars().filter(|y| *y == ',').count() == 1)
        .filter(|x| { 
            let comma_pos = x.find(',').unwrap();
            comma_pos < x.len() - 1 && comma_pos > 0 
        })
        .collect();

    // split at comma and apply multiplication
    let result = candidates.iter()
        .map(|x| x.split(',').flat_map(|y| y.parse::<usize>()).collect::<Vec<usize>>())
        .map(|x| x[0] * x[1])
        .sum::<usize>();

    Ok(result)
}

fn main() {
    println!("Task1: {:?}", solve_task_1("input.txt"));
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
