#![feature(exclusive_range_pattern)]

use std::io::{stdin, stdout};
use std::{io::Write, ops::AddAssign};
fn main() {
    let mut text: String = String::with_capacity(100);
    loop {
        print!("input> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut text).unwrap();
        let mut ex = Executor::new();
        ex.parse(&text).unwrap();
        println!("{:?}", ex);
        text.clear();
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Executor {
    stack: Vec<f64>,
    current_value: Option<f64>,
    decimal: Option<i32>,
}

impl Executor {
    pub fn new() -> Self {
        Self {
            stack: vec![],
            current_value: None,
            decimal: None,
        }
    }
    fn pop(&mut self) -> Option<f64> {
        self.stack.pop()
    }
    fn push(&mut self, value: f64) {
        self.stack.push(value);
    }
    fn set_decimal(&mut self) -> Result<(), String> {
        if let Some(current_decimal) = self.decimal {
            Err(format!(
                "decimal reset after being set to {}",
                current_decimal
            ))
        } else {
            self.decimal = Some(-1);
            Ok(())
        }
    }
    fn reset(&mut self) {
        if let Some(v) = self.current_value {
            *self += v;
        }
        self.current_value = None;
        self.decimal = None;
    }
    pub fn parse(&mut self, state: &str) -> Result<(), String> {
        for (index, ch) in state.chars().enumerate() {
            match ch {
                '0'..'9' => {
                    *self += (ch as u8) - b'0';
                    Ok(())
                }
                '.' => self.set_decimal(),
                '+' => self.apply_operation(|a, b| a + b),
                '-' => self.apply_operation(|a, b| a - b),
                '*' => self.apply_operation(|a, b| a * b),
                '/' => self.apply_operation(|a, b| a / b),
                '^' => self.apply_operation(|a, b| a.powf(b)),
                ' ' | '\t' | '\0' | '\n' => {
                    self.reset();
                    Ok(())
                }
                _ => Err(format!("unknown character: {}", ch)),
            }
            .map_err(|e| format!("Error at index {}: {}", index, e))?;
        }
        self.reset();
        Ok(())
    }
    fn apply_operation<F>(&mut self, operation: F) -> Result<(), String>
    where
        F: Fn(f64, f64) -> f64,
    {
        self.reset();
        if let Some(second_op) = self.pop() {
            if let Some(first_op) = self.pop() {
                *self += operation(first_op, second_op);
                return Ok(());
            }
        }
        Err(String::from("insufficient values to apply operation"))
    }
}

impl AddAssign<f64> for Executor {
    fn add_assign(&mut self, rhs: f64) {
        self.push(rhs);
    }
}
impl AddAssign<u8> for Executor {
    fn add_assign(&mut self, rhs: u8) {
        if let Some(current_value) = self.current_value {
            if let Some(num_decimal_places) = self.decimal {
                self.current_value =
                    Some(current_value + (rhs as f64) * 10.0f64.powi(num_decimal_places));
                self.decimal = Some(num_decimal_places - 1);
            } else {
                self.current_value = Some(current_value * 10.0 + (rhs as f64));
            }
        } else {
            self.current_value = Some(rhs as f64);
        }
    }
}
