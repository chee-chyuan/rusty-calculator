use crate::{
    precedences::{
        all_matcher::AllMatcher, high_precedence::HighPrecedenceMatcher,
        low_precedence::LowPrecedenceMatcher, medium_precedence::MediumPrecendenceMatcher,
        traits::MatchOperator,
    },
    EquationString,
};

#[derive(Debug, Clone, Copy)]
pub struct OperatorFinder {}

pub trait FindOperator {
    fn find_first<M: MatchOperator>(
        eq: EquationString,
        match_operator: M,
        start_from: Option<usize>,
    ) -> Option<(usize, char)>;

    fn find_last<M: MatchOperator>(
        eq: EquationString,
        match_operator: M,
        start_from: Option<usize>,
    ) -> Option<(usize, char)>;
}

impl FindOperator for OperatorFinder {
    /// find the first occuring operator
    /// according to the 'match_operator'
    fn find_first<M: MatchOperator>(
        eq: EquationString,
        match_operator: M,
        start_from: Option<usize>,
    ) -> Option<(usize, char)> {
        let start_index = if let Some(i) = start_from {
            i as usize
        } else {
            0 as usize
        };

        for (i, c) in eq[start_index..].iter().enumerate() {
            if match_operator.match_operator(*c) {
                return Some((i + start_index, *c));
            }
        }

        None
    }

    /// find the last occuring operator
    /// according to the 'match_operator'
    fn find_last<M: MatchOperator>(
        eq: EquationString,
        match_operator: M,
        start_from: Option<usize>,
    ) -> Option<(usize, char)> {
        let start_index = if let Some(i) = start_from {
            i as usize
        } else {
            0 as usize
        };
        let last_element_index = eq.len() - 1;
        for (i, c) in eq[start_index..].iter().rev().enumerate() {
            if match_operator.match_operator(*c) {
                return Some((last_element_index - i, *c));
            }
        }
        None
    }
}

#[derive(Debug, PartialEq)]
pub enum Operators {
    Plus,
    Minus,
    Mult,
    Div,
    Exp,
    None,
}

impl Operators {
    /// convert operator to enum
    pub fn to_enum(c: char) -> Self {
        match c {
            '+' => Self::Plus,
            '-' => Self::Minus,
            '*' => Self::Mult,
            '/' => Self::Div,
            '^' => Self::Exp,
            _ => Self::None,
        }
    }

    /// calculate the result of the operation
    pub fn calculate(&self, left: f64, right: f64) -> f64 {
        match self {
            Self::Plus => left + right,
            Self::Minus => left - right,
            Self::Mult => left * right,
            Self::Div => left / right,
            Self::Exp => left.powf(right),
            Self::None => left,
        }
    }
}

pub trait SplitOperator {
    fn split_by_precedence(&self) -> (Self, Self, Operators)
    where
        Self: Sized;
}

impl SplitOperator for EquationString {
    /// split the equation into two halfs according to precedence
    /// this assumes that the equation doesnt have parentheses
    fn split_by_precedence(&self) -> (Self, Self, Operators) {
        // loop from front for + and -
        {
            let mut left: Vec<char> = Vec::new();
            let mut right = self.to_vec();

            let all_matcher = AllMatcher {};
            let matcher = LowPrecedenceMatcher {};

            loop {
                if right.len() == 0 {
                    break;
                }

                let found_operator_res =
                    OperatorFinder::find_first(right.to_vec(), matcher, Some(1));

                if let Some((index, operator)) = found_operator_res {
                    let left_char = right[index - 1];
                    let is_left_an_operator = all_matcher.match_operator(left_char);
                    if !is_left_an_operator {
                        left.append(&mut right[..index].to_vec());

                        let operator = Operators::to_enum(operator);
                        return (left, right[index + 1..].to_vec(), operator);
                    } else {
                        left.append(&mut right[..index].to_vec());
                        right = right[index..].to_vec();
                    }
                } else {
                    break;
                }
            }
        }

        // match for * and /
        {
            let eq = self.to_vec();

            let matcher = MediumPrecendenceMatcher {};
            let found_operator_res = OperatorFinder::find_last(eq.to_vec(), matcher, None);

            if let Some((index, operator)) = found_operator_res {
                if index == 0 {
                    return (eq.clone(), Vec::new() , Operators::None);
                }

                let operator = Operators::to_enum(operator);
                return (eq[..index].to_vec(), eq[index + 1..].to_vec(), operator);
            }
        }

        // match for ^
        {
            let eq = self.to_vec();

            let matcher = HighPrecedenceMatcher {};
            let found_operator_res = OperatorFinder::find_last(eq.to_vec(), matcher, None);

            if let Some((index, operator)) = found_operator_res {
                if index == 0 {
                    return (eq.clone(), Vec::new() , Operators::None);
                }
                let operator = Operators::to_enum(operator);
                return (eq[..index].to_vec(), eq[index + 1..].to_vec(), operator);
            }
        }

        // else outside of the for loop, we return (left, "", None) as this is just a value
        (self.to_vec(), Vec::new(), Operators::None)
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        eq_sanitize::EqSanitize,
        precedences::{
            all_matcher::AllMatcher, high_precedence::HighPrecedenceMatcher,
            low_precedence::LowPrecedenceMatcher, medium_precedence::MediumPrecendenceMatcher,
        },
    };

