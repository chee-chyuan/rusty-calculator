use std::io::Split;

use crate::{
    precedences::{get_all_precedence_matchers, traits::MatchOperator},
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
    fn split_by_precedence(&self) -> (String, String, Operators);
}

impl SplitOperator for EquationString {
    /// split the equation into two halfs according to precedence
    fn split_by_precedence(&self) -> (Self, Self, Operators) {
        let all_matchers = get_all_precedence_matchers();

        // for matcher in all_matchers {

        //     let xx = matcher.into();

        //     // let found_operator_res = OperatorFinder::find_last(self.to_vec(), matcher, None);
        // }

        // strategy loop through all matchers, starting from lowest precedence
        // find from the end
        // for all cases, check if this operator belongs to a negative number (eg. +-1) and - will be detected, we should move on to +
        // apply to all cases even though other matchers wont have this issue
        // the moment any matcher found a math, we break from the for loop and return both halves

        // else outside of the for loop, we return (left, "", None) as this is just a value
        (self.to_vec(), Vec::new(), Operators::None)
    }
}

#[cfg(test)]
mod tests {

    use crate::precedences::{
        all_matcher::AllMatcher, high_precedence::HighPrecedenceMatcher,
        low_precedence::LowPrecedenceMatcher, medium_precedence::MediumPrecendenceMatcher,
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
}
