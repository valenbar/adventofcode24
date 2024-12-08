#![allow(dead_code)]
#![allow(unused_variables)]
use std::fs;
use eyre::Result;

static RESULT1_DEMO: usize = 18;
static RESULT1_REAL: usize = 2543;
static RESULT2_DEMO: usize = 9;
static RESULT2_REAL: usize = 1930;


fn solve_task_1(input: &str) -> Result<usize> {
    let input = fs::read_to_string(input)?;

    
    let mat = input.lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let (mat_x,mat_y) = (mat.len(), mat[0].len());

    let mut candidates_x: Vec<(usize,usize)> = Vec::new();
    
    for (row_i, row) in mat.iter().enumerate() {
        for (col_i, letter) in row.iter().enumerate() {
            if *letter == 'X' {
                candidates_x.push((row_i, col_i));
            }
        }
    }

    let mut result = 0;

    for (row_i, col_i) in candidates_x {
        let space_up = row_i >= 3;
        let space_left = col_i >= 3;
        let space_right = col_i + 3 < mat_y;
        let space_down = row_i + 3 < mat_x;

        if space_right && [ 
                mat[row_i][col_i],
                mat[row_i][col_i+1],
                mat[row_i][col_i+2],
                mat[row_i][col_i+3],
            ] == ['X','M','A','S'] {
            result += 1;
        }

        if space_left && [ 
                mat[row_i][col_i],
                mat[row_i][col_i-1],
                mat[row_i][col_i-2],
                mat[row_i][col_i-3],
            ] == ['X','M','A','S'] {
            result += 1;
        }

        if space_up && [ 
                mat[row_i][col_i],
                mat[row_i-1][col_i],
                mat[row_i-2][col_i],
                mat[row_i-3][col_i],
            ] == ['X','M','A','S'] {
            result += 1;
        }

        if space_down && [ 
                mat[row_i][col_i],
                mat[row_i+1][col_i],
                mat[row_i+2][col_i],
                mat[row_i+3][col_i],
            ] == ['X','M','A','S'] {
            result += 1;
        }

        if space_up && space_left && [ 
                mat[row_i][col_i],
                mat[row_i-1][col_i-1],
                mat[row_i-2][col_i-2],
                mat[row_i-3][col_i-3],
            ] == ['X','M','A','S'] {
            result += 1;
        }

        if space_up && space_right && [ 
                mat[row_i][col_i],
                mat[row_i-1][col_i+1],
                mat[row_i-2][col_i+2],
                mat[row_i-3][col_i+3],
            ] == ['X','M','A','S'] {
            result += 1;
        }

        if space_down && space_left && [ 
                mat[row_i][col_i],
                mat[row_i+1][col_i-1],
                mat[row_i+2][col_i-2],
                mat[row_i+3][col_i-3],
            ] == ['X','M','A','S'] {
            result += 1;
        }

        if space_down && space_right && [ 
                mat[row_i][col_i],
                mat[row_i+1][col_i+1],
                mat[row_i+2][col_i+2],
                mat[row_i+3][col_i+3],
            ] == ['X','M','A','S'] {
            result += 1;
        }

    }

    Ok(result)
}

fn solve_task_2(input: &str) -> Result<usize> {
    let input = fs::read_to_string(input)?;

    
    let mat = input.lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let (mat_x,mat_y) = (mat.len(), mat[0].len());

    let mut result = 0;

    for (row_i, row) in mat.iter().enumerate() {
        if row_i == 0 || row_i + 1 == mat_x { continue }
        for (col_i, letter) in row.iter().enumerate() {
            if col_i == 0 || col_i + 1 == mat_x { continue }
            if *letter == 'A' && [['M','S','M','S'],['M','S','S','M'],['S','M','M','S'],['S','M','S','M']].contains(&[ 
                    mat[row_i-1][col_i-1],
                    mat[row_i+1][col_i+1],
                    mat[row_i-1][col_i+1],
                    mat[row_i+1][col_i-1],
                ]) {
                result += 1;
            }
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
