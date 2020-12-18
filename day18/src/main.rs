use std::io::{self, BufRead};
use std::error::Error;
use std::fs::File;

fn shunting_yard(input: &str) -> Vec<char> {
    /* Shunting yard algorithm. Produces output in RPN. i.e. converting notation from postfix operators 
       to infix operators. e.g "2 + 5" -> "2 5 +"
    */
    let mut operator_stack: Vec<char> = Vec::new();
    let mut output_queue: Vec<char> = Vec::new();
    
    for token in input.chars() {
        if token == ' ' {
            continue;
        } else if token.is_digit(10) {
            output_queue.push(token);
        } else if token == '+' {
            while operator_stack.last() == Some(&'+') {
                output_queue.push(operator_stack.pop().unwrap());
            }
            operator_stack.push(token);
        } else if token == '*' {
            while operator_stack.last() == Some(&'+') || operator_stack.last() == Some(&'*') {
                output_queue.push(operator_stack.pop().unwrap());
            }
            operator_stack.push(token);
        } else if token == '(' {
            operator_stack.push(token);
        } else if token == ')' {
            while operator_stack.last() != Some(&'(') {
                output_queue.push(operator_stack.pop().unwrap());
            }
            operator_stack.pop();
        } else {
            assert!(false);
        }
    }

    while let Some(op) = operator_stack.pop() {
        output_queue.push(op);
    }

    output_queue
}

fn evaluate_rpn(input: &Vec<char>) -> u64 {
    /* Evaluates statement in RPN notation */
    let mut stack: Vec<u64> = vec![];
    for token in input.into_iter() {
        match token {
            '*' => {
                let first: u64 = stack.pop().unwrap();
                let second: u64 = stack.pop().unwrap();
                stack.push(first * second);
            },
            '+' => {
                let first: u64 = stack.pop().unwrap();
                let second: u64 = stack.pop().unwrap();
                stack.push(first + second);
            },
            _ => {
                stack.push(token.to_digit(10).unwrap() as u64);
            },
        }
    }
    
    assert!(stack.len() == 1);
    return stack[0]
}

fn main() -> Result<(), Box<dyn Error>>{
    let f = File::open("inputs.txt")?;
    let lines = io::BufReader::new(f).lines().map(|line| line.unwrap());

    let res: u64 = lines.map(|line| evaluate_rpn(&shunting_yard(&line))).sum();
    println!("{}", res);

    Ok(())
}
