#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack = Vec::new();
    for i in 0..inputs.len() {
        let input = &inputs[i];
        match input {
            CalculatorInput::Value(value) => {
                stack.push(*value);
            }
            CalculatorInput::Add => {
                let a = match stack.pop() {
                    Some(a) => a,
                    _ => return None,
                };
                let b = match stack.pop() {
                    Some(a) => a,
                    _ => return None,
                };
                stack.push(a + b);
            }
            CalculatorInput::Subtract => {
                let a = match stack.pop() {
                    Some(a) => a,
                    _ => return None,
                };
                let b = match stack.pop() {
                    Some(a) => a,
                    _ => return None,
                };
                stack.push(b - a);
            }
            CalculatorInput::Multiply => {
                let a = match stack.pop() {
                    Some(a) => a,
                    _ => return None,
                };
                let b = match stack.pop() {
                    Some(a) => a,
                    _ => return None,
                };
                stack.push(a * b);
            }
            CalculatorInput::Divide => {
                let a = match stack.pop() {
                    Some(a) => a,
                    _ => return None,
                };
                let b = match stack.pop() {
                    Some(a) => a,
                    _ => return None,
                };
                stack.push(b / a);
            }
        }
    }
    if stack.len() == 1 {
        return stack.pop();
    } else {
        return None;
    }
}
