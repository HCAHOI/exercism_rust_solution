pub fn raindrops(n: u32) -> String {
    let mut words = Vec::new();
    if n % 3 == 0 {
        words.push(String::from("Pling"));
    }
    if n % 5 == 0 {
        words.push(String::from("Plang"));
    }
    if n % 7 == 0 {
        words.push(String::from("Plong"));
    }

    if words.len() > 0 {
        words.join("")
    } else {
        n.to_string()
    }
}
