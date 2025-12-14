#[allow(dead_code)]
mod scraper;

#[allow(dead_code)]
mod task;

#[allow(dead_code)]
mod channel;


fn main() {
    // scraper::single_url();
    // scraper::url_race();

    // task::task_spawn();
    // task::task_async();

    // channel::test_channel();
    // channel::messege();
    channel::multi_messege();
}
