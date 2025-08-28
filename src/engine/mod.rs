pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide
}

pub struct Calculation {
    pub left_operand: i32,
    pub operator: Operator,
    pub right_operand: i32
}

pub fn parse(input: &str) -> Result<Calculation, String> {
    let mut found_index: Option<usize> = None;
    let mut found_operator: Option<Operator> = None;
    
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
        Some(index) => index,
        None => {
            return Err(String::from("No operator provided"))
        }
    };

    let left_side = &input[..the_index];
    let right_side = &input[the_index + 1..];

    let left_operand = match left_side.trim().parse::<i32>() {
        Ok(num) => num,
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

    Ok(calculation)
}