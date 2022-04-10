use std::option;

#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}


fn tpop(n2:&mut i32, n1:&mut i32, stack: &mut Vec<CalculatorInput>) -> Option<i32> {
    if stack.len() < 2 {
        return None;
    }
    match stack.pop()? {
        CalculatorInput::Value(v) => {
            *n2 = v;
        }
        _ => {
            return None;
        }
    }
    match stack.pop()? {
        CalculatorInput::Value(v) => {
            *n1 = v;
        }
        _ => {
            return None;
        }
    }
    Some(0)
}

fn calc(stack: &Vec<CalculatorInput>, input: &CalculatorInput) -> Option<i32> {
    let mut n1;
    let mut n2;
    match input {
        CalculatorInput::Add => {
            tpop(n2, n1, &mut stack)?;
            stack.push(CalculatorInput::Value(n1 + n2));
        }
        CalculatorInput::Subtract => {
            tpop(n1, n2, &mut stack)?;
            stack.push(CalculatorInput::Value(*n1 - *n2));
        }
        CalculatorInput::Multiply => {
            tpop(n1, n2, &mut stack)?;
            stack.push(CalculatorInput::Value(*n1 * *n2));
        }
        CalculatorInput::Divide => {
            tpop(n1, n2, &mut stack)?;
            stack.push(CalculatorInput::Value(*n1 / *n2));
        }
        CalculatorInput::Value(v) => {
            stack.push(CalculatorInput::Value(*v));
        }
    }
    None
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<CalculatorInput> = vec![];
    for input in inputs {
        calc(&mut stack, input)?;
    }
    match stack.len() {
        1 => {
            match stack.pop() {
                Some(CalculatorInput::Value(v)) => {
                    Some(v)
                }
                _ => None
            }
        }
        _ => {
            return None;
        }
    }
}

#[cfg(test)]
fn calculator_input(s: &str) -> Vec<CalculatorInput> {
    s.split_whitespace()
        .map(|s| match s {
            "+" => CalculatorInput::Add,
            "-" => CalculatorInput::Subtract,
            "*" => CalculatorInput::Multiply,
            "/" => CalculatorInput::Divide,
            n => CalculatorInput::Value(n.parse().unwrap()),
        })
        .collect()
}

#[test]
fn test_empty_input_returns_none() {
    let input = calculator_input("");
    assert_eq!(evaluate(&input), None);
}

#[test]
fn test_simple_value() {
    let input = calculator_input("10");
    assert_eq!(evaluate(&input), Some(10));
}

#[test]
fn test_simple_addition() {
    let input = calculator_input("2 2 +");
    assert_eq!(evaluate(&input), Some(4));
}

#[test]
fn test_simple_subtraction() {
    let input = calculator_input("7 11 -");
    assert_eq!(evaluate(&input), Some(-4));
}

#[test]
fn test_simple_multiplication() {
    let input = calculator_input("6 9 *");
    assert_eq!(evaluate(&input), Some(54));
}

#[test]
fn test_simple_division() {
    let input = calculator_input("57 19 /");
    assert_eq!(evaluate(&input), Some(3));
}

#[test]
fn test_complex_operation() {
    let input = calculator_input("4 8 + 7 5 - /");
    assert_eq!(evaluate(&input), Some(6));
}

#[test]
fn test_too_few_operands_returns_none() {
    let input = calculator_input("2 +");
    assert_eq!(evaluate(&input), None);
}

#[test]
fn test_too_many_operands_returns_none() {
    let input = calculator_input("2 2");
    assert_eq!(evaluate(&input), None);
}

#[test]
fn test_zero_operands_returns_none() {
    let input = calculator_input("+");
    assert_eq!(evaluate(&input), None);
}

#[test]
fn test_intermediate_error_returns_none() {
    let input = calculator_input("+ 2 2 *");
    assert_eq!(evaluate(&input), None);
}