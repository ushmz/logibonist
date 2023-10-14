use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PostMessageArguments {
    channel: String,
    // used as a fallback string of notifications
    text: String,
    blocks: Vec<String>,
    thread_ts: Option<String>,
}

impl PostMessageArguments {
    pub fn new() -> Self {
        Self {
            channel: String::new(),
            text: String::new(),
            blocks: Vec::new(),
            thread_ts: None,
        }
    }

    #[allow(dead_code)]
    pub fn create(
        channel: String,
        text: String,
        blocks: Vec<String>,
        thread_ts: Option<String>,
    ) -> Self {
        Self {
            channel,
            text,
            blocks,
            thread_ts,
        }
    }

    pub fn channel(mut self, channel: String) -> Self {
        self.channel = channel;
        self
    }

    pub fn text(mut self, text: String) -> Self {
        self.text = text;
        self
    }

    pub fn blocks(mut self, blocks: Vec<String>) -> Self {
        self.blocks = blocks;
        self
    }

    pub fn thread_ts(mut self, thread_ts: String) -> Self {
        self.thread_ts = Some(thread_ts);
        self
    }
}
