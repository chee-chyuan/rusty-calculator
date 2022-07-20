use crate::{
    precedences::{
        all_matcher::AllMatcher, high_precedence::HighPrecedenceMatcher,
        low_precedence::LowPrecedenceMatcher, medium_precedence::MediumPrecendenceMatcher,
        traits::MatchOperator,
    },
    EquationString,
};

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

pub trait SplitParentheses {
    fn split_by_parentheses(
        &self,
        left_parentheses_index: usize,
        right_parentheses_index: usize,
    ) -> (Self, Self)
    where
        Self: Sized;
}

impl SplitParentheses for EquationString {
    fn split_by_parentheses(
        &self,
        left_parentheses_index: usize,
        right_parentheses_index: usize,
    ) -> (Self, Self) {

        #[allow(unused_assignments)]
        let mut left: Self = Self::new();
        let mut right: Self = Self::new();

        // splitting
        // splitting is left bias, meaning we will put all our parenthesis on the left

        if left_parentheses_index == 0 {
            // parentheses is at the beginning of the eq. eg. (1+3)*3+5
            left = self[..right_parentheses_index + 1].to_vec();
            right = self[right_parentheses_index + 1..].to_vec();
        } else if right_parentheses_index == (self.len() - 1) {
            // parentheses is at the end of the eq. eg. 3+5*3+(3+5)
            left = self.to_vec();
        } else {
            // parentheses is at the middle of the eq. eg. 3+5*3*(5/3)+53
            left = self[..right_parentheses_index + 1].to_vec();
            right = self[right_parentheses_index + 1..].to_vec();
        }

        // regroup according to precedence
        regroup_according_to_precedence(left, right)
    }
}

/// split the eq that contains parentheses into
/// a left half and a right half
fn regroup_according_to_precedence(
    left: EquationString,
    right: EquationString,
) -> (EquationString, EquationString) {
    // if right is not empty, we will gather all the higher precedence and
    // put it to the left
    let mut left = left;
    let mut right = right;

    if !right.is_empty() {
        (left, right) = regroup_to_left(left.clone(), right.clone());
    }

    // then check if right is empty again, as the operation before has regroup
    // some of right to left
    // if so, we split the left and fill in the right

    if right.is_empty() {
        (left, right) = regroup_to_right(left.clone());
    }

    (left, right)
}

/// group the operator of the same precedence to the left
/// for this case, it is assumed that both left and right are not empty
/// checking has been done prior to calling this function
fn regroup_to_left(
    left: EquationString,
    right: EquationString,
) -> (EquationString, EquationString) {
    let all_matcher = AllMatcher {};
    let low_precedence_matcher = LowPrecedenceMatcher {};

    // first, check for all + and - and split it
    // check from front and move the front to the left
    // to treat items in the parentheses as a single entity
    // if split occur return

    {
        let mut left_after = left.clone();
        let mut right_after = right.clone();
        let mut previous_char: Option<char> = None;
        loop {
            if right_after.len() == 0 {
                break;
            }

            let current_char = right_after[0];
            let is_low_precedence_operator = low_precedence_matcher.match_operator(current_char);

            // check if this is a negative or positive number
            // we check if the previous char is None, or an operator, if so this is just a sign for -ve or +ve
            if is_low_precedence_operator {
                if previous_char.is_some() && all_matcher.match_operator(previous_char.unwrap()) {
                    // just a -ve or +ve sign
                    // we update the previous_char, and move this char to the left
                    previous_char = Some(current_char);
                    left_after.push(current_char);
                    right_after = right_after[1..].to_vec();
                } else {
                    // found it, we split here
                    return (left_after, right_after);
                }
            } else {
                // check if it is a parentheses
                // if so we find the end of the parenthese and move the entire block to the left
                // else, we update the previous_char, and move this char to the left
                if current_char == '(' {
                    let (left_parentheses_index, right_parentheses_index) =
                        ParenthesesFinder::find_first(right_after.clone())
                            .unwrap()
                            .unwrap();

                    previous_char = Some(right_after[right_parentheses_index]);
                    left_after.append(
                        &mut right_after[left_parentheses_index..right_parentheses_index + 1]
                            .to_vec(),
                    );
                    right_after = right_after[right_parentheses_index + 1..].to_vec();
                } else {
                    previous_char = Some(current_char);
                    left_after.push(current_char);
                    right_after = right_after[1..].to_vec();
                }
            }
        }
    }

    // whatever that is left has to be *,/ or ^
    // we just group everything to the left

    let mut left_after = left.clone();
    let mut right_after = right.clone();
    left_after.append(&mut right_after);
    (left_after, EquationString::new())
}

