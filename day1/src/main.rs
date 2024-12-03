#![allow(dead_code)]
#![allow(unused_variables)]
use std::{collections::HashMap, fs, iter::zip};
use eyre::Result;


fn solve_task_1(input: &str) -> Result<u32> {
    // split input into two lists
    let input = fs::read_to_string(input)?;
    let (mut a, mut b): (Vec<_>, Vec<_>) = input.lines()
        .map(|line| 
            line.split_once("   ")
            .unwrap()
        )
        .map(|(l,r)| 
            (
                l.parse::<i32>().unwrap(),
                r.parse::<i32>().unwrap()
            )
        ).unzip();
    
    // sort both lists
    a.sort();
    b.sort();

    // zip, difference, sum up
    let result = zip(a, b).map(|(a,b)| a.abs_diff(b)).sum::<u32>();

    println!("{:?}", result);

    Ok(result)
}

fn solve_task_2(input: &str) -> Result<i32> {
    // split input into two lists
    let input = fs::read_to_string(input)?;
    let (a, b): (Vec<_>, Vec<_>) = input.lines()
        .map(|line| 
            line.split_once("   ")
            .unwrap()
        )
        .map(|(l,r)| 
            (
                l.parse::<i32>().unwrap(),
                r.parse::<i32>().unwrap()
            )
        ).unzip();

    // init zero map from entries in a
    let mut m = HashMap::new();
    for k in a.iter() {
        m.insert(*k, 0);
    }

    // count number occurences in b
    for k in b.iter() {
        if let Some(v) = m.get_mut(k) {
            *v += 1;
        }
    }

    let result = a.iter().map(|x| m.get(x).unwrap() * x).sum();

    println!("{:?}", result);

        Ok(result)
}

fn main() {
    let _ = solve_task_1("input_demo.txt");
}

#[test]
fn test_task_1_demo() -> Result<()> {
    let result = solve_task_1("input_demo.txt")?;
    assert_eq!(result, 11);
    Ok(())
}

#[test]
fn test_task_1_real() -> Result<()> {
    let result = solve_task_1("input.txt")?;
    assert_eq!(result, 1651298);
    Ok(())
}

#[test]
fn test_task_2_demo() -> Result<()> {
    let result = solve_task_2("input_demo.txt")?;
    assert_eq!(result, 31);
    Ok(())
}

#[test]
fn test_task_2_real() -> Result<()> {
    let result = solve_task_2("input.txt")?;
    assert_eq!(result, 21306195);
    Ok(())
}
