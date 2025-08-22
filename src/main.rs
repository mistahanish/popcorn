mod scraper;
mod spinner;
mod pagination;

use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    print!("Enter your Letterboxd username: ");
    io::stdout().flush()?;  // Flush to show prompt immediately

    let mut username = String::new();
    io::stdin().read_line(&mut username)?;
    let username = username.trim();

    let movies = scraper::fetch_watchlist(&username).await?;

    spinner::spin_wheel(&movies);

    Ok(())
}
