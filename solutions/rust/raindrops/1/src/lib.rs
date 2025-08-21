pub fn raindrops(n: u32) -> String {
    let (sounds, _): (Vec<_>, Vec<_>) = [("Pling", 3), ("Plang", 5), ("Plong", 7)]
        .iter()
        .filter(|(_, factor)| n % *factor == 0)
        .cloned()
        .unzip();

    if sounds.is_empty() {
        return n.to_string();
    }

    sounds.concat()
}
