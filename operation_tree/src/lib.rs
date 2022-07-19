use eq_split::{eq_sanitize::EqSanitize, operators::Operators, EquationString, FullSplit};
use math_characters::{match_math_character, math_character_position};

mod math_characters;

/// with the higher precedence located at the bottom of the tree
/// we will do breath first search and they will be calculated first

#[derive(Debug)]
pub enum NodeValue {
    UnitValue(f64),
    OperationValue(OperationNode),
}

#[derive(Debug)]
pub struct OperationNode {
    pub operation: Operators,
    pub left_node: Box<NodeValue>,
    pub right_node: Box<NodeValue>,
}

impl OperationNode {
    pub fn new(eq: &str) -> Result<Self, String> {
        //to remove whitespaces
        let eq = EquationString::remove_whitespaces(eq);
        let eq_res = eq.handle_special_character_multiplication();
        if let Err(t) = eq_res {
            return Err(t);
        }

        let eq = eq_res.unwrap();

        let eq_res = eq.handle_direct_multiplication();
        if let Err(t) = eq_res {
            return Err(t);
        }

        let eq = eq_res.unwrap();
        return Self::create(eq);
    }

    fn create(eq: EquationString) -> Result<Self, String> {
        let split_res = eq.split();
        if let Err(t) = split_res {
            return Err(t);
        }

        let (left, right, operator) = split_res.unwrap();

        if operator == Operators::None {
            let mut left_float = 0.0;

            let math_character_position_res = math_character_position(left.clone());
            if let Some((index, operator)) = math_character_position_res {
                let left_len = left.len();
                let operator_str = operator.to_string();

                if index == 0 && left_len == 1 {
                    left_float = match_math_character(&operator_str).unwrap();
                } else if index == 0 && operator_str == "pi" {
                    left_float = match_math_character(&operator_str).unwrap();
                } else if index == 0 && left_len > 2 && operator_str == "pi" {
                    return Err(String::from("Invalid syntax"));
                } else if index == 0 && left_len > 1 {
                    return Err(String::from("Invalid syntax"));
                } else {
                    let num_str: EquationString = left[..index].to_vec();
                    let num_parse_res = num_str.to_string().parse::<f64>();
                    if num_parse_res.is_err() {
                        return Err(String::from("Incorrect number"));
                    }

                    let math_character_start: EquationString = left[index..].to_vec();
                    let math_characters_start_len = math_character_start.len();
                    if math_characters_start_len > 2 && operator_str == "pi" {
                        return Err(String::from("Invalid syntax"));
                    } else if math_characters_start_len > 1 && operator_str != "pi" {
                        return Err(String::from("Invalid syntax"));
                    }

                    left_float =
                        num_parse_res.unwrap() * match_math_character(&operator_str).unwrap();
                }
            } else {
                // handle for +5 or -5 or 1*-5 or 1+-5
                let left_parse_res = left.clone().to_string().parse::<f64>();
                if left_parse_res.is_err() {
                    return Err(String::from("Incorrect number"));
                }

                left_float = left_parse_res.unwrap();
            }

            return Ok(OperationNode {
                operation: Operators::None,
                left_node: Box::new(NodeValue::UnitValue(left_float)),
                right_node: Box::new(NodeValue::UnitValue(0.0)),
            });
        }

        let left_node_res = Self::create(left);
        if let Err(t) = left_node_res {
            return Err(t);
        }

        let right_node_res = Self::create(right);
        if let Err(t) = right_node_res {
            return Err(t);
        }

        let left_node = left_node_res.unwrap();
        let right_node = right_node_res.unwrap();

        Ok(OperationNode {
            operation: operator,
            left_node: Box::new(NodeValue::OperationValue(left_node)),
            right_node: Box::new(NodeValue::OperationValue(right_node)),
        })
    }

