use super::traits::MatchOperator;


#[derive(Debug, Clone, Copy)]
pub struct AllMatcher;

impl MatchOperator for AllMatcher {
    fn match_operator(&self, c: char) -> bool {
        match c {
            '+' | '-' | '*' | '/' | '^' => true,
            _ => false,
        }
    }
}
