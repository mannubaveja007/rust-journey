# Axum JWT API

Just a quick project to figure out JWT authentication using the newer Axum (v0.7+) and Tokio.

## How to test it

1. Fire up the server:
   ```bash
   cargo run
   ```

2. Grab a token from the login route:
   ```bash
   curl http://127.0.0.1:8080/login
   ```

3. Copy the token you got back, and pass it into the protected route as an Authorization Bearer header:
   ```bash
   curl -H "Authorization: Bearer <TOKEN_HERE>" http://127.0.0.1:8080/protected
   ```

If your token is valid, you'll hit the secure endpoint. Otherwise, the auth middleware will bounce you with an unauthorized error.

## Screenshots

![Generating Token](<Screenshot 2026-04-21 at 2.38.41 AM.png>)
![Accessing Protected Route](<Screenshot 2026-04-21 at 2.38.47 AM.png>)
