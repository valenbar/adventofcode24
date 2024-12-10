#![allow(dead_code)]
#![allow(unused_variables)]
use std::{collections::HashMap, fs, usize};
use eyre::Result;

static RESULT1_DEMO: usize = 143;
static RESULT1_REAL: usize = 5747;
static RESULT2_DEMO: usize = 123;
static RESULT2_REAL: usize = 5502;

fn all_rules_obeyed(rules: &[(usize,usize)], page: &[usize]) -> bool {
    rules.iter().all(|x| rule_obeyed(x, page))
}

fn rule_obeyed(rule: &(usize, usize), page: &[usize]) -> bool {
    if let Some(first) = page.iter().position(|&x| x == rule.0) {
        if let Some(second) = page.iter().position(|&x| x == rule.1) {
            return first < second
        }
    } 
    true
}

fn solve_task_1(input: &str) -> Result<usize> {
    let input = fs::read_to_string(input)?;

    let cut = input.lines().enumerate().find(|(i, line)| line.trim().is_empty()).unwrap().0;
    let lines: Vec<String> = input.lines().map(|l| l.to_owned()).collect();
    let rules = &lines[0..cut];
    let pages = &lines[cut+1..];

    let rules: Vec<(usize,usize)> = rules.iter()
        .map(|s| s.split_once('|').unwrap())
        .map(|(x,y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
        .collect();

    let pages: Vec<Vec<usize>> = pages.iter()
        .map(|s| 
            s.split(',').map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>()
        ).collect();
    
    let mut result = 0;

    for page in pages {
        if rules.iter().all(|r| rule_obeyed(r, &page)) {
            let mid = page.len() / 2;
            result += page[mid];
        }
    }
    

    Ok(result)
}

fn solve_task_2(input: &str) -> Result<usize> {
    let input = fs::read_to_string(input)?;

    let cut = input.lines().enumerate().find(|(i, line)| line.trim().is_empty()).unwrap().0;
    let lines: Vec<String> = input.lines().map(|l| l.to_owned()).collect();
    let rules = &lines[0..cut];
    let pages = &lines[cut+1..];

    let rules: Vec<(usize,usize)> = rules.iter()
        .map(|s| s.split_once('|').unwrap())
        .map(|(x,y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
        .collect();

    let pages: Vec<Vec<usize>> = pages.iter()
        .map(|s| 
            s.split(',').map(|n| n.parse::<usize>().unwrap()).collect::<Vec<usize>>()
        ).collect();
    
    let mut result = 0;

    for page in pages {
        let rules = &rules;
        if !all_rules_obeyed(rules, &page) {

            let page_rules: Vec<(usize,usize)> = {
                let mut v: Vec<(usize, usize)> = vec![];
                for rule in rules {
                    if page.contains(&rule.0) && page.contains(&rule.1) {
                        v.push(*rule);
                    }
                }
                v
            };

            let mut map: HashMap<usize, usize> = page.iter().map(|x| (*x, 0)).collect();
            for rule in rules {
                if page.contains(&rule.0) && page.contains(&rule.1) {
                    if let Some(x) = map.get_mut(&rule.0) {
                        *x += 1;
                    }
                }
            }
            result += map.iter().find(|(&k, &v)| v == page.len() / 2).unwrap().0;
        }
    }
    

    Ok(result)
}

fn main() {
    println!("Task1: {:?}", solve_task_1("input_demo.txt"));
    println!("Task2: {:?}", solve_task_2("input_demo.txt"));
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
