pub fn abbreviate(s: &str) -> String {
    s.split(' ')
        .flat_map(
            |w| if w.chars().filter(|&c| c.is_uppercase()).count() > 1 &&
                w.chars().any(char::is_lowercase)
            {
                w.chars().filter(|&c| c.is_uppercase()).collect::<Vec<_>>()
            } else if w.chars().any(|c| !c.is_alphabetic()) {
                w.split(|c: char| !c.is_alphabetic())
                    .flat_map(|x| x.chars().take(1))
                    .collect::<Vec<_>>()
            } else {
                w.chars().take(1).collect::<Vec<_>>()
            },
        )
        .collect::<String>()
        .to_uppercase()
}
