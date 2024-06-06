use async_openai::types::{CreateMessageRequest, MessageContent, MessageObject};

use crate::Result;

// region:    --- Message Constructors

pub fn user_msg(content: impl Into<String>) -> CreateMessageRequest {
    CreateMessageRequest {
        role: "user".to_string(),
        content: content.into(),
        ..Default::default()
    }
}

// endregion: --- Message Constructors

// region:    --- Content Extractor

pub fn get_text_content(msg:MessageObject) -> Result<String> {
    let msg_content = msg
        .content
        .into_iter()
        .next()
        .ok_or_else(|| "no message content fount".to_string())?;

    // -- Get the text
    let txt=match msg_content {
        MessageContent::Text(text) => text.text.value,
        MessageContent::ImageFile(_) => {
            return Err("Message image not supported yet".into())
        }
    };

    Ok(txt)
}

// endregion: --- Content Extractor

