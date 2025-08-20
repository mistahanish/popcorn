use rand::seq::IndexedRandom;

// takes in a array of strings with movie titles
fn spin_wheel(movies: &[String]) {
    let mut rng = rand::rng();

    // just chooses movie title on random
    let chosen = movies.choose(&mut rng).unwrap();

    //trying to figure out how to give the user a sense of spinning wheel
    // or maybe i should add like a matrix decoder effect instead?

    println!("\nYou should watch: {}", chosen);
}

fn main() {
    let movies = vec![
        "Spirited Away".to_string(),
        "Your Name".to_string(),
        "Akira".to_string(),
        "Perfect Blue".to_string(),
    ];
    spin_wheel(&movies);
}
