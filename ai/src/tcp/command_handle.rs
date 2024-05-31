//
// EPITECH PROJECT, 2024
// Zappy
// File description:
// commands
//

#![allow(dead_code)]

// use crate::commands;
use crate::tcp::TcpClient;

use async_trait::async_trait;

use tokio::io::{self};

use std::fmt::Display;
use std::io::{Error, ErrorKind};

use log::info;

pub enum ResponseResult {
    OK,
    KO,
    Dead,
    Value(usize),
    Text(String),
    Tiles(Vec<String>),
    Inventory(Vec<(String, i32)>),
    Incantation(usize),
    Message((Direction, String)),
}

pub enum Direction {
    North,
    NorthWest,
    West,
    SouthWest,
    South,
    SouthEast,
    East,
    NorthEast,
}

pub enum CommandError {
    RequestError,
    NoResponseReceived,
    InvalidResponse,
    DeadReceived,
}

#[async_trait]
pub trait CommandHandler {
    async fn send_command(&mut self, command: &str) -> Result<String, CommandError>;
    async fn check_dead(&mut self, command: &str) -> Result<String, CommandError>;
    async fn handle_response(&mut self, response: String) -> Result<ResponseResult, CommandError>;
}

#[async_trait::async_trait]
impl CommandHandler for TcpClient {
    async fn send_command(&mut self, command: &str) -> Result<String, CommandError> {
        info!("Sending command: ({})...", command);
        if self.send_request(command.to_string()).await.is_err() {
            return Err(CommandError::RequestError);
        }
        match self.get_response().await {
            Some(res) => Ok(res),
            None => Err(CommandError::NoResponseReceived),
        }
    }

    async fn check_dead(&mut self, command: &str) -> Result<String, CommandError> {
        info!("Checking if request receives dead...");
        let response = self.send_command(command).await?;
        if response == "dead\n" {
            info!("Dead received.");
            return Err(CommandError::DeadReceived);
        }
        info!("Not dead received, response is forwarded.");
        Ok(response)
    }

    async fn handle_response(&mut self, response: String) -> Result<ResponseResult, CommandError> {
        info!("Handling response: ({})...", response);
        match response.as_str() {
            "ok\n" => Ok(ResponseResult::OK),
            "ko\n" => Ok(ResponseResult::KO),
            _ => Err(CommandError::InvalidResponse),
        }
    }
}

impl Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandError::RequestError => write!(f, "Request error."),
            CommandError::NoResponseReceived => write!(f, "No response received."),
            CommandError::InvalidResponse => write!(f, "Invalid response."),
            CommandError::DeadReceived => write!(f, "Dead received."),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::North => write!(f, "North"),
            Direction::NorthWest => write!(f, "NorthWest"),
            Direction::West => write!(f, "West"),
            Direction::SouthWest => write!(f, "SouthWest"),
            Direction::South => write!(f, "South"),
            Direction::SouthEast => write!(f, "SouthEast"),
            Direction::East => write!(f, "East"),
            Direction::NorthEast => write!(f, "NorthEast"),
        }
    }
}

impl Display for ResponseResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseResult::OK => write!(f, "OK"),
            ResponseResult::KO => write!(f, "KO"),
            ResponseResult::Dead => write!(f, "Dead"),
            ResponseResult::Value(nb) => write!(f, "Value: {}", nb),
            ResponseResult::Text(text) => write!(f, "Text: {}", text),
            ResponseResult::Tiles(tiles) => {
                write!(f, "Tiles: [")?;
                for tile in tiles {
                    write!(f, "{}, ", tile)?;
                }
                write!(f, "]")
            }
            ResponseResult::Inventory(inventory) => {
                write!(f, "Inventory: [")?;
                for (item, nb) in inventory {
                    write!(f, "({}: x{}), ", item, nb)?;
                }
                write!(f, "]")
            }
            ResponseResult::Incantation(level) => write!(f, "Incantation Level: {}", level),
            ResponseResult::Message((dir, msg)) => write!(f, "Message: ({}, {})", dir, msg),
        }
    }
}

#[warn(unused_mut)]
pub async fn start_ai(mut client: TcpClient, team: String) -> io::Result<()> {
    info!("Starting AI...");
    client.send_request(team + "\n").await?;
    if let Some(response) = client.get_response().await {
        match response.as_str() {
            "ko\n" => {
                print!("server> {}", response);
                return Err(Error::new(
                    ErrorKind::ConnectionRefused,
                    "No room for player.",
                ));
            }
            _ => print!("server> {}", response),
        }
    } else {
        return Err(Error::new(
            ErrorKind::ConnectionRefused,
            "Couldn't reach host.",
        ));
    }
    Ok(())
}
