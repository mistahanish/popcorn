use crate::pagination;
use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashSet;

pub async fn fetch_watchlist(
    username: &str,
) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let client = Client::new();
    let mut page = 1;
    let mut movies = Vec::new();
    let mut visited_pages = HashSet::new();

    loop {
        // Construct URL for the current page
        let url = if page == 1 {
            format!("https://letterboxd.com/{}/watchlist/", username)
        } else {
            format!(
                "https://letterboxd.com/{}/watchlist/page/{}/",
                username, page
            )
        };

        // Avoid infinite loops by tracking visited URLs
        if visited_pages.contains(&url) {
            break;
        }
        visited_pages.insert(url.clone());

        // Fetch page content
        let res = client.get(&url).send().await?.text().await?;
        //std::fs::write("output.html", &res).unwrap();
        //everything works up to here

        // Parse HTML document
        let document = Html::parse_document(&res);

        // iterate over li.griditem
        let selector = Selector::parse("li.griditem div.react-component").unwrap();


        let mut found_films_on_page = 0;
        for element in document.select(&selector) {
            if let Some(name) = element.value().attr("data-item-full-display-name") {
                //ok fixed this line
                //println!("Found movie: {}", name); // Debugging line
                if let Some(slug) = element.value().attr("data-target-link") {
                    let full_url = format!("https://letterboxd.com{}", slug);
                    //println!("Full URL: {}", full_url); // Debugging line
                    movies.push((name.to_string(), full_url));
                    found_films_on_page += 1;
                }
            }
        }

        // for (title, _) in &movies {
        // println!("{}", title);
        // }
        // Debugging line to print all collected movie titles
        // Stop if no films found on this page (end of pagination)
        if found_films_on_page == 0 {
            println!("breaking here");
            break;
        }

        // Check if there is a link to the next page by looking for pagination div, and then a a.next link
        // let paginationdiv = Selector::parse("div.pagination").unwrap();
        // let document2 = Html::parse_document(&paginationdiv);
        // let next_page_selector = Selector::parse("a.next").unwrap();
        //let has_next_page = document.select(&next_page_selector).next().is_some();

        if !pagination::has_next_page(&res) {
            break; // No more pages
        }

        page += 1;
    }

    println!("Total movies found: {}", movies.len());
    Ok(movies)
}
