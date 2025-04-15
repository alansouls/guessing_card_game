use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum MessageType {
    PlayerJoin,
    Guess,
    PlayCard,
    UpdateState,
}

pub struct MessageParam {
    pub key: String,
    pub value: String,
}

pub struct GameMessage {
    pub player_id: usize,
    pub message_type: MessageType,
    pub message_params: Vec<MessageParam>,
}

impl ToString for GameMessage {
    fn to_string(&self) -> String {
        let mut message = format!("{}|", self.player_id);
        message.push_str(&format!("{:?}", self.message_type));
        for param in &self.message_params {
            message.push_str(&format!("|{}={}", param.key, param.value));
        }
        message
    }
}

impl FromStr for MessageType {
    type Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PlayerJoin" => Ok(MessageType::PlayerJoin),
            "Guess" => Ok(MessageType::Guess),
            "PlayCard" => Ok(MessageType::PlayCard),
            "UpdateState" => Ok(MessageType::UpdateState),
            _ => Err(()),
        }
    }
}