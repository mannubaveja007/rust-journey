# 🔴 Redis Cheatsheet — Commands + redis-rs (Rust)

A complete reference for every Redis data type, command, and its rust-rs equivalent.

---

## 📦 Setup

### Cargo.toml
```toml
[dependencies]
redis = { version = "1.1", features = ["tokio-comp"] }
tokio = { version = "1", features = ["full"] }
```

### Connecting
```rust
// Async (recommended)
use redis::AsyncCommands;

let client = redis::Client::open("redis://127.0.0.1:6379/")?;
let mut con = client.get_multiplexed_async_connection().await?;

// With password + custom port
let client = redis::Client::open("redis://default:PASSWORD@host:PORT")?;

// Sync (simple scripts)
use redis::Commands;
let client = redis::Client::open("redis://127.0.0.1/")?;
let mut con = client.get_connection()?;
```

---

## 🔑 Key Rules

```
// Key naming — use colons as separators (namespacing)
user:1                 // a user with id 1
user:1:sessions        // sessions for user 1
paste:abc123           // a paste with code abc123
ratelimit:192.168.1.1  // rate limit for an IP

// Wildcards for pattern matching
*           matches everything
?           matches one character
paste:*     matches paste:abc, paste:xyz123, etc.
paste:????  matches paste keys with exactly 4 char codes
```

---

## 1️⃣ Strings — The Simplest Type

The most basic type. Key → single value (text, number, serialized JSON).

### Redis CLI
```
SET    name "Alice"           # store a string
GET    name                   # → "Alice"
DEL    name                   # delete the key
EXISTS name                   # → 1 (exists) or 0 (not found)
APPEND name " Smith"          # append to existing value → "Alice Smith"

# With expiry
SET    session "abc123" EX 3600   # expires in 1 hour
SETEX  session 3600 "abc123"      # same thing, older syntax
TTL    session                    # → seconds remaining
PERSIST session                   # remove TTL, make it permanent

# Counters
SET    visits 0
INCR   visits          # → 1  (increment by 1)
INCRBY visits 5        # → 6  (increment by 5)
DECR   visits          # → 5  (decrement by 1)
DECRBY visits 2        # → 3  (decrement by 2)

# Get and set at once
GETSET name "Bob"      # returns old value "Alice", sets new value "Bob"
MSET k1 v1 k2 v2      # set multiple keys at once
MGET k1 k2             # get multiple values at once
```

### redis-rs
```rust
use redis::AsyncCommands;

// Basic set / get
let _: () = con.set("name", "Alice").await?;
let name: String = con.get("name").await?;

// Optional get (key might not exist)
let val: Option<String> = con.get("maybe_missing").await?;

// Set with TTL
let _: () = con.set_ex("session", "abc123", 3600u64).await?;

// Check TTL
let secs: i64 = con.ttl("session").await?;
// -2 = does not exist, -1 = no expiry, N = seconds left

// Delete
let deleted: i64 = con.del("name").await?;
// 1 = deleted, 0 = did not exist

// Check existence
let exists: bool = con.exists("name").await?;

// Counters
let _: () = con.set("visits", 0).await?;
let count: i64 = con.incr("visits", 1).await?;   // +1
let count: i64 = con.incr("visits", 5).await?;   // +5
let count: i64 = con.decr("visits", 1).await?;   // -1

// Multiple keys at once
let _: () = con.mset(&[("k1", "v1"), ("k2", "v2")]).await?;
let vals: Vec<String> = con.mget(&["k1", "k2"]).await?;
```

---

## 2️⃣ Hashes — Structs in Redis

Store multiple fields under one key. Perfect for objects/structs.

