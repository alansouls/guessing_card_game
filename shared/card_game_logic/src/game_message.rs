use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum MessageType {
    //Requests
    PlayerJoin = 0,
    Guess = 1,
    PlayCard = 2,
    StartMatch = 5,

    //Responses
    PlayerJoined = 3,
    UpdateState = 4,
}

impl ToString for MessageType {
    fn to_string(&self) -> String {
        (*self as u8).to_string()
    }
}

impl FromStr for MessageType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(MessageType::PlayerJoin),
            "1" => Ok(MessageType::Guess),
            "2" => Ok(MessageType::PlayCard),
            "3" => Ok(MessageType::PlayerJoined),
            "4" => Ok(MessageType::UpdateState),
            "5" => Ok(MessageType::StartMatch),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq)]
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
        message.push_str(&format!("{}", self.message_type.to_string()));
        for param in &self.message_params {
            message.push_str(&format!("|{}|{}", param.key, param.value));
        }
        message
    }
}

impl FromStr for GameMessage {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('|').collect();
        if parts.len() < 2 {
            return Err(());
        }
        let player_id = parts[0].parse::<usize>().map_err(|_| ())?;
        let message_type = parts[1].parse::<MessageType>().map_err(|_| ())?;
        let mut message_params = Vec::new();
        for i in (2..parts.len()).step_by(2) {
            if i + 1 < parts.len() {
                message_params.push(MessageParam {
                    key: parts[i].to_string(),
                    value: parts[i + 1].to_string(),
                });
            }
        }
        Ok(GameMessage {
            player_id,
            message_type,
            message_params,
        })
    }
}
