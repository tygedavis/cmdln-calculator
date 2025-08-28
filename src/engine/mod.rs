// Needed for running unit tests
#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide
}

// Needed for running unit tests
#[derive(Debug, PartialEq)]
pub struct Calculation {
    pub left_operand: i32,
    pub operator: Operator,
    pub right_operand: i32
}

// "Skinny arrow" -> specifies the return type of the parse() function.
// This means that on success, parse() will return a Result, which contains
// a Calculation on success and on a failure will contain a String
pub fn parse(input: &str) -> Result<Calculation, String> {
    let mut found_index: Option<usize> = None;
    let mut found_operator: Option<Operator> = None;
    
    /*
    `index` is a temporary value that is only scoped within the
    if statement. That is why a new variable `index` must be
    declared with every if statement.
     */
    if let Some(index) = input.find('+') {
        found_index = Some(index);
        found_operator = Some(Operator::Add);
    } else if let Some(index) = input.find('-') {
        found_index = Some(index);
        found_operator = Some(Operator::Subtract);
    } else if let Some(index) = input.find('*') {
        found_index = Some(index);
        found_operator = Some(Operator::Multiply);
    } else if let Some(index) = input.find('/') {
        found_index = Some(index);
        found_operator = Some(Operator::Divide);
    }

    let the_index = match found_index {
        // This is saying that if there is something in `found_index`
        // grab that something and assign it to an `index` variable.
        // Then, return the `index` variable
        Some(index) => index,
        None => {
            return Err(String::from("No operator provided"))
        }
    };

    // `&` creates a reference, allowing us to use the original `input`
    // data without needing to copy it
    let left_side = &input[..the_index];
    let right_side = &input[the_index + 1..];

    let left_operand = match left_side.trim().parse::<i32>() {
        Ok(num) => num,
        // `_` is a wildcard that matches an Err containing any value
        Err(_) => return Err(String::from("There was an error parsing the left operand"))
    };

    let right_operand = match right_side.trim().parse::<i32>() {
        Ok(num) => num,
        Err(_) => return Err(String::from("There was an error parsing the right operand"))
    };

    let calculation = Calculation {
        left_operand: left_operand,
        operator: found_operator.unwrap(),
        right_operand: right_operand
    };

    // This is a successful return value, as indicated by the
    // Ok() method (success) and no semicolon (return value)
    Ok(calculation)
}

pub fn evaluate(calculation: &Calculation) -> Result<i32, String> {
    match calculation.operator {
        Operator::Add => Ok(calculation.left_operand + calculation.right_operand),
        Operator::Subtract => Ok(calculation.left_operand - calculation.right_operand),
        Operator::Multiply => Ok(calculation.left_operand * calculation.right_operand),
        Operator::Divide => {
            if calculation.right_operand == 0 {
                Err(String::from("Cannot divide a number by zero"))
            } else {
                Ok(calculation.left_operand / calculation.right_operand)
            }

        }
    }
}

// The #[cfg(test)] attribute tells Rust to only compile this module (tests) when testing.
#[cfg(test)]
mod tests;
