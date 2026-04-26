# MongoDB Rust Tutorial

This is a small Rust project that connects to a local MongoDB database and fetches one stock document.

Right now it:

- connects to `mongodb://localhost:27017/`
- uses the database `stockDB`
- reads from the `stocks` collection
- looks for a document where `name = "Tesla"`

## Run It

Make sure MongoDB is running locally, then start the app with:

```bash
cargo run
```

## Sample MongoDB Data

If you want to test it quickly, insert this in MongoDB:

```javascript
use stockDB

db.stocks.insertOne({
  name: "Tesla",
  symbol: "TSLA"
})
```

## Output

The program prints the matching document if it finds one. If not, it prints `None`.

## Main File

The code is in [src/main.rs](/Users/mannubaveja/temp/rust/MongoDB-Rust-tutorial/src/main.rs:1).

## Video

https://www.tella.tv/video/mannus-video-1a5p
