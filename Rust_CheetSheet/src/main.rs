use redis::{AsyncCommands, Commands, Connection, RedisResult};
use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() -> RedisResult<()> {
    // Initialize the Array
    let list = vec!["name", "name1"];
    // For loop to print the values in side the array
    // for i in &list {
    //     println!("{}", i);
    // }
    println!("Connecting to the redis async....");
    sleep(Duration::from_secs(3)).await;
    println!("Connected to the string!");

    // Connecting to the Redis Client
    let client = redis::Client::open("redis://127.0.0.1:6379/").unwrap();
    // Getting the Connection out of the Client
    let mut conn = client.get_multiplexed_async_connection().await.unwrap();

    // some basic learning stuff experimenting!
    // Game On!

    // let result_ttl: i32 = conn.set_ex("name1", "System Design Class", 3600u64).await?;
    //
    let result_all: () = conn.get("*").await?;
    let ttl: i64 = conn.ttl("name1").await?;
    for i in &list {
        println!("{}", i);
        let result: String = conn.get(i).await?;
        println!("{result}");
    }
    println!(
        "The value that we are storing is :  {}  it's TTL is  {:?}",
        ttl, result_all
    );
    Ok(())
}
