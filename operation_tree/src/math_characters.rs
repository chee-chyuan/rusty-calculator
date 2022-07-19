use eq_split::EquationString;

pub fn match_math_character(val: &str) -> Option<f64> {
    match val {
        "π" => Some(std::f64::consts::PI), // in macos, option+p
        "pi" => Some(std::f64::consts::PI),
        "e" => Some(std::f64::consts::E),
        _ => None,
    }
}

pub fn math_character_position(val: EquationString) -> Option<(usize, EquationString)> {
    let has_pi = val.clone().iter().position(|&r| r == 'π');
    if has_pi.is_some() {
        return Some((has_pi.unwrap(), vec!['π']));
    }

    let has_e = val.clone().iter().position(|&r| r == 'e');
    if has_e.is_some() {
        return Some((has_e.unwrap(), vec!['e']));
    }


    let has_pi_word_p = val.clone().iter().position(|&r| r == 'p');
    let has_pi_word_i = val.clone().iter().position(|&r| r == 'i');
    if has_pi_word_p.is_some() && has_pi_word_i.is_some() {
        let index_p = has_pi_word_p.unwrap();
        let index_i = has_pi_word_i.unwrap();

        if index_p + 1 == index_i {
            return Some((index_p, vec!['p','i']));
        }
    }

    None
}
