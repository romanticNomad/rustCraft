#[allow(dead_code)]
mod scraper;

#[allow(dead_code)]
mod task;

#[allow(dead_code)]
mod channel;

#[allow(dead_code)]
mod controlflow;

#[allow(dead_code)]
mod timeout;


fn main() {
    // scraper::single_url();
    // scraper::url_race();

    // task::task_spawn();
    // task::task_async();

    // channel::test_channel();
    // channel::messege();
    // channel::multi_messege();

    // controlflow::await_sleep();
    // controlflow::await_nosleep();
    // controlflow::await_yield();

    timeout::timeout_test();

}
