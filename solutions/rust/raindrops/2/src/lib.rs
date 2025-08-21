pub fn raindrops(n: u32) -> String {
    let sounds: String = [(3, "Pling"), (5, "Plang"), (7, "Plong")]
        .iter()
        .filter_map(|(factor, sound)| match n % *factor {
            0 => Some(*sound),
            _ => None,
        })
        .collect();

    if !sounds.is_empty() {
        sounds
    } else {
        n.to_string()
    }
}
