pub fn build_proverb(list: &[&str]) -> String {
    let mut sentences = (1..list.len())
        .map(|idx| {
            format!(
                "For want of a {} the {} was lost.",
                list[idx - 1],
                list[idx],
            )
        })
        .collect::<Vec<_>>();
    if list.len() != 0 {
        sentences.push(format!("And all for the want of a {}.", list[0]));
    }
    sentences.join("\n")
}
