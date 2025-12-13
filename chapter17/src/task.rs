use std::time::Duration;

pub fn task_spawn() {
    trpl::block_on(async {
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("task 1: {i}");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("task 2: {i}");
            trpl::sleep(Duration::from_millis(500)).await;
        }
        handle.await.unwrap();
    });
}