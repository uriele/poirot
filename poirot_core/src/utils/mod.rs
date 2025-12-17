pub fn normalize_to_lowercase(s: &str) -> String {
    s.split_whitespace()
     .map(|word| word.to_lowercase())
     .filter(|word| !word.is_empty())
     .collect::<Vec<String>>()
     .join(" ")
}

pub fn normalize_and_filter(s: &str) -> Option<String>{
    if s.is_empty(){
        None
    } else {
        Some(normalize_to_lowercase(s))
    }
}


pub fn capitalize_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
