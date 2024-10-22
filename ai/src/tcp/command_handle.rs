//
// EPITECH PROJECT, 2024
// Zappy
// File description:
// commands
//

#![allow(dead_code)]

use crate::{
    commands::{
        incantation::get_current_level, inventory::read_inventory_output,
        look_around::read_look_output,
    },
    crypt::Crypt,
    tcp::TcpClient,
};

use std::fmt;
use std::fmt::{Display, Formatter};

use async_trait::async_trait;

#[allow(unused_imports)]
use log::{debug, error, info, warn};

#[derive(PartialEq, Debug)]
pub enum ResponseResult {
    OK,
    KO,
    Dead,
    Elevating,
    Value(usize),
    Text(String),
    Tiles(Vec<Vec<String>>),
    Inventory(Vec<(String, i32)>),
    Incantation(usize),
    Message((DirectionMessage, String)),
    Eject(DirectionEject),
    EjectUndone,
    Unknown,
}

#[derive(Debug, PartialEq, Clone)]
#[repr(u8)]
pub enum DirectionMessage {
    Center,
    North,
    NorthWest,
    West,
    SouthWest,
    South,
    SouthEast,
    East,
    NorthEast,
}

#[derive(Debug, PartialEq, Clone)]
#[repr(u8)]
pub enum DirectionEject {
    North = 1,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq)]
pub enum CommandError {
    RequestError,
    NoResponseReceived,
    InvalidResponse,
    DeadReceived,
    InvalidKey,
}

#[async_trait]
pub trait CommandHandler {
    async fn send_command(&mut self, command: &str) -> Result<String, CommandError>;
    async fn check_dead(&mut self, command: &str) -> Result<String, CommandError>;
    async fn handle_response(&mut self, response: String) -> Result<ResponseResult, CommandError>;
    async fn check_response(&mut self) -> Option<String>;
    async fn get_broadcast(&mut self) -> Result<ResponseResult, CommandError>;
}

#[async_trait]
impl CommandHandler for TcpClient {
    async fn send_command(&mut self, command: &str) -> Result<String, CommandError> {
        if self.send_request(command.to_string()).await.is_err() {
            return Err(CommandError::RequestError);
        }
        match self.get_response().await {
            Some(res) => Ok(res),
            None => Err(CommandError::NoResponseReceived),
        }
    }

    async fn check_response(&mut self) -> Option<String> {
        match self.get_response().await {
            Some(res) => {
                debug!("Response checked gives: ({})", res);
                Some(res)
            }
            None => {
                warn!("No response received.");
                None
            }
        }
    }

    async fn check_dead(&mut self, command: &str) -> Result<String, CommandError> {
        let response: String = self.send_command(command).await?;
        if response == "dead\n" {
            warn!("Dead received.");
            return Err(CommandError::DeadReceived);
        }
        Ok(response)
    }

    async fn get_broadcast(&mut self) -> Result<ResponseResult, CommandError> {
        let res = self
            .check_response()
            .await
            .ok_or(CommandError::NoResponseReceived)?;
        if res.starts_with("message ") {
            if let ResponseResult::Message(msg) =
                handle_message_response(res.clone(), self.crypt())?
            {
                debug!("[{}] Received message, to handle...", self.id);
                return Ok(ResponseResult::Message(msg));
            }
        }
        self.handle_response(res).await
    }

