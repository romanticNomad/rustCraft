use std::{thread, time::Duration};

fn halt(msg: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("{msg} halted for {ms} ms");
}

pub fn await_sleep() {
    trpl::block_on(async {
        let a = async {
            println!("a startes here");
            halt("a", 100);
            halt("a", 120); // these timstamps will not let execution of b start immidiatly
            // trpl::sleep(Duration::from_millis(200)).await;
            println!("a stops here");
        };

        let b = async {
            println!("b starts here");
            halt("b", 130);
            halt("b", 170); // these timestamps will block the execution of 'a'.
            trpl::sleep(Duration::from_millis(200)).await;
            println!("b stops here");
        };

        trpl::select(a, b).await; // we don't use join but select to see runtime control is transfered
    })
}

pub fn await_nosleep() {
    trpl::block_on(async {
        let a = async {
            println!("a startes here");
            halt("a", 100);
            trpl::sleep(Duration::from_millis(1)).await; // manually passing the conctrol to runtime which switches it to b, but puts a sleep.
            halt("a", 120);
            trpl::sleep(Duration::from_millis(1)).await;
            println!("a stops here");
        };

        let b = async {
            println!("b starts here");
            halt("b", 130);
            trpl::sleep(Duration::from_millis(1)).await;
            halt("b", 170);
            trpl::sleep(Duration::from_millis(1)).await;
            println!("b stops here");
        };

        trpl::select(a, b).await; // we don't use join but select to see runtime control is transfered
    })
}

pub fn await_yield() {
    trpl::block_on(async {
        let a = async {
            println!("a startes here");
            halt("a", 100);
            trpl::yield_now().await; // manually passing the conctrol to runtime which switches it to b.
            halt("a", 120);
            trpl::yield_now().await;
            println!("a stops here");
            trpl::yield_now().await;
        };

        let b = async {
            println!("b starts here");
            halt("b", 130);
            trpl::yield_now().await;
            halt("b", 170);
            trpl::yield_now().await;
            println!("b stops here");
        };

        trpl::select(a, b).await; // we don't use join but select to see runtime control is transfered
    })    
}
