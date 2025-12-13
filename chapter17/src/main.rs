#[allow(dead_code)]
mod future;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    trpl::block_on(async{
        let url = &args[1];
        let (_, title) = future::page_title(url).await;
        match title {
            Some(title) => println!("The title at {url} is\n{title}"),
            None => println!("No tittle found at {url}"),
        }
    });


}
