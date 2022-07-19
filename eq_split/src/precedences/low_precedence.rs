use super::traits::MatchOperator;

#[derive(Debug, Clone, Copy)]
pub struct LowPrecedenceMatcher {}

impl MatchOperator for LowPrecedenceMatcher {
    fn match_operator(&self, c: char) -> bool {
        match c {
            '+' | '-' => true,
            _ => false,
        }
    }
}
