#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();
    for (_index, element) in inputs.iter().enumerate() {
        match element {
            &CalculatorInput::Value(number) => stack.push(number),
            &CalculatorInput::Add => {
                let right = stack.pop();
                let left = stack.pop();
                if right == None || left == None {
                    return None;
                } else {
                    let total = left.unwrap() + right.unwrap();
                    stack.push(total)
                }
            }
            &CalculatorInput::Subtract => {
                let right = stack.pop();
                let left = stack.pop();
                if right == None || left == None {
                    return None;
                } else {
                    let total = left.unwrap() - right.unwrap();
                    stack.push(total)
                }
            }
            &CalculatorInput::Multiply => {
                let right = stack.pop();
                let left = stack.pop();
                if right == None || left == None {
                    return None;
                } else {
                    let total = left.unwrap() * right.unwrap();
                    stack.push(total)
                }
            }
            &CalculatorInput::Divide => {
                let right = stack.pop();
                let left = stack.pop();
                if right == None || left == None {
                    return None;
                } else {
                    let total = left.unwrap() / right.unwrap();
                    stack.push(total)
                }
            }
        }
    }
    let result = stack.pop();
    let remaining = stack.pop();
    if result != None && remaining == None {
        result
    } else {
        None
    }
}
