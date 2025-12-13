#[allow(dead_code)]
mod future;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    trpl::block_on(async{
        let url = &args[1];
        match future::page_title(url).await {
            Some(title) => println!("The title at {url} is\n{title}"),
            None => println!("No tittle found at {url}"),
        }
    });
}
