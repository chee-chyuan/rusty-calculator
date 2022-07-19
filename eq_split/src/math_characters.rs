pub fn is_math_character(val: char) -> bool {
    match val {
        'π' => true,
        'p' => true,
        'e' => true,
        _ => false,
    }
}
