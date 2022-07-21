use super::traits::MatchOperator;

#[derive(Debug, Clone, Copy)]
pub struct HighPrecedenceMatcher;

impl MatchOperator for HighPrecedenceMatcher {
    fn match_operator(&self, c: char) -> bool {
        match c {
            '^' => true,
            _ => false,
        }
    }
}
