pub fn is_armstrong_number(num: u32) -> bool {
    let mut mut_num = num;
    let mut digits = Vec::new();
    
    while mut_num > 0 {
        digits.push(mut_num % 10);
        mut_num /= 10;
    }

    digits.iter().fold(0, |acc, x| acc + x.pow(digits.len() as u32) as u64) == num as u64
    
}
