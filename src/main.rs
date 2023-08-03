use tokio;

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        // Do some async work
        "return value"
    });
    // Do some other work
    let out = handle.await.unwrap();
    println!("GOT {}", out);
}

// async fn say_world() {
//     println!("world");
// }
// #[tokio::main]
// async fn main() {
//     // Calling `say_world()` does not execute the body of `say_world()`.
//     let op = say_world();
//     // This println! comes first
//     println!("hello");
//     // Calling `.await` on `op` starts executing `say_world`.
//     op.await;
// }
