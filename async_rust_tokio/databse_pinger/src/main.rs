use std::time::Duration;
use tokio::time::sleep;
use tokio::sync::mpsc;
use tokio::sync::Semaphore;
use std::sync::Arc;

async fn ping(id : u32) -> String{
    sleep(Duration::from_secs(id%3 +1)).await;
    format!("Server {} is healthy",id);
}



#[tokio::main]
async fn main() {
    let sem = Arc::new(Semaphore::new(3));
    let mut handles = vec![];

    for i in 1..=6 {
        // 1. Grab a permit. We wait here if 3 tasks are already running.
        let permit = Arc::clone(&sem).acquire_owned().await.unwrap();

        // 2. Spawn the background worker
        let handle = tokio::spawn(async move {
            let result = ping(i).await;
            
            // 3. Drop the permit so another task can run
            drop(permit);
            
            result
        });

        // 4. Save the worker handle
        handles.push(handle);
    }
    
    // Now we must wait for all handles to finish
    let mut final_results = vec![];
    for handle in handles {
        final_results.push(handle.await.unwrap());
    }

    println!("{:#?}", final_results);
}