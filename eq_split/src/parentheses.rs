use crate::EquationString;

#[derive(Debug, Clone, Copy)]
pub struct ParenthesesFinder {}

pub trait FindParentheses {
    fn find_first(eq: EquationString) -> Result<Option<(usize, usize)>, String>;
    fn find_last(eq: EquationString) -> Result<Option<(usize, usize)>, String>;
}

impl FindParentheses for ParenthesesFinder {
    fn find_first(eq: crate::EquationString) -> Result<Option<(usize, usize)>, String> {
        let mut opened_parentheses = 0;
        let mut left_parentheses_index: Option<usize> = None;
        let mut right_parentheses_index: Option<usize> = None;

        for (i, c) in eq.iter().enumerate() {
            if *c == '(' {
                opened_parentheses = opened_parentheses + 1;
                if left_parentheses_index == None {
                    left_parentheses_index = Some(i)
                }
            } else if *c == ')' {
                if opened_parentheses == 0 {
                    return Err(String::from("Close bracket found without open brackets"));
                }
                opened_parentheses = opened_parentheses - 1;

                if right_parentheses_index.is_none() && opened_parentheses == 0 {
                    right_parentheses_index = Some(i);
                }
            }
        }

        if opened_parentheses > 0 {
            return Err(String::from("Open bracket not closed"));
        }

        if left_parentheses_index == None && right_parentheses_index == None {
            return Ok(None);
        }

        Ok(Some((
            left_parentheses_index.unwrap(),
            right_parentheses_index.unwrap(),
        )))
    }

    fn find_last(eq: crate::EquationString) -> Result<Option<(usize, usize)>, String> {
        let mut closed_parentheses = 0;
        let mut left_parentheses_index: Option<usize> = None;
        let mut right_parentheses_index: Option<usize> = None;

        let last_index = eq.len() - 1;

        for (i, c) in eq.iter().rev().enumerate() {
            if *c == ')' {
                closed_parentheses = closed_parentheses + 1;
                if right_parentheses_index == None {
                    right_parentheses_index = Some(last_index - i)
                }
            } else if *c == '(' {
                if closed_parentheses == 0 {
                    return Err(String::from("Close bracket found without open brackets"));
                }
                closed_parentheses = closed_parentheses - 1;

                if left_parentheses_index.is_none() && closed_parentheses == 0 {
                    left_parentheses_index = Some(last_index - i);
                }
            }
        }

        if closed_parentheses > 0 {
            return Err(String::from("Open bracket not closed"));
        }

        if left_parentheses_index == None && right_parentheses_index == None {
            return Ok(None);
        }

        Ok(Some((
            left_parentheses_index.unwrap(),
            right_parentheses_index.unwrap(),
        )))
    }
}

#[cfg(test)]
mod tests {
    use crate::EquationString;

    use super::*;

    #[test]
    pub fn test_find_first_parentheses() {
        let eq = "(1+3+5)";
        let eq = eq.chars().collect::<EquationString>();
        let (left, right) = ParenthesesFinder::find_first(eq).unwrap().unwrap();
        assert_eq!(left, 0);
        assert_eq!(right, 6);

        let eq = "1+(3+5)+2";
        let eq = eq.chars().collect::<EquationString>();
        let (left, right) = ParenthesesFinder::find_first(eq).unwrap().unwrap();
        assert_eq!(left, 2);
        assert_eq!(right, 6);

        let eq = "1+3+5";
        let eq = eq.chars().collect::<EquationString>();
        let res = ParenthesesFinder::find_first(eq).unwrap();
        assert_eq!(res, None);

        let eq = "1+343)";
        let eq = eq.chars().collect::<EquationString>();
        ParenthesesFinder::find_first(eq).expect_err("Close bracket found without open brackets");

        let eq = "1+(343";
        let eq = eq.chars().collect::<EquationString>();
        ParenthesesFinder::find_first(eq).expect_err("Open bracket not closed");
    }

    #[test]
    pub fn test_find_last_parentheses() {
        let eq = "(1+3+5)";
        let eq = eq.chars().collect::<EquationString>();
        let (left, right) = ParenthesesFinder::find_last(eq).unwrap().unwrap();
        assert_eq!(left, 0);
        assert_eq!(right, 6);

        let eq = "(1+3+5)+((1+3+5))+3/13*3";
        let eq = eq.chars().collect::<EquationString>();
        let (left, right) = ParenthesesFinder::find_last(eq).unwrap().unwrap();
        assert_eq!(left, 8);
        assert_eq!(right, 16);
    }
}
