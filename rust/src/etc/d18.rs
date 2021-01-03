use std::collections::HashMap;
use ExprToken::*;

pub type Expression = Vec<ExprToken>;

///////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum ExprToken {
    Sum, 
    Mult,
    LeftParens,
    Value {val: u64},
}

impl ExprToken {
    pub fn from(ch: char) -> Self {
        match ch {
            '*' => Mult,
            '+' => Sum,
             _  => panic!("Not supported!"),
        }
    }

    pub fn eval(&self, a: u64, b: u64) -> u64 {
        match &self {
            Mult => a * b,
             Sum => a + b,
               _ => panic!("Not supported!"),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

// Simplified version of the Shunting-Yard algorithm
pub fn line_to_expr(line: &str, op_priority: &HashMap<ExprToken, u8>) -> Expression {
    let mut output_q: Expression = Vec::new();
    let mut operator_stack: Expression = Vec::new();

    for ch in line.chars() {
        match ch {
            '+' | '*' => {
                let op = ExprToken::from(ch);
                let priority = op_priority[&op];

                while !operator_stack.is_empty() {
                    let top_op = operator_stack[operator_stack.len() - 1];
                    if top_op == LeftParens || op_priority[&top_op] < priority {
                        break;
                    }
                    output_q.push(operator_stack.pop().unwrap());
                }
                operator_stack.push(op);
            },
            '(' => operator_stack.push(LeftParens),
            ')' => {
                let mut op = operator_stack.pop().unwrap();
                while op != LeftParens {
                    output_q.push(op);
                    op = operator_stack.pop().unwrap();
                }
            },
             x  => output_q.push(Value{ val: x.to_digit(10).unwrap() as u64 }), // The character is a number
        }
    }

    while !operator_stack.is_empty() {
        output_q.push(operator_stack.pop().unwrap());
    }

    return output_q;
}

pub fn eval_expr(expr: &[ExprToken]) -> u64 {
    let mut stack = Vec::new();

    for token in expr {
        let val = match token {
            Value{val} => *val,
            _ => token.eval(stack.pop().unwrap(), stack.pop().unwrap()),
        };
        stack.push(val);
    }

    return stack[0];
}
