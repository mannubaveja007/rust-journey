use redis::Commands;
use tokio::time::{sleep, Duration};


#[tokio::main]
async fn main() -> redis::RedisResult<()> {
    println!("Connecting to redis...");

    // 1. conencting to redis
    let client  = redis::Client::open("redis://default:jlSSAuQWiDzAQeTU4YZ29GVurjB1Sc0J@redis-10320.crce220.us-east-1-4.ec2.cloud.redislabs.com:10320")?;

    let mut con = client.get_connection()?; // connects to the client
    sleep(Duration::from_secs(2)).await;
    // 2. Set key - value pair
    println!("Setting key 'test'....");
    let _: () = con.set("test","Hellooooo Panchooooooo! Hogeyaaa oyeee Rust me")?;

    // 3. set the value back
    let result: String = con.get("test")?;
    let result2: String = con.get("test:v2")?;
    println!("Retrieved value: {}",result);
    println!("Retrieved value: {}",result2);

    // 4. Perform an integer operation
    let _: () = con.set("counter",10)?;
    let new_counter : i32 = con.incr("counter",11)?;
    println!("Incremented counter: {}", new_counter);

    Ok(())
}