### Redis CLI
```
HSET    user:1 name "Alice"          # set one field
HSET    user:1 email "a@b.com"       # set another field
HMSET   user:1 name "Alice" age "30" # set multiple fields (older syntax)
HSET    user:1 name "Alice" age "30" # set multiple (newer syntax, same command)

HGET    user:1 name              # → "Alice"
HMGET   user:1 name email        # → ["Alice", "a@b.com"]
HGETALL user:1                   # → { name: Alice, email: a@b.com, age: 30 }
HKEYS   user:1                   # → ["name", "email", "age"]
HVALS   user:1                   # → ["Alice", "a@b.com", "30"]
HLEN    user:1                   # → 3 (number of fields)

HDEL    user:1 email             # delete one field
HEXISTS user:1 name              # → 1 (field exists) or 0

HINCRBY user:1 age 1             # increment numeric field by 1 → 31
HINCRBYFLOAT user:1 score 1.5    # increment by float
```

### redis-rs
```rust
use redis::AsyncCommands;

// Set one field
let _: () = con.hset("user:1", "name", "Alice").await?;

// Set multiple fields at once
let _: () = con.hset_multiple("user:1", &[
    ("name",  "Alice"),
    ("email", "a@b.com"),
    ("age",   "30"),
]).await?;

// Get one field
let name: String = con.hget("user:1", "name").await?;

// Get all fields as HashMap
use std::collections::HashMap;
let data: HashMap<String, String> = con.hgetall("user:1").await?;
let name = data.get("name").cloned().unwrap_or_default();

// Check if field exists
let exists: bool = con.hexists("user:1", "email").await?;

// Delete a field
let _: () = con.hdel("user:1", "email").await?;

// Increment a numeric field
let age: i64 = con.hincr("user:1", "age", 1).await?;

// Set TTL on the whole hash key
let _: () = con.expire("user:1", 3600).await?;
```

---

## 3️⃣ Lists — Ordered, Allows Duplicates

A list of strings in insertion order. Great for queues, feeds, logs.

```
HEAD ← [task-1, task-2, task-3] → TAIL
LPUSH adds to HEAD              RPUSH adds to TAIL
LPOP  removes from HEAD         RPOP  removes from TAIL
```

### Redis CLI
```
LPUSH tasks "task-1"       # push to HEAD → [task-1]
LPUSH tasks "task-2"       # push to HEAD → [task-2, task-1]
RPUSH tasks "task-3"       # push to TAIL → [task-2, task-1, task-3]

LPOP  tasks                # remove + return HEAD → "task-2"
RPOP  tasks                # remove + return TAIL → "task-3"

LRANGE tasks 0 -1          # get all items → ["task-1"]
LRANGE tasks 0 2           # get first 3 items
LLEN   tasks               # → length of list

# Blocking pop — WAITS until an item is available (perfect for job queues!)
BRPOP tasks 30             # block for up to 30 seconds waiting for an item
BLPOP tasks 30             # same but from the left

LINDEX tasks 0             # get item at index 0 (no removal)
LSET   tasks 0 "new-task"  # update item at index 0
LREM   tasks 1 "task-1"    # remove 1 occurrence of "task-1"
```

### redis-rs
```rust
use redis::AsyncCommands;

// Push items
let _: () = con.lpush("tasks", "task-1").await?;
let _: () = con.rpush("tasks", "task-3").await?;

// Pop items
let item: Option<String> = con.lpop("tasks", None).await?;
let item: Option<String> = con.rpop("tasks", None).await?;

// Get range (0 to -1 = everything)
let all: Vec<String> = con.lrange("tasks", 0, -1).await?;

// Length
let len: i64 = con.llen("tasks").await?;

// Blocking pop — waits up to 30s for an item (great for workers!)
let result: Option<(String, String)> = con.brpop("tasks", 30.0).await?;
if let Some((key, value)) = result {
    println!("Got job from {key}: {value}");
}
```

---

## 4️⃣ Sets — Unique Values, No Order

Like a List but every item is **unique** and there is **no order**.
Great for: online users, tags, unique visitors.