/// splitting left to right according to precedence
/// right is assumed to be empty
fn regroup_to_right(left: EquationString) -> (EquationString, EquationString) {
    let all_matcher = AllMatcher {};
    let low_precedence_matcher = LowPrecedenceMatcher {};
    let medium_precedence_matcher = MediumPrecendenceMatcher {};
    let high_precedence_matcher = HighPrecedenceMatcher {};

    // we will first loop from left to right to check for + and -
    // checking for negatives and treating parentheses as a unit
    // if we are able to split, we return

    {
        let mut left_after: EquationString = Vec::new();
        let mut right_after = left.clone();
        let mut previous_char: Option<char> = None;

        // we can ignore the first char as it could be -ve or +ve or number
        // it doesnt matter to us
        let first_char = right_after[0];

        if first_char != '(' {
            left_after.push(first_char);
            right_after = right_after[1..].to_vec();
        }

        loop {
            if right_after.len() == 0 {
                break;
            }

            let current_char = right_after[0];
            let is_low_precedence_operator = low_precedence_matcher.match_operator(current_char);

            // check if this is a negative or positive number
            // we check if the previous char is None, or an operator, if so this is just a sign for -ve or +ve
            if is_low_precedence_operator {
                if previous_char.is_some() && all_matcher.match_operator(previous_char.unwrap()) {
                    // just a -ve or +ve sign
                    // we update the previous_char, and move this char to the left
                    previous_char = Some(current_char);
                    left_after.push(current_char);
                    right_after = right_after[1..].to_vec();
                } else {
                    // found it, we split here
                    return (left_after, right_after);
                }
            } else {
                // check if it is a parentheses
                // if so we find the end of the parenthese and move the entire block to the left
                // else, we update the previous_char, and move this char to the left
                if current_char == '(' {
                    let (left_parentheses_index, right_parentheses_index) =
                        ParenthesesFinder::find_first(right_after.clone())
                            .unwrap()
                            .unwrap();

                    previous_char = Some(right_after[right_parentheses_index]);
                    left_after.append(
                        &mut right_after[left_parentheses_index..right_parentheses_index + 1]
                            .to_vec(),
                    );
                    right_after = right_after[right_parentheses_index + 1..].to_vec();
                } else {
                    previous_char = Some(current_char);
                    left_after.push(current_char);
                    right_after = right_after[1..].to_vec();
                }
            }
        }
    }

    // loop from right to left to check for * and /
    // treating parentheses as a unit
    // if we are able to split, we return

    {
        let mut left_after = left.clone();
        let mut right_after: EquationString = Vec::new();

        loop {
            let left_len = left_after.len();
            if left_len == 0 {
                break;
            }

            let current_index = left_len - 1;
            let current_char = left_after[current_index];
            let is_medium_precendence_operator =
                medium_precedence_matcher.match_operator(current_char);
            if is_medium_precendence_operator {
                // move char to right (front)
                // remove char from left_after
                // return

                right_after.insert(0, current_char);
                left_after.pop();

                return (left_after, right_after);
            } else {
                // detect for closing bracket
                // if so we detect for the opening bracket and move everythig to the right
                // update temp_left and right_after (front) accordingly
                if current_char == ')' {
                    let (left_parentheses_index, right_parentheses_index) =
                        ParenthesesFinder::find_last(left_after.clone())
                            .unwrap()
                            .unwrap();

                    let mut parentheses_vec =
                        left_after[left_parentheses_index..right_parentheses_index + 1].to_vec();
                    parentheses_vec.extend_from_slice(&right_after);
                    right_after = parentheses_vec;
                    left_after = left_after[..left_parentheses_index].to_vec();
                } else {
                    right_after.insert(0, current_char);
                    left_after.pop();
                }
            }
        }
    }

    // loop from right to left to check for ^
    // treating parentheses as a unit
    //if we are able to split, we return

    {
        let mut left_after = left.clone();
        let mut right_after: EquationString = Vec::new();

        loop {
            let left_len = left_after.len();
            if left_len == 0 {
                break;
            }

            let current_index = left_len - 1;
            let current_char = left_after[current_index];
            let is_high_precendence_operator = high_precedence_matcher.match_operator(current_char);
            if is_high_precendence_operator {
                // move char to right (front)
                // remove char from left_after
                // return

                right_after.insert(0, current_char);
                left_after.pop();

                return (left_after, right_after);
            } else {
                // detect for closing bracket
                // if so we detect for the opening bracket and move everythig to the right
                // update temp_left and right_after (front) accordingly
                if current_char == ')' {
                    let (left_parentheses_index, right_parentheses_index) =
                        ParenthesesFinder::find_last(left_after.clone())
                            .unwrap()
                            .unwrap();

                    let mut parentheses_vec =
                        left_after[left_parentheses_index..right_parentheses_index + 1].to_vec();
                    parentheses_vec.extend_from_slice(&right_after);
                    right_after = parentheses_vec;
                    left_after = left_after[..left_parentheses_index].to_vec();
                } else {
                    right_after.insert(0, current_char);
                    left_after.pop();
                }
            }
        }
    }

    (left.clone(), left.clone())
}

