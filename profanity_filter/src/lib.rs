pub struct Message<'a> {
    content: &'a str,
    user: &'a str,
}

impl<'a> Message<'a> {
    pub fn new(content: &'a str, user: &'a str) -> Self {
        Self { content, user }
    }

    pub fn send_ms(&self) -> Option<&'a str> {
        if self.content.is_empty() || self.content.contains("stupid") {
            None
        } else {
            Some(self.content)
        }
    }
}

pub fn check_ms(message: &str) -> Result<&str, &str> {
    let msg = Message::new(message, "anonymous");
    match msg.send_ms() {
        Some(valid_msg) => Ok(valid_msg),
        None => Err("ERROR: illegal"),
    }
}
