# Diswh

A webhook api for Discord, built in rust. Async can be enabled using the `async` feature. Then you must use one of the many async dispatchers like `tokio` or `async-std`!

# How to use:

```rs
use diswh::{MessageBuilder, WebhookBuilder};

fn main() -> Result<(), std::error::Error> {
    WebhookBuilder::new("url")
        .send_message(
            MessageBuilder::new("Hello webhook!", false).build()
        )?;
}
```

Or for Async use:

```rs
use diswh::{MessageBuilder, WebhookBuilder};

async fn main() -> Result<(), std::error::Error> {
    WebhookBuilder::new("url")
        .send_message_async(
            MessageBuilder::new("Hello webhook!", false).build()
        ).await?;
}
```