    async fn handle_response(&mut self, response: String) -> Result<ResponseResult, CommandError> {
        if response.starts_with("message ") {
            if let ResponseResult::Message(msg) = handle_message_response(response, self.crypt())? {
                self.push_message(msg);
                debug!("~[{}] Message pushed to queue.", self.id);
            }
            let response = self
                .check_response()
                .await
                .ok_or(CommandError::NoResponseReceived)?;
            return self.handle_response(response).await;
        }

        if response.starts_with("eject: ") {
            return handle_eject_response(response);
        }

        match response.trim_end() {
            "dead" => Err(CommandError::DeadReceived),
            "ok" => Ok(ResponseResult::OK),
            "ko" => Ok(ResponseResult::KO),
            "Elevation underway" => Ok(ResponseResult::Elevating),
            x if x.starts_with("Current level:") => {
                Ok(ResponseResult::Incantation(get_current_level(x)?))
            }
            x if x.starts_with("[food ") => {
                Ok(ResponseResult::Inventory(read_inventory_output(response)))
            }
            x if x.starts_with("[player") => Ok(ResponseResult::Tiles(read_look_output(response))),
            x if !x.is_empty() && x.as_bytes()[0].is_ascii_digit() => match x.parse::<usize>() {
                Ok(nb) => Ok(ResponseResult::Value(nb)),
                Err(_) => Ok(ResponseResult::KO),
            },
            x if x.starts_with("ko\n") => Ok(ResponseResult::KO),
            _ => {
                error!("Invalid Response: ({}).", response.trim_end());
                Ok(ResponseResult::KO)
            }
        }
    }
}

fn handle_message_response(
    response: String,
    crypt: &Crypt,
) -> Result<ResponseResult, CommandError> {
    let parts: Vec<&str> = response.split_whitespace().collect();

    if parts.len() >= 3 && parts[0] == "message" {
        match parts[1].trim_end_matches(',').parse::<usize>() {
            Ok(direction) => {
                if let Some(dir_enum) = DirectionMessage::from_usize(direction) {
                    let final_msg: String = parts[2..].join(" ");
                    debug!("Encrypted message received: {}", final_msg);
                    let decrypted_message = match crypt.decrypt(&final_msg) {
                        Some(data) => data,
                        None => return Ok(ResponseResult::OK), // corrupt any undecryptable message for other teams?
                    };
                    debug!(
                        "Message received from direction {} (aka {}): {}",
                        dir_enum, direction, decrypted_message
                    );
                    return Ok(ResponseResult::Message((dir_enum, decrypted_message)));
                }
                warn!("Failed to parse direction {}.", direction);
            }
            Err(_) => warn!("Failed to parse direction from message: {}", response),
        }
    }

    Ok(ResponseResult::Unknown)
}

fn handle_eject_response(response: String) -> Result<ResponseResult, CommandError> {
    let parts: Vec<&str> = response.split_whitespace().collect();

    if parts.len() == 2 && parts[0] == "eject:" {
        match parts[1].trim_start().parse::<usize>() {
            Ok(direction) => {
                if let Some(dir_enum) = DirectionEject::from_usize(direction) {
                    debug!(
                        "Receiving ejection from direction {} (aka {}).",
                        dir_enum, direction
                    );
                    return Ok(ResponseResult::Eject(dir_enum));
                }
                warn!("Failed to parse direction {}.", direction);
            }
            Err(_) => warn!(
                "Failed to parse direction from eject response: {}",
                response
            ),
        }
    }

    Ok(ResponseResult::Unknown)
}

pub trait DirectionHandler {
    fn to_usize(&self) -> usize;
    fn from_usize(value: usize) -> Option<Self>
    where
        Self: Sized;
}

impl DirectionHandler for DirectionMessage {
    fn to_usize(&self) -> usize {
        match self {
            DirectionMessage::Center => 0,
            DirectionMessage::North => 1,
            DirectionMessage::NorthWest => 2,
            DirectionMessage::West => 3,
            DirectionMessage::SouthWest => 4,
            DirectionMessage::South => 5,
            DirectionMessage::SouthEast => 6,
            DirectionMessage::East => 7,
            DirectionMessage::NorthEast => 8,
        }
    }

    fn from_usize(value: usize) -> Option<Self> {
        match value {
            0 => Some(DirectionMessage::Center),
            1 => Some(DirectionMessage::North),
            2 => Some(DirectionMessage::NorthWest),
            3 => Some(DirectionMessage::West),
            4 => Some(DirectionMessage::SouthWest),
            5 => Some(DirectionMessage::South),
            6 => Some(DirectionMessage::SouthEast),
            7 => Some(DirectionMessage::East),
            8 => Some(DirectionMessage::NorthEast),
            _ => None,
        }
    }
}

