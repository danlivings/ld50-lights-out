pub fn format_number(n: u64) -> String {
    let n_string = n.to_string();

    let mut chars = vec![];

    let mut i = 0;
    for n_char in n_string.chars().rev() {
        if i != 0 && i % 3 == 0 {
            chars.push(' ');
        }
        chars.push(n_char);
        i = i + 1;
    }

    chars.into_iter()
        .rev()
        .collect()
}
