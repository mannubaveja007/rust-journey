// use redis::Commands;
// use tokio::time::{sleep, Duration};


// #[tokio::main]
// async fn main() -> redis::RedisResult<()> {
//     println!("Connecting to redis...");

//     // 1. conencting to redis
//     let client  = redis::Client::open("redis://default:jlSSAuQWiDzAQeTU4YZ29GVurjB1Sc0J@redis-10320.crce220.us-east-1-4.ec2.cloud.redislabs.com:10320")?;

//     let mut con = client.get_connection()?; // connects to the client
//     sleep(Duration::from_secs(2)).await;
//     // 2. Set key - value pair
//     println!("Setting key 'test'....");
//     let _: () = con.set("test","Hellooooo Panchooooooo! Hogeyaaa oyeee Rust me")?;

//     // 3. set the value back
//     let result: String = con.get("test")?;
//     let result2: String = con.get("test:v2")?;
//     println!("Retrieved value: {}",result);
//     println!("Retrieved value: {}",result2);

//     // 4. Perform an integer operation
//     let _: () = con.set("counter",10)?;
//     let new_counter : i32 = con.incr("counter",0)?;
//     println!("Incremented counter: {}", new_counter);

//     Ok(())
// }

// connection Pooling and things

use redis::AsyncCommands;
use tokio::time::{sleep, Duration};
use deadpool_redis::{Config,Runtime};


// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     println!("Connecting to redis pool...");

//     // 1. Configure the pool
    
//     let redis_url  = "redis://default:jlSSAuQWiDzAQeTU4YZ29GVurjB1Sc0J@redis-10320.crce220.us-east-1-4.ec2.cloud.redislabs.com:10320";

//     let cfg = Config::from_url(redis_url);


//     // 2. Create the pool(holds the hourses of connection)

//     let pool = cfg.create_pool(Some(Runtime::Tokio1))?;


//     let mut con = pool.get().await?; // connects to the client

//     sleep(Duration::from_secs(2)).await;


//     // 3. Set key - value pair
//     println!("Setting key 'test'....");
//     let _: () = con.set("test","Hellooooo Panchooooooo! Hogeyaaa oyeee Rust me").await?;

//     // 3. set the value back
//     let result: String = con.get("test").await?;
//     let result2: String = con.get("test:v2").await?;
//     println!("Retrieved value: {}",result);
//     println!("Retrieved value: {}",result2);

//     // 4. Perform an integer operation
//     let _: () = con.set("counter",10).await?;
//     let new_counter : i32 = con.incr("counter",0).await?;
//     println!("Incremented counter: {}", new_counter);

//     Ok(())
// }

// Concurrent Tasks using the pool



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Connecting to redis pool...");
    let mut tasks = vec![];



    // 1. Configure the pool
    
    let redis_url  = "redis://default:jlSSAuQWiDzAQeTU4YZ29GVurjB1Sc0J@redis-10320.crce220.us-east-1-4.ec2.cloud.redislabs.com:10320";

    let cfg = Config::from_url(redis_url);


    // 2. Create the pool(holds the hourses of connection)

    let pool = cfg.create_pool(Some(Runtime::Tokio1))?;


    let mut con = pool.get().await?; // connects to the client

    sleep(Duration::from_secs(2)).await;

        for i in 0..5{
        let pool_clone = pool.clone();

        let task = tokio::spawn(async move{
            let mut con = pool_clone.get().await.unwrap();
            let key = format!("task_key_{}",i);

            let _:() = con.set(&key,i).await.unwrap();
            let val : i32 = con.get(&key).await.unwrap();

            println!("Task {} set and retrieved: {}" , i , val);
        });
        tasks.push(task);
    }

    for task in tasks{
        task.await?;
    }
   
    let mut con = pool.get().await?; 
    sleep(Duration::from_secs(2)).await;

    println!("Setting key 'test'....");
    let _: () = con.set("test","Hellooooo Panchooooooo! Hogeyaaa oyeee Rust me").await?;

    let result: String = con.get("test").await?;
    println!("Retrieved value: {}",result);

    let _: () = con.set("counter",10).await?;
    let new_counter : i32 = con.incr("counter",0).await?;
    println!("Incremented counter: {}", new_counter);

    Ok(())
}

