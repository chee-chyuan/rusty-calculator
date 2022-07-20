use operators::{Operators, SplitOperator};
use parentheses::{FindParentheses, ParenthesesFinder, SplitParentheses};

pub mod eq_sanitize;
pub mod operators;
mod parentheses;
mod precedences;
mod math_characters;

pub type EquationString = Vec<char>;

pub trait FullSplit {
    fn split(&self) -> Result<(Self, Self, Operators), String>
    where
        Self: Sized;
}

impl FullSplit for EquationString {
    /// split equation to a two halves, a left and a right
    /// callee will need to reiterative call this function
    /// to ensure all nodes are single value only
    fn split(&self) -> Result<(Self, Self, Operators), String>
    where
        Self: Sized,
    {
        let first_parentheses = ParenthesesFinder::find_first((&self).to_vec());
        if let Err(t) = first_parentheses {
            return Err(t);
        }
        let first_parentheses = first_parentheses.unwrap();

        if first_parentheses.is_none() {
            // eq has no parentheses
            // normal eq eg.1+3*5
            // has no operators eg. 578

            return Ok(self.split_by_precedence());
        }
        // eq has parentheses
        let (left_parentheses_index, right_parentheses_index) = first_parentheses.unwrap();

        if left_parentheses_index == 0 && right_parentheses_index == self.len() - 1 {
            // parentheses contains the entire eq eg. (1+3*5)
            // remove the outer most parentheses and call 'split' recursively
            // return split(&eq[1..eq.len() - 1]);
            let eq = self[1..self.len() - 1].to_vec();
            return eq.split();
        }

        if left_parentheses_index == 1 && right_parentheses_index == self.len() - 1 {
            // parentheses contains the entire eq except for a negative operator eg. -(1+3*5)
            // remove the outer most parentheses and call 'split' recursively
            // return split(&eq[1..eq.len() - 1]);
            if self[0] == '-' {
                let left = vec!['-', '1'];
                let right = self[1..].to_vec();
                let operators = Operators::Mult;

                return Ok((left, right, operators));
            } else {
                return Err(String::from("Invalid operator in front of bracket"));
            }
        }

        // we split the eq into two halves according to the parentheses
        let (left, right) =
            self.split_by_parentheses(left_parentheses_index, right_parentheses_index);
        let operators = Operators::to_enum(right[0]);
        let right = right[1..].to_vec();

        Ok((left, right, operators))
    }
}

#[cfg(test)]
mod tests {
    use crate::eq_sanitize::EqSanitize;

    use super::*;

    #[test]
    pub fn test_split_no_parentheses() {
        let eq = "1+-2*2+5";
        let eq = EquationString::remove_whitespaces(eq);
        let (left, right, operator) = eq.split().unwrap();
        assert_eq!(left.to_string(), "1");
        assert_eq!(right.to_string(), "-2*2+5");
        assert_eq!(operator, Operators::Plus);

        let eq = "-2*2-5";
        let eq = EquationString::remove_whitespaces(eq);
        let (left, right, operator) = eq.split().unwrap();
        assert_eq!(left.to_string(), "-2*2");
        assert_eq!(right.to_string(), "5");
        assert_eq!(operator, Operators::Minus);

        let eq = "-2*2^5";
        let eq = EquationString::remove_whitespaces(eq);
        let (left, right, operator) = eq.split().unwrap();
        assert_eq!(left.to_string(), "-2");
        assert_eq!(right.to_string(), "2^5");
        assert_eq!(operator, Operators::Mult);

        let eq = "-2^2^5";
        let eq = EquationString::remove_whitespaces(eq);
        let (left, right, operator) = eq.split().unwrap();
        assert_eq!(left.to_string(), "-2^2");
        assert_eq!(right.to_string(), "5");
        assert_eq!(operator, Operators::Exp);
    }

    #[test]
    pub fn test_split_eq_wrapped_with_parentheses() {
        let eq = "(-2^2^5)";
        let eq = EquationString::remove_whitespaces(eq);
        let (left, right, operator) = eq.split().unwrap();
        assert_eq!(left.to_string(), "-2^2");
        assert_eq!(right.to_string(), "5");
        assert_eq!(operator, Operators::Exp);

        let eq = "-(-2^2^5)";
        let eq = EquationString::remove_whitespaces(eq);
        let (left, right, operator) = eq.split().unwrap();
        assert_eq!(left.to_string(), "-1");
        assert_eq!(right.to_string(), "(-2^2^5)");
        assert_eq!(operator, Operators::Mult);
    }

    #[test]
    pub fn test_split_eq_contains_parentheses() {
        let eq = "-1+(-2^2^5)*25+3^3";
        let eq = EquationString::remove_whitespaces(eq);
        let (left, right, operator) = eq.split().unwrap();
        assert_eq!(left.to_string(), "-1+(-2^2^5)*25");
        assert_eq!(right.to_string(), "3^3");
        assert_eq!(operator, Operators::Plus);

        let eq = "-1+(-2^2^5)*25";
        let eq = EquationString::remove_whitespaces(eq);
        let (left, right, operator) = eq.split().unwrap();
        assert_eq!(left.to_string(), "-1");
        assert_eq!(right.to_string(), "(-2^2^5)*25");
        assert_eq!(operator, Operators::Plus);
    }

    #[test]
    pub fn test_more() {
        let eq = "0.1+(2+3)*5/3*2+((5+2)+2)";
        let eq = EquationString::remove_whitespaces(eq);
        let (left, right, operator) = eq.split().unwrap();
        assert_eq!(left.to_string(), "0.1+(2+3)*5/3*2");
        assert_eq!(right.to_string(), "((5+2)+2)");
        assert_eq!(operator, Operators::Plus);

        let eq = "0.1+(2+3)*5/3*2";
        let eq = EquationString::remove_whitespaces(eq);
        let (left, right, operator) = eq.split().unwrap();
        assert_eq!(left.to_string(), "0.1");
        assert_eq!(right.to_string(), "(2+3)*5/3*2");
        assert_eq!(operator, Operators::Plus);

        let eq = "(2+3)*5/3*2";
        let eq = EquationString::remove_whitespaces(eq);
        let (left, right, operator) = eq.split().unwrap();
        assert_eq!(left.to_string(), "(2+3)*5/3");
        assert_eq!(right.to_string(), "2");
        assert_eq!(operator, Operators::Mult);

        let eq = "(2+3)*5/3";
        let eq = EquationString::remove_whitespaces(eq);
        let (left, right, operator) = eq.split().unwrap();
        assert_eq!(left.to_string(), "(2+3)*5");
        assert_eq!(right.to_string(), "3");
        assert_eq!(operator, Operators::Div);
    }
}
