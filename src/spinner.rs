use rand::seq::IndexedRandom;

// takes in a array of strings with movie titles
pub fn spin_wheel(movies: &Vec<(String, String)>) {
    let mut rng = rand::rng();
    if movies.is_empty() {
            println!("No movies found in the watchlist.");
            return;
        }
    // just chooses movie title on random
    let chosen = movies.choose(&mut rng).unwrap();

    //trying to figure out how to give the user a sense of spinning wheel
    // or maybe i should add like a matrix decoder effect instead?

    println!("\nYou should watch: {}", chosen.0);
}
