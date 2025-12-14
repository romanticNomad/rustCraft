use std::time::Duration;
use trpl::Either;


pub async fn timeout<F: Future> (future: F, limit: Duration) -> Result<F::Output, Duration> {
    match trpl::select(future, trpl::sleep(limit)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(limit),
    }
}

pub fn timeout_test() {
    trpl::block_on(async{
        let slow = async {
            trpl::sleep(Duration::from_millis(500)).await;
            "finally finished"
        };

        match timeout(slow, Duration::from_millis(600)).await {
            Ok(output) => println!("{}", output),
            Err(limit) => println!("timed out by {}", limit.as_millis()),
        };
    });
}
