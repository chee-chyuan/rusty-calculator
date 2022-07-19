use crate::{
    math_characters::is_math_character,
    parentheses::{FindParentheses, ParenthesesFinder},
    precedences::{all_matcher::AllMatcher, traits::MatchOperator},
    EquationString,
};

pub trait EqSanitize {
    fn remove_whitespaces(eq: &str) -> Self;
    fn handle_special_character_multiplication(&self) -> Result<Self, String>
    where
        Self: Sized;
    fn handle_direct_multiplication(&self) -> Result<Self, String>
    where
        Self: Sized;

    fn to_string(self) -> String;
}

impl EqSanitize for EquationString {
    /// remove all whitespaces in the eq string
    fn remove_whitespaces(eq: &str) -> Self {
        let new_eq = eq.replace(" ", "");
        new_eq.chars().collect::<EquationString>()
    }

    fn handle_special_character_multiplication(&self) -> Result<Self, String> {
        let mut new_eq: EquationString = Vec::new();
        let mut eq = self.to_vec();

        let mut previous_char: Option<char> = None;

        loop {
            let eq_len = eq.len();
            if eq_len == 0 {
                break;
            }

            let current_char = eq[0];
            if is_math_character(current_char)
                && previous_char.is_some()
                && previous_char.unwrap().is_digit(10)
            {
                previous_char = Some(current_char);
                new_eq.push('*');
                new_eq.push(current_char);
                eq = eq[1..].to_vec();
            } else {
                previous_char = Some(current_char);
                new_eq.push(current_char);
                eq = eq[1..].to_vec();
            }
        }
        Ok(new_eq)
    }

    /// perform string manipulation to add '*' to operations involving brackets
    /// eg. 5(2) -> 5*(2)
    fn handle_direct_multiplication(&self) -> Result<Self, String>
    where
        Self: Sized,
    {
        let mut new_eq: EquationString = Vec::new();
        let mut eq = self.to_vec();
        let all_matcher = AllMatcher {};

        loop {
            let eq_len = eq.len();
            if eq_len == 0 {
                break;
            }

            let res = ParenthesesFinder::find_first(eq.clone());
            if let Err(t) = res {
                return Err(t);
            }

            if let Some((left_index, mut right_index)) = res.unwrap() {
                if left_index == 0 {
                    if right_index == eq_len - 1 {
                        // ')' is at the end of string
                        new_eq.append(&mut eq);
                        eq = EquationString::new();
                    } else {
                        let right_char = eq[right_index + 1];
                        let is_right_an_operator = all_matcher.match_operator(right_char); //check if the char to the right of ')' is an operator

                        new_eq.append(&mut eq[..right_index + 1].to_vec()); // appendinng to the new_eq up to the ending ')'

                        if !is_right_an_operator {
                            new_eq.push('*');
                        } else {
                            new_eq.push(right_char);
                            right_index = right_index + 1;
                        }

                        eq = eq[right_index + 1..].to_vec();
                    }
                    continue;
                }

                // check if char to the left of '(' is not an operator
                let left_char = eq[left_index - 1];
                let is_left_an_operator = all_matcher.match_operator(left_char);

                let mut is_right_operator = true;
                if right_index != eq.len() - 1 {
                    let right_char = eq[right_index + 1];
                    is_right_operator = all_matcher.match_operator(right_char);
                }

                new_eq.append(&mut eq[..left_index].to_vec());

                if !is_left_an_operator {
                    new_eq.push('*');
                }

                new_eq.append(&mut eq[left_index..right_index + 1].to_vec());

                if !is_right_operator {
                    new_eq.push('*');
                }

                eq = eq[right_index + 1..].to_vec();
            } else {
                new_eq.append(&mut eq);
                eq = Vec::new();
            }
        }
        Ok(new_eq)
    }

    /// convert EquationString to string
    fn to_string(self) -> String {
        self.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_handle_special_character_multiplication() {
        let eq = "e(e)e+2e^ep";
        let eq = EquationString::remove_whitespaces(eq);
        let new_eq = eq.handle_special_character_multiplication().unwrap();
        assert_eq!(new_eq.to_string(), "e(e)e+2*e^ep");
    }

    #[test]
    pub fn test_handle_direct_multiplication() {
        let eq = "(5+5)+5(7+3)(5*8)";
        let eq = EquationString::remove_whitespaces(eq);
        let new_eq = eq.handle_direct_multiplication().unwrap();
        // let new_eq = handle_direct_multiplication(eq).unwrap();
        assert_eq!(new_eq.to_string(), "(5+5)+5*(7+3)*(5*8)");

        let eq = "(5+5)5+5(7+3)(5*8)";
        let eq = EquationString::remove_whitespaces(eq);
        let new_eq = eq.handle_direct_multiplication().unwrap();
        assert_eq!(new_eq.to_string(), "(5+5)*5+5*(7+3)*(5*8)");

        let eq = "(5+5)*5+5(7+3)(5*8)";
        let eq = EquationString::remove_whitespaces(eq);
        let new_eq = eq.handle_direct_multiplication().unwrap();
        assert_eq!(new_eq.to_string(), "(5+5)*5+5*(7+3)*(5*8)");

        let eq = "(1+2)*5π(5+2)/4";
        let eq = EquationString::remove_whitespaces(eq);
        let new_eq = eq.handle_direct_multiplication().unwrap();
        assert_eq!(new_eq.to_string(), "(1+2)*5π*(5+2)/4");

        let eq = "(1+2)π*5π(5+2)/4";
        let eq = EquationString::remove_whitespaces(eq);
        let new_eq = eq.handle_direct_multiplication().unwrap();
        assert_eq!(new_eq.to_string(), "(1+2)*π*5π*(5+2)/4");
    }
}
