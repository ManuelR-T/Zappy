//
// EPITECH PROJECT, 2024
// ai.rs
// File description:
// zappy ai main functions
//

#![allow(dead_code)]

pub mod bot;
pub mod fetus;
pub mod knight;
pub mod npc;
pub mod queen;

use crate::tcp::{
    self,
    command_handle::{CommandError, ResponseResult},
    TcpClient,
};

use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::{self, Error, ErrorKind};
use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc,
};

use async_trait::async_trait;
use tokio::{sync::Mutex, task};

use log::{debug, error, info, warn};
use zappy_macros::Bean;

#[derive(Debug, Clone, Bean)]
pub struct AI {
    address: String,
    team: String,
    cli_id: usize,
    p_id: usize,
    client: Arc<Mutex<TcpClient>>,
    map: (i32, i32),
    level: usize,
    slots: i32,
}

#[async_trait]
pub trait AIHandler {
    fn init(info: AI) -> Self;
    async fn update(&mut self) -> Result<(), CommandError>;
    async fn fork_dupe(info: AI, set_id: Option<usize>) -> io::Result<()>;
}

#[async_trait]
pub trait Incantationers {
    async fn handle_eject(
        client: &mut TcpClient,
        res: Result<ResponseResult, CommandError>,
    ) -> Result<ResponseResult, CommandError>;

    async fn handle_elevating(
        client: &mut TcpClient,
        res: Result<ResponseResult, CommandError>,
    ) -> Result<ResponseResult, CommandError>;
}

#[async_trait]
pub trait Listeners {
    async fn handle_message(&mut self) -> Result<ResponseResult, CommandError>;
}

impl AI {
    fn new(
        team: String,
        address: String,
        (cli_id, p_id): (usize, usize),
        client: Arc<Mutex<TcpClient>>,
        map: (i32, i32),
        level: usize,
        slots: i32,
    ) -> Self {
        Self {
            team,
            address,
            cli_id,
            p_id,
            client,
            map,
            level,
            slots,
        }
    }
}

impl Display for AI {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "AI #{} = [team: {}, player ID: {}, map: ({}, {}), level: {}, leftover slots: {}]",
            self.cli_id, self.team, self.p_id, self.map.0, self.map.1, self.level, self.slots
        )
    }
}

async fn parse_response(
    response: &str,
    client: Arc<Mutex<TcpClient>>,
) -> Result<(i32, i32, i32), io::Error> {
    let mut cli: tokio::sync::MutexGuard<TcpClient> = client.lock().await;
    let client_number = response
        .parse::<i32>()
        .map_err(|_| Error::new(ErrorKind::InvalidData, "Invalid client number."))?;

    let response = cli
        .get_response()
        .await
        .ok_or_else(|| Error::new(ErrorKind::InvalidData, "Invalid response."))?;

    let mut words = response.split_whitespace();
    let x = words
        .next()
        .and_then(|word| word.parse::<i32>().ok())
        .ok_or_else(|| Error::new(ErrorKind::InvalidData, "Invalid x coordinate."))?;
    let y = words
        .next()
        .and_then(|word| word.parse::<i32>().ok())
        .ok_or_else(|| Error::new(ErrorKind::InvalidData, "Invalid y coordinate."))?;
    Ok((client_number, x, y))
}

async fn checkout_ai_info(
    client: Arc<Mutex<TcpClient>>,
    response: &str,
    team: String,
    address: String,
    (c_id, p_id): (usize, usize),
) -> io::Result<AI> {
    parse_response(response, client.clone())
        .await
        .map(|(client_number, x, y)| {
            info!("[{}] x{} unused slot(s)/ egg(s).", c_id, client_number);
            info!("[{}] Map size: {}x{}.", c_id, x, y);
            let ai = AI::new(
                team,
                address,
                (c_id, p_id),
                client.clone(),
                (x, y),
                1,
                client_number,
            );
            println!("[{}] New! >> {}", c_id, ai);
            debug!("[{}] AI is initialized.", ai.cli_id);
            ai
        })
        .map_err(|e: Error| {
            warn!("[{}] Failed to parse response: {}", c_id, e);
            e
        })
}

