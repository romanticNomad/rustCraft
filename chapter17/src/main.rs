#[allow(dead_code)]
mod scraper;

#[allow(dead_code)]
mod task;


fn main() {
    // scraper::single_url();
    // scraper::url_race();

    // task::task_spawn();
    task::task_async();
}
