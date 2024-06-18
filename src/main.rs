mod lib;

use lib::IterJoinSet;
use std::pin::Pin;

async fn print_and_return(x: usize) -> usize {
    println!("Gracias {x}");
    x
}

#[tokio::main]
async fn main() {
    let mut js = IterJoinSet::<usize>::new();
    let mut futs: Vec<Pin<Box<dyn futures::Future<Output = usize> + Send + 'static>>> = vec![];
    (0..100).for_each(|x| {
        let b = Box::pin(print_and_return(x));
        futs.push(b);
    });
    let bfuts = Box::new(futs.into_iter());
    js.spawn_iter(bfuts);
    while let Some(i) = js.join_next().await {
        println!("{}", i.unwrap());
    }
}