async fn init_ai(
    client: Arc<Mutex<TcpClient>>,
    response: &str,
    team: String,
    address: String,
    (c_id, p_id): (usize, usize),
) -> io::Result<AI> {
    info!("[{}] Initializing AI...", c_id);

    let ai = checkout_ai_info(client, response, team, address, (c_id, p_id)).await?;
    match ai.cli_id {
        0..=3 => {
            let mut queen = queen::Queen::init(ai.clone());
            if let Err(e) = queen.update().await {
                error!("[{}] Error: {}", queen.info().cli_id, e);
            }
        }
        _ => {
            let mut bot = bot::Bot::init(ai.clone());
            if let Err(e) = bot.update().await {
                error!("[{}] Error: {}", bot.info().cli_id, e);
            }
        }
    }
    Ok(ai)
}

async fn start_ai(
    client: Arc<Mutex<TcpClient>>,
    team: String,
    address: String,
    (c_id, p_id): (usize, usize),
    start: bool,
) -> io::Result<AI> {
    println!("[{}] Starting AI process...", c_id);
    {
        let client_lock = client.lock().await;
        client_lock.send_request(team.clone() + "\n").await?;
    }
    if let Some(response) = {
        let mut client_lock = client.lock().await;
        client_lock.get_response().await
    } {
        println!("[{}] server> {}", c_id, response);
        match response.trim_end() {
            "ko" => {
                debug!("[{}] Server doesn't handle any more connection.", c_id);
                Err(Error::new(
                    ErrorKind::ConnectionRefused,
                    "No room for player.",
                ))
            }
            _ => {
                info!("[{}] Connection to team successful.", c_id);
                let ai = match start {
                    true => init_ai(client.clone(), &response, team, address, (c_id, p_id)).await?,
                    false => {
                        checkout_ai_info(client.clone(), &response, team, address, (c_id, p_id))
                            .await?
                    }
                };
                Ok(ai)
            }
        }
    } else {
        debug!("[{}] Host not reachable.", c_id);
        Err(Error::new(
            ErrorKind::ConnectionRefused,
            "Couldn't reach host.",
        ))
    }
}

pub async fn launch(address: String, team: String) -> io::Result<()> {
    let mut handles = vec![];
    let team = Arc::new(team);
    let address = Arc::new(address);
    let connection_id = Arc::new(AtomicUsize::new(0));
    let stop_flag = Arc::new(AtomicBool::new(false));

    loop {
        if stop_flag.load(Ordering::SeqCst) {
            println!(
                "[AT {:?}] Stop flag is set, breaking the loop.",
                connection_id
            );
            break;
        }

        let team = Arc::clone(&team);
        let address = Arc::clone(&address);
        let connection_id = Arc::clone(&connection_id);
        let stop_flag = Arc::clone(&stop_flag);

        let curr_id = connection_id.load(Ordering::SeqCst);
        println!("[{}] Attempting connection...", curr_id);

        match tcp::handle_tcp(address.to_string(), team.to_string(), curr_id).await {
            Ok(client) => {
                let client = Arc::new(Mutex::new(client));
                let id = connection_id.fetch_add(1, Ordering::SeqCst);

                let handle = task::spawn(async move {
                    let result = start_ai(
                        client.clone(),
                        team.to_string(),
                        address.to_string(),
                        (id, 0),
                        true,
                    )
                    .await;

                    match result {
                        Ok(_) => {
                            println!("[{}] Connection handled successfully", id);
                            Ok(())
                        }
                        Err(e) => {
                            println!("[{}] Connection failed: {}", id, e);
                            stop_flag.store(true, Ordering::SeqCst);
                            Err(e)
                        }
                    }
                });
                handles.push(handle);
            }
            Err(e) => {
                println!("[{}] Failed to handle TCP: {}", curr_id, e);
                break;
            }
        }
    }

    if handles.is_empty() {
        warn!("Connection refused, handles is empty.");
        return Err(Error::new(
            ErrorKind::ConnectionRefused,
            "Couldn't reach host.",
        ));
    }

    for (id, handle) in handles.into_iter().enumerate() {
        if let Err(e) = handle.await {
            println!("[{}] Task failed: {:?}", id, e);
        }
    }

    Ok(())
}