#[cfg(test)]
mod tests {
    use crate::{eq_sanitize::EqSanitize, EquationString};

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

    #[test]
    pub fn test_regroup_to_left_low_precedence() {
        let left = "(1+3)";
        let right = "-1*55";
        let left = EquationString::remove_whitespaces(left);
        let right = EquationString::remove_whitespaces(right);
        let (left_after, right_after) = regroup_to_left(left, right);
        assert_eq!(left_after.to_string(), "(1+3)");
        assert_eq!(right_after.to_string(), "-1*55");

        let left = "(1+3)";
        let right = "*1*-2^2+13+5";
        let left = EquationString::remove_whitespaces(left);
        let right = EquationString::remove_whitespaces(right);
        let (left_after, right_after) = regroup_to_left(left, right);
        assert_eq!(left_after.to_string(), "(1+3)*1*-2^2");
        assert_eq!(right_after.to_string(), "+13+5");

        let left = "(1+3)";
        let right = "*1*-2^2/(13+5)+1";
        let left = EquationString::remove_whitespaces(left);
        let right = EquationString::remove_whitespaces(right);
        let (left_after, right_after) = regroup_to_left(left, right);
        assert_eq!(left_after.to_string(), "(1+3)*1*-2^2/(13+5)");
        assert_eq!(right_after.to_string(), "+1");
    }

    #[test]
    pub fn test_regroup_to_left_medium_precedence() {
        let left = "(1+3)";
        let right = "*1*-2^2/(13+5)^1";
        let left = EquationString::remove_whitespaces(left);
        let right = EquationString::remove_whitespaces(right);
        let (left_after, right_after) = regroup_to_left(left, right);
        assert_eq!(left_after.to_string(), "(1+3)*1*-2^2/(13+5)^1");
        assert_eq!(right_after.to_string(), "");

        let left = "(1+3)";
        let right = "*1*-2^2/-2^1";
        let left = EquationString::remove_whitespaces(left);
        let right = EquationString::remove_whitespaces(right);
        let (left_after, right_after) = regroup_to_left(left, right);
        assert_eq!(left_after.to_string(), "(1+3)*1*-2^2/-2^1");
        assert_eq!(right_after.to_string(), "");
    }

    #[test]
    pub fn test_regroup_to_left_high_precedence() {
        let left = "(1+3)";
        let right = "^1^(1/3)";
        let left = EquationString::remove_whitespaces(left);
        let right = EquationString::remove_whitespaces(right);
        let (left_after, right_after) = regroup_to_left(left, right);
        assert_eq!(left_after.to_string(), "(1+3)^1^(1/3)");
        assert_eq!(right_after.to_string(), "");
    }

    #[test]
    pub fn test_regroup_to_right_low_precedence() {
        let left = "-2*2/5^7+(1+3)";
        let left = EquationString::remove_whitespaces(left);
        let (left_after, right_after) = regroup_to_right(left);
        assert_eq!(left_after.to_string(), "-2*2/5^7");
        assert_eq!(right_after.to_string(), "+(1+3)");

        let left = "-2*2/5^7*(5+2)+(1+3)";
        let left = EquationString::remove_whitespaces(left);
        let (left_after, right_after) = regroup_to_right(left);
        assert_eq!(left_after.to_string(), "-2*2/5^7*(5+2)");
        assert_eq!(right_after.to_string(), "+(1+3)");
    }

    #[test]
    pub fn test_regroup_to_right_medium_precedence() {
        let left = "-2*2/5^7*(5+2)^(1+3)";
        let left = EquationString::remove_whitespaces(left);
        let (left_after, right_after) = regroup_to_right(left);
        assert_eq!(left_after.to_string(), "-2*2/5^7");
        assert_eq!(right_after.to_string(), "*(5+2)^(1+3)");

        let left = "-2*2/5^7*5/2^(1+3)";
        let left = EquationString::remove_whitespaces(left);
        let (left_after, right_after) = regroup_to_right(left);
        assert_eq!(left_after.to_string(), "-2*2/5^7*5");
        assert_eq!(right_after.to_string(), "/2^(1+3)");
    }

    #[test]
    pub fn test_regroup_to_right_high_precedence() {
        let left = "5^7^-(5+2/1+3)";
        let left = EquationString::remove_whitespaces(left);
        let (left_after, right_after) = regroup_to_right(left);
        assert_eq!(left_after.to_string(), "5^7");
        assert_eq!(right_after.to_string(), "^-(5+2/1+3)");
    }
}