    pub fn calculate(&self) -> f64 {
        if self.operation == Operators::None {
            return if let NodeValue::UnitValue(i) = *self.left_node {
                self.operation.calculate(i, 0.0)
            } else {
                0.0
            };
        }

        let left_sum = match &*self.left_node {
            NodeValue::UnitValue(i) => i.to_owned(),
            NodeValue::OperationValue(node) => node.calculate(),
        };

        let right_sum = match &*self.right_node {
            NodeValue::UnitValue(i) => i.to_owned(),
            NodeValue::OperationValue(node) => node.calculate(),
        };

        self.operation.calculate(left_sum, right_sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_create_operation_node() {
        let test_eq = "1";
        let operation_node = OperationNode::new(test_eq).unwrap();
        assert_eq!(operation_node.operation, Operators::None);
        assert_eq!(operation_node.calculate(), 1.0);

        let test_eq = "5/2*2";
        let operation_node = OperationNode::new(test_eq).unwrap();
        println!("operation_node: {:?}", operation_node.calculate());
        assert_eq!(operation_node.calculate(), 5.0);

        let test_eq = "0.1+(2+3)*5/3*2+((5+2)+2)";
        let operation_node = OperationNode::new(test_eq).unwrap();
        assert_eq!(operation_node.calculate(), 25.76666666666667);
        // println!("operation_node: {:?}", operation_node.calculate());

        let test_eq = "1+2*3*(1+2)";
        let operation_node = OperationNode::new(test_eq).unwrap();
        assert_eq!(operation_node.calculate(), 19.0);

        let test_eq = "0.1+5*(2+3)+((5+2)+2)";
        let operation_node = OperationNode::new(test_eq).unwrap();
        assert_eq!(operation_node.calculate(), 34.1);

        let test_eq = "(1+3)/5*2+3";
        let operation_node = OperationNode::new(test_eq).unwrap();
        assert_eq!(operation_node.calculate(), 4.6);
    }

    #[test]
    pub fn test_invalid_eq() {
        let eq = "*1+5";
        assert!(OperationNode::new(eq).is_err());

        let eq = "--1";
        assert!(OperationNode::new(eq).is_err());

        let eq = "(1343+3";
        assert!(OperationNode::new(eq).is_err());

        let eq = "1343=3";
        assert!(OperationNode::new(eq).is_err());

        let eq = "1343x3x";
        assert!(OperationNode::new(eq).is_err());
    }

    #[test]
    pub fn test_math_character() {
        let eq = "1+e";
        let operation_node = OperationNode::new(eq).unwrap();
        assert_eq!(operation_node.calculate(), 3.718281828459045);

        let eq = "5e";
        let operation_node = OperationNode::new(eq).unwrap();
        assert_eq!(operation_node.calculate(), 13.591409142295225);

        let eq = "5pi(5+2)/4";
        let operation_node = OperationNode::new(eq).unwrap();
        assert_eq!(operation_node.calculate(), 27.48893571891069);

        let eq = "(1+2)*5π(5+2)/4";
        let operation_node = OperationNode::new(eq).unwrap();
        assert_eq!(operation_node.calculate(), 82.46680715673206);

        let eq = "e^2";
        let operation_node = OperationNode::new(eq).unwrap();
        assert_eq!(operation_node.calculate(), 7.3890560989306495);

        let eq = "2e^2";
        let operation_node = OperationNode::new(eq).unwrap();
        assert_eq!(operation_node.calculate(), 14.778112197861299);

        let eq = "5e2";
        assert!(OperationNode::new(eq).is_err());
    }

    #[test]
    pub fn test_starting_with_plus_or_minus() {
        let test_eq = "-(1+2)";
        let operation_node = OperationNode::new(test_eq).unwrap();
        assert_eq!(operation_node.calculate(), -3.0);

        let test_eq = "1+-(1+2)";
        let operation_node = OperationNode::new(test_eq).unwrap();
        assert_eq!(operation_node.calculate(), -2.0);

        let test_eq = "-0.5++1";
        let operation_node = OperationNode::new(test_eq).unwrap();
        assert_eq!(operation_node.calculate(), 0.5);

        let test_eq = "-0.5+-1";
        let operation_node = OperationNode::new(test_eq).unwrap();
        assert_eq!(operation_node.calculate(), -1.5);

        let test_eq = "-0.5+(-1+5)";
        let operation_node = OperationNode::new(test_eq).unwrap();
        assert_eq!(operation_node.calculate(), 3.5);

        let test_eq = "-0.5+5*-(-1+5)";
        let operation_node = OperationNode::new(test_eq).unwrap();
        assert_eq!(operation_node.calculate(), -20.5);

        let test_eq = "-0.5+5*(-1+5)";
        let operation_node = OperationNode::new(test_eq).unwrap();
        assert_eq!(operation_node.calculate(), 19.5);

        let test_eq = "(1+3)(5+34)(5+3341)";
        let operation_node = OperationNode::new(test_eq).unwrap();
        assert_eq!(operation_node.calculate(), 521976.0);

        let test_eq = "-(1+3)(5+34)(5+3341)";
        let operation_node = OperationNode::new(test_eq).unwrap();
        assert_eq!(operation_node.calculate(), -521976.0);

        let test_eq = "(-0.5+5)(-1+5)";
        let operation_node = OperationNode::new(test_eq).unwrap();
        assert_eq!(operation_node.calculate(), 18.0);

        let test_eq = "(-0.5+5)(-1+5)+5";
        let operation_node = OperationNode::new(test_eq).unwrap();
        assert_eq!(operation_node.calculate(), 23.0);

        let test_eq = "(-0.5+5)(-1+5)-5";
        let operation_node = OperationNode::new(test_eq).unwrap();
        assert_eq!(operation_node.calculate(), 13.0);

        let test_eq = "-(-0.5+5)(-1+5)5";
        let operation_node = OperationNode::new(test_eq).unwrap();
        assert_eq!(operation_node.calculate(), -90.0);

        let test_eq = "5(-0.5+5)(-1+5)";
        let operation_node = OperationNode::new(test_eq).unwrap();
        assert_eq!(operation_node.calculate(), 90.0);
    }

    #[test]
    pub fn test_more() {
        let eq = "2(e+2)^π*2+-((5+7/2)-3^pi)";
        let operation_node = OperationNode::new(eq).unwrap();
        assert_eq!(operation_node.calculate(), 546.4210876660936);
    }
}

