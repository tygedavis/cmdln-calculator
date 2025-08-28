use super::*;

mod parse_tests {
    use super::*;

    #[test]
    fn test_simple_addition() {
        let res = parse("10 + 5").unwrap();
    
        assert_eq!(res.left_operand, 10);
        assert_eq!(res.right_operand, 5);
        assert_eq!(res.operator, Operator::Add);
    }
    
    #[test]
    fn test_no_whitespace() {
        let res = parse("7-3").unwrap();
    
        assert_eq!(res.left_operand, 7);
        assert_eq!(res.right_operand, 3);
        assert_eq!(res.operator, Operator::Subtract);
    }
    
    #[test]
    fn test_no_operator() {
        let res = parse("4 0");
    
        if let Err(message) = res {
            assert_eq!(message, "No operator provided");
        } else {
            // If the result was `Ok`, fail the test
            panic!("The test should have failed but it returned Ok");
        }
    }
    
    #[test]
    fn test_no_left_operand() {
        let res = parse("- 0");
    
        if let Err(message) = res {
            assert_eq!(message, "There was an error parsing the left operand");
        } else {
            // If the result was `Ok`, fail the test
            panic!("The test should have failed but it returned Ok");
        }
    }
    
    #[test]
    fn test_no_right_operand() {
        let res = parse("0+");
    
        if let Err(message) = res {
            assert_eq!(message, "There was an error parsing the right operand");
        } else {
            // If the result was `Ok`, fail the test
            panic!("The test should have failed but it returned Ok");
        }
    }
}

mod evaluate_tests {
    use super::*;

    #[test]
    fn test_addition() {
        let calculation = &Calculation {
            left_operand: 5,
            operator: Operator::Add,
            right_operand: 3
        };

        let res = evaluate(calculation);

        assert_eq!(res.unwrap(), 8);
    }

    #[test]
    fn test_subtraction() {
        let calculation = &Calculation {
            left_operand: 5,
            operator: Operator::Subtract,
            right_operand: 3
        };

        let res = evaluate(calculation);

        assert_eq!(res.unwrap(), 2);
    }

    #[test]
    fn test_multiplication() {
        let calculation = &Calculation {
            left_operand: 5,
            operator: Operator::Multiply,
            right_operand: 3
        };

        let res = evaluate(calculation);

        assert_eq!(res.unwrap(), 15);
    }

    #[test]
    fn test_division() {
        let calculation = &Calculation {
            left_operand: 10,
            operator: Operator::Divide,
            right_operand: 5
        };

        let res = evaluate(calculation);

        assert_eq!(res.unwrap(), 2);
    }

    #[test]
    fn test_division_by_zero() {
        let calculation = &Calculation {
            left_operand: 10,
            operator: Operator::Divide,
            right_operand: 0
        };

        let res = evaluate(calculation);

        if let Err(message) = res {
            assert_eq!(message, "Cannot divide a number by zero");
        } else {
            // If the result was `Ok`, fail the test
            panic!("The test should have failed but it returned Ok");
        }
    }
}