    use super::*;

    #[test]
    pub fn test_find_next_operator() {
        let all_operator = AllMatcher {};
        let low_precedence = LowPrecedenceMatcher {};
        let medium_precedence = MediumPrecendenceMatcher {};
        let high_precedence = HighPrecedenceMatcher {};

        let eq = "123456789+1*1-1/1";
        let eq = eq.chars().collect::<EquationString>();
        let (index, operator) = OperatorFinder::find_first(eq.clone(), all_operator, None).unwrap();
        assert_eq!((index, operator), (9, '+'));

        let (index, operator) =
            OperatorFinder::find_first(eq.clone(), low_precedence, None).unwrap();
        assert_eq!((index, operator), (9, '+'));

        let (index, operator) = OperatorFinder::find_first(eq, medium_precedence, None).unwrap();
        assert_eq!((index, operator), (11, '*'));

        let eq = "123456789+1*1-1/1^234";
        let eq = eq.chars().collect::<EquationString>();
        let (index, operator) = OperatorFinder::find_first(eq, high_precedence, None).unwrap();
        assert_eq!((index, operator), (17, '^'));

        let eq = "13434323";
        let eq = eq.chars().collect::<EquationString>();
        assert!(OperatorFinder::find_first(eq, medium_precedence, None).is_none());
    }

    #[test]
    fn test_find_next_operator_with_start_index() {
        let all_operator = AllMatcher {};
        let low_precedence = LowPrecedenceMatcher {};
        let medium_precedence = MediumPrecendenceMatcher {};
        let high_precedence = HighPrecedenceMatcher {};

        let eq = "123456789+1*1-1/1";
        let eq = eq.chars().collect::<EquationString>();
        let (index, operator) = OperatorFinder::find_first(eq, all_operator, Some(1)).unwrap();
        assert_eq!((index, operator), (9, '+'));

        let eq = "-123456789+1*1-1/1";
        let eq = eq.chars().collect::<EquationString>();
        let (index, operator) = OperatorFinder::find_first(eq, all_operator, Some(1)).unwrap();
        assert_eq!((index, operator), (10, '+'));

        let eq = "-123456789";
        let eq = eq.chars().collect::<EquationString>();
        assert!(OperatorFinder::find_first(eq, all_operator, Some(1)).is_none());
    }

    #[test]
    pub fn test_find_last_operator() {
        let all_operator = AllMatcher {};
        let low_precedence = LowPrecedenceMatcher {};
        let medium_precedence = MediumPrecendenceMatcher {};
        let high_precedence = HighPrecedenceMatcher {};

        let eq = "5/2*3";
        let eq = eq.chars().collect::<EquationString>();
        let (index, operator) = OperatorFinder::find_last(eq, medium_precedence, None).unwrap();
        assert_eq!((index, operator), (3, '*'));
    }

    #[test]
    pub fn test_split_by_precedence() {
        let eq = "1234";
        let eq = EquationString::remove_whitespaces(eq);
        let (left, right, operator) = eq.split_by_precedence();
        assert_eq!(left.to_string(), "1234");
        assert_eq!(right.to_string(), "");
        assert_eq!(operator, Operators::None);

        let eq = "-1234";
        let eq = EquationString::remove_whitespaces(eq);
        let (left, right, operator) = eq.split_by_precedence();
        assert_eq!(left.to_string(), "-1234");
        assert_eq!(right.to_string(), "");
        assert_eq!(operator, Operators::None);

        let eq = "-1234+134-+2";
        let eq = EquationString::remove_whitespaces(eq);
        let (left, right, operator) = eq.split_by_precedence();
        assert_eq!(left.to_string(), "-1234");
        assert_eq!(right.to_string(), "134-+2");
        assert_eq!(operator, Operators::Plus);

        let eq = "134-+2";
        let eq = EquationString::remove_whitespaces(eq);
        let (left, right, operator) = eq.split_by_precedence();
        assert_eq!(left.to_string(), "134");
        assert_eq!(right.to_string(), "+2");
        assert_eq!(operator, Operators::Minus);

        let eq = "-134*+2";
        let eq = EquationString::remove_whitespaces(eq);
        let (left, right, operator) = eq.split_by_precedence();
        assert_eq!(left.to_string(), "-134");
        assert_eq!(right.to_string(), "+2");
        assert_eq!(operator, Operators::Mult);

        let eq = "-3^2";
        let eq = EquationString::remove_whitespaces(eq);
        let (left, right, operator) = eq.split_by_precedence();
        assert_eq!(left.to_string(), "-3");
        assert_eq!(right.to_string(), "2");
        assert_eq!(operator, Operators::Exp);
    }
}