### Redis CLI
```
SADD   online "alice"      # add member
SADD   online "bob"
SADD   online "alice"      # duplicate — ignored, still only 1 alice

SMEMBERS online            # → { alice, bob }
SCARD    online            # → 2 (count of members)
SISMEMBER online "alice"   # → 1 (is alice in the set?)
SRANDMEMBER online         # → random member
SPOP   online              # remove and return a random member
SREM   online "bob"        # remove specific member

# Set operations (like math sets)
SUNION  set1 set2          # all members from both sets (union)
SINTER  set1 set2          # only members in BOTH sets (intersection)
SDIFF   set1 set2          # members in set1 but NOT in set2
```

### redis-rs
```rust
use redis::AsyncCommands;

// Add members
let _: () = con.sadd("online", "alice").await?;
let _: () = con.sadd("online", "bob").await?;

// Get all members
let members: Vec<String> = con.smembers("online").await?;

// Count
let count: i64 = con.scard("online").await?;

// Check membership (O(1) — instant!)
let is_online: bool = con.sismember("online", "alice").await?;

// Remove
let _: () = con.srem("online", "bob").await?;

// Set operations
let union: Vec<String> = con.sunion(&["set1", "set2"]).await?;
let inter: Vec<String> = con.sinter(&["set1", "set2"]).await?;
let diff:  Vec<String> = con.sdiff(&["set1", "set2"]).await?;
```

---

## 5️⃣ Sorted Sets — Unique Values WITH a Score

Like a Set but every member has a **numeric score** used for ordering.
Perfect for: leaderboards, priority queues, time-series.

```
Member  Score
alice   9500
bob     7200
carol   11000

Sorted automatically by score (low → high by default)
```

### Redis CLI
```
ZADD  leaderboard 9500  "alice"    # add with score
ZADD  leaderboard 7200  "bob"
ZADD  leaderboard 11000 "carol"

ZRANGE    leaderboard 0 -1              # all members, low → high score
ZRANGE    leaderboard 0 -1 WITHSCORES  # with scores
ZREVRANGE leaderboard 0 -1             # all members, high → low score (top first)
ZREVRANGE leaderboard 0 2              # top 3

ZSCORE  leaderboard "alice"        # → 9500.0 (alice's score)
ZRANK   leaderboard "alice"        # → 1 (alice's rank, 0-indexed, low=best)
ZREVRANK leaderboard "alice"       # → 1 (rank from top, 0 = #1)

ZINCRBY leaderboard 500 "alice"    # add 500 to alice's score → 10000
ZREM    leaderboard "bob"          # remove bob
ZCARD   leaderboard                # → count of members

# Range by score
ZRANGEBYSCORE leaderboard 7000 10000   # members with score 7000–10000
ZCOUNT leaderboard 7000 10000          # count members in score range
```

### redis-rs
```rust
use redis::AsyncCommands;

// Add members with scores
let _: () = con.zadd("leaderboard", "alice", 9500).await?;
let _: () = con.zadd("leaderboard", "bob",   7200).await?;
let _: () = con.zadd("leaderboard", "carol", 11000).await?;

// Top N (highest score first)
let top3: Vec<String> = con.zrevrange("leaderboard", 0, 2).await?;
// → ["carol", "alice", "bob"]

// With scores
let top3: Vec<(String, f64)> = con.zrevrange_withscores("leaderboard", 0, 2).await?;
// → [("carol", 11000.0), ("alice", 9500.0), ("bob", 7200.0)]

// Get a member's score
let score: f64 = con.zscore("leaderboard", "alice").await?;

// Get a member's rank (0 = lowest score)
let rank: i64 = con.zrank("leaderboard", "alice").await?;

// Get rank from top (0 = highest score = #1 place)
let rank: i64 = con.zrevrank("leaderboard", "alice").await?;

// Increment score
let new_score: f64 = con.zincr("leaderboard", "alice", 500.0).await?;

// Remove member
let _: () = con.zrem("leaderboard", "bob").await?;

// Count total members
let count: i64 = con.zcard("leaderboard").await?;
```

