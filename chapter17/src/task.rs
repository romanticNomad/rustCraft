use std::time::Duration;

pub fn task_spawn() {
    trpl::block_on(async {
        let _handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("task 1: {i}");
                trpl::sleep(Duration::from_millis(5)).await;
            }
        });

        for i in 1..5 {
            println!("task 2: {i}");
            trpl::sleep(Duration::from_millis(5)).await;
        }
        // handle.await.unwrap();
    });
}

// crating futures of the 2 async block and using trpl::join adds an order to the concurrency system
pub fn task_async() {
    trpl::block_on(async{
        let fut1 = async {
            for i in 1..10 {
                println!("task1: {i}");
                trpl::sleep(Duration::from_millis(5)).await;
            };
        };

        let fut2 = async {
            for i in 1..5 {
                println!("task2: {i}");
                trpl::sleep(Duration::from_millis(5)).await;
            };
        };

        trpl::join(fut1, fut2).await;
    });
}
