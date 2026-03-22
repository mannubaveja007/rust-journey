use std::time::Duration;
use tokio::time::sleep;
use tokio::sync::mpsc; // multiple producer , single consumer
use tokio::sync::Semaphore;
use std::sync::Arc;


async fn scrape(url : &str) -> String{
    sleep(Duration::from_secs(1)).await;
    format!("Scraped : {}",url)
}

#[tokio::main]
async fn main() {

    let sites = vec!["google.com", "rust-lang.org", "github.com", "amazon.com", "reddit.com"];
    println!("Hello, world!");

    // let output = vec![];

    let sem = Arc::new(Semaphore::new(2));
    let mut handles = vec![]; 

    for url in sites {
        let permit = Arc::clone(&sem).acquire_owned().await.unwrap();
        let url = url.to_string(); 

        let handle = tokio::spawn(async move {
            let result = scrape(&url).await;
            drop(permit);
            result
        });
        
        // PUSH IT HERE! Inside the loop!
        handles.push(handle);
    } // <-- End of the for loop

    let mut final_results = vec![];

    for handle in handles{
        final_results.push(handle.await.unwrap());
    }

    println!("{:?}",final_results);

}
