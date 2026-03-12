use tokio::time::{sleep,Duration};

async fn task1(){
    println!("Task 1 started");
    sleep(Duration::from_secs(2)).await;
    println!("Task 1 is completed! after 2 seconds");
}


async fn task2(){
    println!("Task 2 started");
    sleep(Duration::from_secs(10)).await;
    println!("Task 2 is completed! after 10 seconds");
}

// #[tokio::main]
// async fn main(){
    // let t1 = task1();
    // let t2 = task2();

    // let handle = tokio::spawn(async {
    //     sleep(Duration::from_secs(5)).await;
    //     println!("Task is completed! after 5 seconds");
    //     33
    // });

    // println!("Main continues while background task runs...");
    // let result = handle.await.unwrap();
    // println!("Result from background task: {}", result);


//      tokio::select! {
//         _ = sleep(Duration::from_secs(3)) => {
//             println!("Slow path finished first");
//         }
//         _ = sleep(Duration::from_secs(1)) => {
//             println!("Fast path finished first"); // This wins
//         }
// }




// }

// use std::io;

// async fn read_number() -> Result<i32 , String> {
//     Ok(33)
// }

// #[tokio::main]

// async fn main(){
//     match read_number().await {
//         Ok(numb) => println!("Read Number : {}" , numb),
//         Err(e) => println!("Error : {}" , e),
//     }

//     println!("{}" , read_number().await.unwrap());
// }


// use tokio::fs;
// use tokio::io::AsyncWriteExt;

// #[tokio::main]

// async fn main() -> Result<() , Box<dyn std::error::Error>> {
//     let mut file = tokio::fs::File::create("output.txt").await?;
//     file.write_all(b"Hello, Tokio!").await?;
//     println!("File written successfully!");
    
//     //  Read the file that we wrote ok cool async manner

//     let contents = fs::read_to_string("output.txt").await?;
//     println!("File contents: {}", contents);
//     Ok(())
// }


//   Async TCP Server + Client


// use tokio::net::TcpListener;
// use tokio::io::{AsyncReadExt, AsyncWriteExt};


// #[tokio::main]

// async fn main() -> Result<() , Box<dyn std::error::Error>> {
//     let listener = TcpListener::bind("127.0.0.1:1337").await?;
//     println!("Server is listening on port 1337");

//     loop {
//         let (mut socket , addr) = listener.accept().await?;
//         println!("New Connection From : {}" , addr);

//         // Spawn a task per connection - handles many clients concruently

//         tokio::spawn(async move{
//             let mut buf = [0;1024];
//             loop {
//                 let n = match socket.read(&mut buf).await {
//                     Ok(0) => return,
//                     Ok(n) => n,
//                     Err(_) => return,
//                 };

//                 // Echo back

//                 socket.write_all(&buf[..n]).await.unwrap();
//             }
//         });
//     }
    
// }



// use tokio::sync::broadcast;

// #[tokio::main]

// async fn main(){
//     let ( tx , mut rx1 ) = broadcast::channel(16);
//     let mut rx2 = tx.subscribe();

//     tokio::spawn(async move{
//         println!("rx1 got : {}",rx1.recv().await.unwrap());
//     });

//     tokio::spawn(async move{
//         println!("rx2 got : {}" , rx2.recv().await.unwrap());
//     });

//     tx.send("Hello Everyone this is from broadcast message").unwrap();
//     tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
// }


