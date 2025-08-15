fn main() {
    let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];
    for item in &haystack {
        let result = match item {
            42 | 132 => "Hit! 🤜🤛",
            _ => "miss 🫢",
        };

            println!("It's a {}", result);
    }
}

// study banger https://open.spotify.com/artist/2uFUBdaVGtyMqckSeCl0Qj?si=c7IyeBvvTG-08YiiD_4Q9Q
