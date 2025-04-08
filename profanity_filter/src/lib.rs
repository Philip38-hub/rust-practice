pub struct Message {
    content: String,
    user: String,
}

impl Message {
    pub fn new(content: String, user: String) -> Self {
        Self { content, user }
    }

    pub fn send_ms(&self) -> Option<String> {
        if self.content.is_empty() || self.content.contains("stupid") {
            None
        } else {
            Some(self.content.clone())
        }
    }
}

pub fn check_ms(message: &str) -> Result<String, String> {
    let msg = Message::new(message.to_string(), "anonymous".to_string());
    match msg.send_ms() {
        Some(valid_msg) => Ok(valid_msg),
        None => Err("ERROR: illegal".to_string()),
    }
}
