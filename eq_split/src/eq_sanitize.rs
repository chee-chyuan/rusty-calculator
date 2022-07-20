use crate::{
    math_characters::is_math_character,
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

    /// add a * in between a number and a special character
    /// eg 5π -> 5*π
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
        let mut previous_char: Option<char> = None;
        let all_matcher = AllMatcher {};

        loop {
            let eq_len = eq.len();
            if eq_len == 0 {
                break;
            }

            let current_char = eq[0];
            if current_char == '('
                && previous_char.is_some()
                && !all_matcher.match_operator(previous_char.unwrap())
                && previous_char.unwrap() != '('
            {
                previous_char = Some(current_char);
                new_eq.push('*');
                new_eq.push(current_char);
                eq = eq[1..].to_vec();
            } else if current_char == ')' && eq.len() > 1 {
                let next_char = eq[1];
                if !all_matcher.match_operator(next_char) {
                    previous_char = Some('*');
                    new_eq.push(current_char);
                    new_eq.push('*');
                    eq = eq[1..].to_vec();
                } else {
                    previous_char = Some(current_char);
                    new_eq.push(current_char);
                    eq = eq[1..].to_vec();
                }
            } else {
                previous_char = Some(current_char);
                new_eq.push(current_char);
                eq = eq[1..].to_vec();
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
        let eq = "2(e+2)^π*2+-((5+7/2)-3^pi)";
        let eq = EquationString::remove_whitespaces(eq);
        let new_eq = eq.handle_direct_multiplication().unwrap();
        assert_eq!(new_eq.to_string(), "2*(e+2)^π*2+-((5+7/2)-3^pi)");

        let eq = "(pi*2(6+7)4)";
        let eq = EquationString::remove_whitespaces(eq);
        let new_eq = eq.handle_direct_multiplication().unwrap();
        assert_eq!(new_eq.to_string(), "(pi*2*(6+7)*4)");

        let eq = "(5+5)+5(7+3)(5*8)";
        let eq = EquationString::remove_whitespaces(eq);
        let new_eq = eq.handle_direct_multiplication().unwrap();
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
