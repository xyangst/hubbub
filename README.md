# Hubbub
Rust library for creating discord self-bots (very early in development)

## Current features
- Connects to gateway with token
- Supports reconnecting to gateway (untested, should work)
- Supports making calls to discords http api
- Supports sending and recieving gateway events
- *Some* of discords many, MANY, data structures have been translated into serde-compatible structs

## Using the library
1. Add the library to your project using `cargo add hubbub`
2. Import `hubbub::prelude::*:`
3. Create the client
```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = Client::new(/* Event handler */).await?;

    client.token(/* Token */).await?;
    client.login().await?;
    client.run().await?;
}
```
4. Create the event handler
```rust
Box::from(
    |ctx: Arc<Mutex<Context>>, ws: Arc<Mutex<Websocket>>, msg: DiscordMessage| async move {
        /* do work here */
    }
)
```
5. Success, hopefully!

## Any questions?
- Look at the examples
- Look at the `Context` and `DiscordMessage` structs
