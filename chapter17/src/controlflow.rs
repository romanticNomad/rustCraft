use std::{thread, time::Duration};

fn halt(msg: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("{msg} halted for {ms} ms");
}

pub fn concept() {
    trpl::block_on(async {
        let a = async {
            println!("a startes here");
            halt("a", 100);
            halt("a", 120); // these timstamps will not let execution of b start immidiatly
            trpl::sleep(Duration::from_millis(200)).await;
            println!("a stops here");
        };

        let b = async {
            println!("b starts here");
            halt("b", 130);
            halt("c", 170); // these timestamps will block the execution of 'a'.
            trpl::sleep(Duration::from_millis(200)).await;
            println!("b stops here");
        };

        trpl::select(a, b).await; // select whichevers is working currently
    })
}
