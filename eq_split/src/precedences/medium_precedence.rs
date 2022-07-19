use super::traits::MatchOperator;

#[derive(Debug, Clone, Copy)]
pub struct MediumPrecendenceMatcher {}

impl MatchOperator for MediumPrecendenceMatcher {
    fn match_operator(&self, c: char) -> bool {
        match c {
            '*' | '/' => true,
            _ => false,
        }
    }
}
