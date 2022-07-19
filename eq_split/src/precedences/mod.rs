use self::{
    high_precedence::HighPrecedenceMatcher, low_precedence::LowPrecedenceMatcher,
    medium_precedence::MediumPrecendenceMatcher, traits::MatchOperator,
};

pub mod all_matcher;
pub mod high_precedence;
pub mod low_precedence;
pub mod medium_precedence;
pub mod traits;

/// get all matching function from lowest to highest precedence
pub fn get_all_precedence_matchers() -> Vec<Box<dyn MatchOperator>> {
    let mut matchers: Vec<Box<dyn MatchOperator>> = Vec::new();
    matchers.push(Box::new(LowPrecedenceMatcher {}));
    matchers.push(Box::new(MediumPrecendenceMatcher {}));
    matchers.push(Box::new(HighPrecedenceMatcher {}));

    matchers
}


pub fn get_all_precedence_matchers2() -> Vec<Box<dyn MatchOperator>> {
    let mut matchers: Vec<Box<dyn MatchOperator>> = Vec::new();
    matchers.push(Box::new(LowPrecedenceMatcher {}));
    matchers.push(Box::new(MediumPrecendenceMatcher {}));
    matchers.push(Box::new(HighPrecedenceMatcher {}));

    matchers
}