# Diswh

A webhook api for Discord, built in rust. Only supports blocking calls.

# How to use:

```rs
use diswh::{MessageBuilder, WebhookBuilder};

fn main() {
    WebhookBuilder::new("url")
        .send_message(
            MessageBuilder::new("Hello webhook!", false).build()
        );
}
```