---

## 6️⃣ Pub/Sub — Real-Time Messaging

Publisher sends messages to a channel. All subscribers receive them instantly.
Great for: chat, notifications, live updates.

### Redis CLI
```
SUBSCRIBE  chat            # listen on "chat" channel (blocks)
PUBLISH    chat "Hello!"   # send message to all subscribers
PSUBSCRIBE news:*          # subscribe to all channels matching pattern
```

### redis-rs
```rust
// PUBLISHER
use redis::AsyncCommands;
let client = redis::Client::open("redis://127.0.0.1/")?;
let mut con = client.get_multiplexed_async_connection().await?;
let _: () = con.publish("chat", "Hello everyone!").await?;

// SUBSCRIBER
use futures_util::StreamExt;
let client = redis::Client::open("redis://127.0.0.1/")?;
let mut pubsub = client.get_async_pubsub().await?;
pubsub.subscribe("chat").await?;

let mut stream = pubsub.on_message();
while let Some(msg) = stream.next().await {
    let payload: String = msg.get_payload()?;
    println!("[chat] {payload}");
}
```

---

## 7️⃣ Expiry / TTL Commands

```rust
// Set expiry (seconds)
let _: () = con.expire("key", 3600).await?;

// Set expiry (milliseconds)
let _: () = con.pexpire("key", 3600000).await?;

// Set expiry as Unix timestamp
let _: () = con.expireat("key", 1893456000).await?;

// Remove expiry (make permanent)
let _: () = con.persist("key").await?;

// Check TTL in seconds
let secs: i64 = con.ttl("key").await?;
// -2 = key does not exist
// -1 = key has no expiry
//  N = seconds remaining

// Check TTL in milliseconds
let ms: i64 = con.pttl("key").await?;
```

---

## 8️⃣ Key Scanning

```rust
// KEYS — simple but blocks Redis (only use in dev!)
let keys: Vec<String> = con.keys("paste:*").await?;

// SCAN — non-blocking, production safe
use futures_util::StreamExt;
let mut iter: redis::AsyncIter<String> = con.scan_match("paste:*").await?;
let mut keys = vec![];
while let Some(key) = iter.next_item().await {
    keys.push(key);
}

// SCAN with count hint (Redis may return more or fewer)
// Not directly in redis-rs, use scan_match which handles pagination
```

---

## 9️⃣ Transactions (MULTI/EXEC)

Group commands so they all execute atomically — no other client can interrupt.

### Redis CLI
```
MULTI              # start transaction
SET balance 100
INCR balance
GET balance
EXEC               # execute all at once → [OK, 101, "101"]
DISCARD            # cancel transaction
```

### redis-rs (Pipeline + atomic)
```rust
use redis::pipe;

// Atomic pipeline = MULTI/EXEC
let (set_ok, new_balance, balance): (bool, i64, i64) = pipe()
    .atomic()                        // wraps in MULTI/EXEC
    .set("balance", 100)
    .incr("balance", 1)
    .get("balance")
    .query_async(&mut con)
    .await?;

println!("balance = {balance}");    // 101
```

---

## 🔟 Pipelining — Batch Commands

Send multiple commands in one network round-trip (no MULTI/EXEC wrapper).
Faster than sending commands one by one.

```rust
use redis::pipe;

// Non-atomic pipeline
let (v1, v2, v3): (bool, i64, String) = pipe()
    .set("key", "hello")
    .incr("counter", 1)
    .get("key")
    .query_async(&mut con)
    .await?;
```

---

## 1️⃣1️⃣ Common Patterns

### Cache-aside (most common caching pattern)
```rust
// Try cache first, fallback to DB, then cache the result
async fn get_user(con: &mut impl AsyncCommands, id: &str) -> User {
    let key = format!("user:{id}");

    // 1. Check cache
    if let Ok(cached) = con.get::<_, String>(&key).await {
        return serde_json::from_str(&cached).unwrap();
    }

    // 2. Cache miss — fetch from DB
    let user = db_get_user(id).await;

    // 3. Store in cache for 5 minutes
    let json = serde_json::to_string(&user).unwrap();
    let _: () = con.set_ex(&key, json, 300u64).await.unwrap();

    user
}
```

