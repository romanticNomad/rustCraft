#[allow(dead_code)]
mod future;

#[allow(dead_code)]
mod scraper;


fn main() {
    scraper::single_url();
    // scraper::url_race();
}
