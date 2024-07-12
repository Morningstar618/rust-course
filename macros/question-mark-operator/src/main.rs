//--------------------------------------------
//          Question Mark Operator
//--------------------------------------------

use std::num::ParseIntError;

fn parse_str(input: &str) -> Result<i32, ParseIntError> {
    // The `?` operator below will return an error if the string cannot be parsed as a valid i32. Else
    // it will unwrap the value from the Result enum.
    let integer = input.parse::<i32>()?;

    println!("The value is {:?} is integer {:?}", input, integer);
    Ok(integer)
}

fn divisor(divident: f64, divisor: f64) -> Option<f64> {
    let answer = match divisor {
        0.0 => None,
        _ => Some(divident / divisor),
    };

    let correct = answer?;
    /*
    If one of the variants of the Result enum is `heap-allocated`, then the `?` operator will pass the
    ownership to the variable assigned to it, i.e. in this case, the `correct` value.

    The point to remember is that the question mark will take the ownership of the values in connection
    with the result enum if it happens to be heap allocated.
    */
    // println!("{:?}", answer);

    println!(
        "This line will note be printed in case of an error {:?}",
        correct
    );
    Some(correct)
}

#[derive(Debug)]
enum MathError {
    DivisionError_DivisionByZero,
    LogError_NonPositiveLogrithm,
    SqrtError_NegativeSquareRoot,
}

type MathResult = Result<(), MathError>;

fn division(x: f64, y: f64) -> MathResult {
    if y == 0.0 {
        Err(MathError::DivisionError_DivisionByZero)
    } else {
        println!("Division value: {}", x / y);
        Ok(())
    }
}

fn sqrt(x: f64) -> MathResult {
    if x < 0.0 {
        Err(MathError::SqrtError_NegativeSquareRoot)
    } else {
        println!("Square Root value: {}", x.sqrt());
        Ok(())
    }
}

fn ln(x: f64) -> MathResult {
    if x <= 0.0 {
        Err(MathError::LogError_NonPositiveLogrithm)
    } else {
        println!("Log value: {}", x.ln());
        Ok(())
    }
}

fn operations(x: f64, y: f64) -> MathResult {
    division(x, y)?;
    sqrt(x)?;
    ln(x)?;
    Ok(())
}

fn main() {
    let some_values = vec!["123", "some1", "some(123)", "abc", "53"];
    for value in some_values {
        println!("{:?}", parse_str(value));
    }

    println!();

    println!(
        "Call from main with result equals to {:?}",
        divisor(9.0, 3.0)
    );
    println!(
        "Call from main with result equals to {:?}",
        divisor(9.0, 0.0)
    );
    println!(
        "Call from main with result equals to {:?}",
        divisor(0.0, 3.0)
    );

    println!();

    let result = operations(0.0, 10.0);
    if result.is_ok() {
        println!("All operations executed successfully.");
    } else {
        println!("{:?}", result);
    }
}
