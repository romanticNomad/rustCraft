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
