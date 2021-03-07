pub fn encrypt(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }

    let normalized = input
        .to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect::<Vec<_>>();

    let sqrt_of_len = (normalized.len() as f64).sqrt();
    let cols = sqrt_of_len.ceil() as usize;
    let rows = sqrt_of_len.floor() as usize;

    (0..cols)
        .map(|i| {
            (0..rows)
                .map(|j| normalized.get(i + j * cols).unwrap_or(&' '))
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(" ")
}