impl DirectionHandler for DirectionEject {
    fn to_usize(&self) -> usize {
        match self {
            DirectionEject::North => 1,
            DirectionEject::East => 2,
            DirectionEject::South => 3,
            DirectionEject::West => 4,
        }
    }

    fn from_usize(value: usize) -> Option<Self> {
        match value {
            1 => Some(DirectionEject::North),
            2 => Some(DirectionEject::East),
            3 => Some(DirectionEject::South),
            4 => Some(DirectionEject::West),
            _ => None,
        }
    }
}

impl Display for CommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CommandError::RequestError => write!(f, "Request error."),
            CommandError::NoResponseReceived => {
                write!(f, "No response has been successfully received.")
            }
            CommandError::InvalidResponse => write!(f, "Invalid response, unknown."),
            CommandError::DeadReceived => write!(f, "Dead has been received, end of program."),
            CommandError::InvalidKey => {
                write!(f, "Invalid key, message for broadcast can't be encrypted.")
            }
        }
    }
}

impl Display for DirectionMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DirectionMessage::Center => write!(f, "Center"),
            DirectionMessage::North => write!(f, "North"),
            DirectionMessage::NorthWest => write!(f, "NorthWest"),
            DirectionMessage::West => write!(f, "West"),
            DirectionMessage::SouthWest => write!(f, "SouthWest"),
            DirectionMessage::South => write!(f, "South"),
            DirectionMessage::SouthEast => write!(f, "SouthEast"),
            DirectionMessage::East => write!(f, "East"),
            DirectionMessage::NorthEast => write!(f, "NorthEast"),
        }
    }
}

impl Display for DirectionEject {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DirectionEject::North => write!(f, "North"),
            DirectionEject::West => write!(f, "West"),
            DirectionEject::South => write!(f, "South"),
            DirectionEject::East => write!(f, "East"),
        }
    }
}

impl Display for ResponseResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ResponseResult::OK => write!(f, "OK"),
            ResponseResult::KO => write!(f, "KO"),
            ResponseResult::Dead => write!(f, "Dead"),
            ResponseResult::Elevating => write!(f, "Elevating"),
            ResponseResult::Value(nb) => write!(f, "Value: {}", nb),
            ResponseResult::Text(text) => write!(f, "Text: {}", text),
            ResponseResult::Tiles(tiles) => {
                write!(f, "Tiles: [")?;
                for (i, tile) in tiles.iter().enumerate() {
                    write!(f, "[")?;
                    for (j, item) in tile.iter().enumerate() {
                        if j + 1 == tile.len() {
                            write!(f, "{}", item)?;
                        } else {
                            write!(f, "{}, ", item)?;
                        }
                    }
                    if i + 1 == tiles.len() {
                        write!(f, "]")?;
                    } else {
                        write!(f, "], ")?;
                    }
                }
                write!(f, "]")
            }
            ResponseResult::Inventory(inventory) => {
                write!(f, "Inventory: [")?;
                for (i, (item, nb)) in inventory.iter().enumerate() {
                    if i + 1 == inventory.len() {
                        write!(f, "({}: x{})", item, nb)?;
                    } else {
                        write!(f, "({}: x{}), ", item, nb)?;
                    }
                }
                write!(f, "]")
            }
            ResponseResult::Incantation(level) => write!(f, "Incantation Level: {}", level),
            ResponseResult::Message((dir, msg)) => write!(f, "Message: ({}, {})", dir, msg),
            ResponseResult::Eject(dir) => write!(f, "Eject: {}", dir),
            ResponseResult::EjectUndone => write!(f, "Eject Undoed"),
            ResponseResult::Unknown => write!(f, "Unknown type of response, seems invalid..."),
        }
    }
}
