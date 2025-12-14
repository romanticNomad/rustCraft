use std::time::Duration;

pub fn test_channel() {
    trpl::block_on(async{
        let (tx, mut rx) = trpl::channel();

        let val = String::from("hi");
        tx.send(val).unwrap();

        let received = rx.recv().await.unwrap();
        println!("received '{received}'");
    });
}

pub fn messege() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();
        let tx_fut = async move{
            let val = vec!["save".to_string(), "the".to_string(), "environment".to_string(),
                                    "its".to_string(), "getting".to_string(), "late".to_string()];
            for message in val {
                tx.send(message).unwrap();
                trpl::sleep(Duration::from_millis(50)).await;
            }
        };

        let rx_fut = async{    
            while let Some(message) = rx.recv().await {
                println!("{}", message);
            }
        };

        trpl::join(tx_fut, rx_fut).await;
    });
}

pub fn multi_messege() {
    trpl::block_on(async{
        let (tx1, mut rx) = trpl::channel();
        let tx2 = tx1.clone();

        let tx_fut1 = async move {
            let val = vec!["I", "upto", "really"];
            for msg1 in val {
                tx1.send(msg1).unwrap();
                trpl::sleep(Duration::from_millis(50)).await;
            }
        };

        let tx_fut2 = async move {
            let val = vec!["am", "something", "cool."];
            for msg2 in val {
                tx2.send(msg2).unwrap();
                trpl::sleep(Duration::from_millis(50)).await;
            };
        };

        let rx_fut = async {
            while let Some(msg) = rx.recv().await {
                println!("{msg}");
            }
        };
        trpl::join!(rx_fut, tx_fut1 ,tx_fut2);
    });
}
