# MongoDB Rust Tutorial

This is a simple Rust project I made to test how Rust can talk to MongoDB through a small backend server.

The app runs on `Axum`, connects to a local MongoDB instance, and checks the `stocks` collection inside `stockDB`. Right now it looks for a stock with the name `Tesla` and returns that document as JSON when you open the root route.

If you want to run it locally, first make sure MongoDB is running on `mongodb://localhost:27017/`. After that, start the project with:

```bash
cargo run
```

The server will start on `http://127.0.0.1:3000`.

To test it properly, you can add a sample document in MongoDB:

```javascript
use stockDB

db.stocks.insertOne({
  name: "Tesla",
  symbol: "TSLA"
})
```

Then open `http://127.0.0.1:3000` in your browser or call it with any API tool. If the document exists, you will get it back as JSON. If not, the app will return `Stock not found`.

The main code is in [src/main.rs](/Users/mannubaveja/temp/rust/MongoDB-Rust-tutorial/src/main.rs:1).

Demo video: https://www.tella.tv/video/vid_cmoff9uvn009u04l4cw32hxw4/view?quick
