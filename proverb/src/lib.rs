pub fn build_proverb(list: &[&str]) -> String {
    match list.len() {
        0 => String::new(),
        1 => "And all for the want of a nail.".to_string(),
        _ => {
            let mut result = String::new();
            result.push_str(format!("For want of a {}", list[0]).as_str());
            for s in list[1..list.len() - 1].iter() {
                result.push_str(format!(" the {} was lost.\nFor want of a {}", s, s).as_str());
            }
            result.push_str(format!(" the {} was lost.\nAnd all for the want of a {}.", list[list.len() - 1], list[0]).as_str());

            result
        }
    }
}
