use scraper::{Html, Selector};

// takes in the pagination selector string and finds the pages, should return true if there is a next page
pub fn has_next_page(html: &str) -> bool{
    let document = Html::parse_document(html);
    let next_selector = Selector::parse("div.pagination a.next").unwrap();
    document.select(&next_selector).next().is_some()
}