# Diswh

A webhook api for Discord, built in rust. Supports both blocking and nonblocking by default!

# How to use:

```rs
use diswh::{MessageBuilder, WebhookBuilder};

#[tokio::main]
async fn main() {
    WebhookBuilder::new("url")
        .send_message_async(
            MessageBuilder::new("Hello webhook!", false).build()
        ).await;
}
```
