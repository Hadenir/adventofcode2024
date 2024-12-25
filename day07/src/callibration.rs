#[derive(Debug)]
pub struct Equation {
    pub test_value: i64,
    pub operands: Vec<i64>,
}

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Add,
    Multiply,
    Concatenate,
}

impl Operator {
    pub fn evaluate(self, a: i64, b: i64) -> i64 {
        use Operator::*;
        match self {
            Add => a + b,
            Multiply => a * b,
            Concatenate => format!("{a}{b}")
                .parse()
                .expect("Concatenation of two numbers must be a number"),
        }
    }
}