### Rate limiter
```rust
async fn is_allowed(con: &mut impl AsyncCommands, ip: &str) -> bool {
    let key = format!("ratelimit:{ip}");
    let count: i64 = con.incr(&key, 1).await.unwrap_or(1);
    if count == 1 {
        let _: () = con.expire(&key, 60).await.unwrap();
    }
    count <= 100  // allow max 100 requests per minute
}
```

### Session store
```rust
async fn create_session(con: &mut impl AsyncCommands, user_id: &str) -> String {
    let token = Uuid::new_v4().to_string();
    let key   = format!("session:{token}");
    let _: () = con.set_ex(&key, user_id, 86400u64).await.unwrap(); // 24hr
    token
}

async fn get_session(con: &mut impl AsyncCommands, token: &str) -> Option<String> {
    con.get(format!("session:{token}")).await.unwrap_or(None)
}
```

### Job queue (producer + consumer)
```rust
// Producer — push job
async fn enqueue(con: &mut impl AsyncCommands, job: &str) {
    let _: () = con.lpush("jobs:pending", job).await.unwrap();
}

// Consumer — blocking pop, waits for jobs
async fn worker(mut con: impl AsyncCommands) {
    loop {
        let result: Option<(String, String)> = con
            .brpop("jobs:pending", 30.0)
            .await
            .unwrap_or(None);

        if let Some((_, job)) = result {
            println!("Processing: {job}");
            // do work...
        }
    }
}
```

### Leaderboard
```rust
// Update score
async fn update_score(con: &mut impl AsyncCommands, player: &str, score: f64) {
    let _: () = con.zadd("leaderboard", player, score).await.unwrap();
}

// Get top N players
async fn top_players(con: &mut impl AsyncCommands, n: isize) -> Vec<(String, f64)> {
    con.zrevrange_withscores("leaderboard", 0, n - 1).await.unwrap()
}

// Get a player's rank (1-indexed for display)
async fn player_rank(con: &mut impl AsyncCommands, player: &str) -> i64 {
    let rank: i64 = con.zrevrank("leaderboard", player).await.unwrap_or(-1);
    rank + 1  // convert 0-indexed to 1-indexed
}
```

---

## ⚡ Quick Reference Card

| Type        | Best For                          | Key Commands                              |
|-------------|-----------------------------------|-------------------------------------------|
| String      | Counters, cache, sessions, flags  | SET GET DEL INCR DECR SETEX TTL           |
| Hash        | Structs, objects, user profiles   | HSET HGET HGETALL HDEL HINCRBY            |
| List        | Queues, feeds, logs               | LPUSH RPUSH LPOP RPOP LRANGE BRPOP        |
| Set         | Unique items, tags, online users  | SADD SMEMBERS SISMEMBER SREM SUNION SINTER|
| Sorted Set  | Leaderboards, rankings, scores    | ZADD ZREVRANGE ZSCORE ZRANK ZINCRBY       |
| Pub/Sub     | Real-time messaging, notifications| PUBLISH SUBSCRIBE PSUBSCRIBE              |

---

## 🚨 Things to NEVER Do in Production

```
KEYS *              # blocks Redis — use SCAN instead
FLUSHALL            # deletes EVERYTHING in Redis
FLUSHDB             # deletes everything in current DB
DEBUG SLEEP 10      # artificially blocks Redis
```

---

## 🔗 Useful Links

- redis-rs docs:     https://docs.rs/redis/latest/redis/
- Redis commands:    https://redis.io/commands/
- Redis data types:  https://redis.io/docs/data-types/
- Try Redis online:  https://try.redis.io/