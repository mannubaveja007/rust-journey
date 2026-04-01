use futures_util::StreamExt;
use redis::{self, AsyncTypedCommands, pipe};
use tokio;

// #[tokio::main]
// async fn main() -> redis::RedisResult<()> {
//     println!("Hello, world!");
//     println!("Arshdeep yoyo\n");
//     println!("Connecting to the redis server asynchronously");

//     let client = redis::Client::open(
//         "redis://default:jlSSAuQWiDzAQeTU4YZ29GVurjB1Sc0J@redis-10320.crce220.us-east-1-4.ec2.cloud.redislabs.com:10320",
//     )?;

//     let mut conn = client.get_async_connection().await?;
//     conn.set("key", "value").await?;
//     let value: String = conn.get("key").await?;
//     println!("Value: {}", value);

//     Ok(())
// }

#[tokio::main]
async fn main() {
    let mut r = match redis::Client::open(
        "redis://default:jlSSAuQWiDzAQeTU4YZ29GVurjB1Sc0J@redis-10320.crce220.us-east-1-4.ec2.cloud.redislabs.com:10320",
    ) {
        Ok(client) => match client.get_multiplexed_async_connection().await {
            Ok(conn) => conn,
            Err(e) => {
                println!("Failed to connect to Redis: {e}");
                return;
            }
        },
        Err(e) => {
            println!("Failed to create Redis client: {e}");
            return;
        }
    };
    let mut bruh = match redis::Client::open(
        "redis://default:jlSSAuQWiDzAQeTU4YZ29GVurjB1Sc0J@redis-10320.crce220.us-east-1-4.ec2.cloud.redislabs.com:10320",
    ) {
        Ok(client) => match client.get_connection() {
            Ok(conn) => conn,
            Err(e) => {
                println!("Failed to connect to Redis: {e}");
                return;
            }
        },
        Err(e) => {
            println!("Failed to create Redis client: {e}");
            return;
        }
    };

    r.set("testtt", "just_test").await.unwrap();
    let value = r.get("testtt").await.unwrap();
    match value {
        Some(v) => println!("Got value: {v}"),
        None => println!("Key not found!"),
    }
    bulk_insert(&mut bruh).unwrap();
    publish_example().await.unwrap();
    subscribe_example().await.unwrap();
}

fn bulk_insert(conn: &mut redis::Connection) -> redis::RedisResult<()> {
    let (v1, v2, v3): (bool, i64, String) = pipe()
        .set("key", "random_value")
        .incr("visits", 1)
        .get("key")
        .query(conn)?;

    println!("Set = {v1} , visits = {v2} , Key = {v3}");

    let result: Vec<redis::Value> = pipe()
        .atomic()
        .set("balance", 100)
        .incr("balance", 1)
        .get("balance")
        .query(conn)?;

    println!("Transaction results : {result:?}");
    Ok(())
}

async fn publish_example() -> redis::RedisResult<()> {
    let mut r = match redis::Client::open(
        "redis://default:jlSSAuQWiDzAQeTU4YZ29GVurjB1Sc0J@redis-10320.crce220.us-east-1-4.ec2.cloud.redislabs.com:10320",
    ) {
        Ok(client) => match client.get_multiplexed_async_connection().await {
            Ok(conn) => conn,
            Err(e) => {
                println!("Failed to connect to Redis: {e}");
                return Ok(());
            }
        },
        Err(e) => {
            println!("Failed to create Redis client: {e}");
            return Ok(());
        }
    };

    r.publish("Chat", "Hellooooooo everyone!").await?;
    Ok(())
}

async fn subscribe_example() -> redis::RedisResult<()> {
    let client = redis::Client::open(
        "redis://default:jlSSAuQWiDzAQeTU4YZ29GVurjB1Sc0J@redis-10320.crce220.us-east-1-4.ec2.cloud.redislabs.com:10320",
    )?;

    // get_async_pubsub().await? already unwraps RedisResult<PubSub>
    // giving you a PubSub directly — no inner match needed
    let mut pubsub = client.get_async_pubsub().await?;
    pubsub.subscribe("Chat").await?;
    println!("Subscribed to 'Chat', waiting for messages...");

    let mut stream = pubsub.on_message();
    while let Some(msg) = stream.next().await {
        let payload: String = msg.get_payload()?;
        println!("[Chat] received: {payload}");
    }

    Ok(())
}
