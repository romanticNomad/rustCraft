#[allow(dead_code)]
mod future;

use trpl::Either;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // trpl::block_on(async{
    //     let url = &args[1];
    //     let title = future::page_title(url).await;
    //     match title {
    //         Some(title) => println!("The title at {url} is\n{title}"),
    //         None => println!("No tittle found at {url}"),
    //     }
    // });

    trpl::block_on(async {
        let title1 = future::race_title(&args[1]);
        let title2 = future::race_title(&args[2]);

        let (url, win_title) =
            match trpl::select(title1, title2).await {
                Either::Left(left) => left,
                Either::Right(right) => right, 
            };
        
        println!("{url} returned first:\n");
        match win_title {
            Some(title) => println!("The tile of {url} is\n{title}"),
            None => println!("{url} did not return any title.")
        }
    });
}
