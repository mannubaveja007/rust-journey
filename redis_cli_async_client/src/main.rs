use redis::{Client, Commands, RedisError};
use tokio;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    println!("Connecting to the redis CLI for Caching of the messages!...");

    let client = match redis::Client::open(
        "redis://default:jlSSAuQWiDzAQeTU4YZ29GVurjB1Sc0J@redis-10320.crce220.us-east-1-4.ec2.cloud.redislabs.com:10320",
    ) {
        Ok(conn) => conn,
        Err(_e) => return,
    };
    // println!("{:?}", client);
    // this client give us the Result ENUM!
    //
    let mut con = client.get_connection().unwrap();

    // con.set("date", "1th April");
    let set_data: () = con.set("date", "1th April").unwrap();
    let get_data: String = con.get("date").unwrap();
    println!("{:?}", get_data);
    // we can set the data now!
}
