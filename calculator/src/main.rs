use std::io;

enum Operator { 
    Add,
    Substract,
    Multiply,
    Divide,
}

struct Calculation { 
    left : f64,
    operator: Operator,
    right : f64,
}

impl Calculation { 
    fn new(left: f64, operator: Operator, right: f64) -> Calculation {
        Calculation { left, operator, right}
    }

    fn calculate(&self) -> Option<f64> {
        match self.operator { 
            Operator::Add => Some(self.left + self.right),
            Operator::Substract => Some(self.left - self.right),
            Operator::Multiply => Some(self.left * self.right),
            Operator::Divide => { 
                if self.right == 0.0 {
                    None
                } else {
                    Some(self.left / self.right)
                }
                }
            }
        }
    }

fn parse_operator(op: &str) -> Option<Operator> { 
    match op { 
        "+ " => Some(Operator::Add) , 
        "-" => Some(Operator::Substract),
        "*" => Some(Operator::Multiply),
        "/" => Some(Operator::Divide),
        _ => None,
    }
}

fn main() {
    println!("Simple Calculator");
    println!("Usage: 10 + 5");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.len() != 3 { 
        println!("Invalid format! Example: 10 +5");
        return;
    }

    let left: f64 = match parts[0].parse() { 
        Ok(n) => n,
        Err(_) => { println!( "Invalid number!"); return; }
    };

    let operator = match parse_operator(parts[1]) {
        Some(op) => op,
        None => { println!( "Invalid operator!"); return; }
    };

    let right: f64 = match parts[2].parse() {
        Ok(n) => n,
        Err(_) => { println!("Invalid number!"); return; }
    };

    let calc = Calculation::new(left, operator, right);

    match calc.calculate() { 
        Some(result) => println!("Result: {result}"),
        None => println!("Error: division by zero!"),
    }
}