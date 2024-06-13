//
// EPITECH PROJECT, 2024
// Zappy
// File description:
// queen
//

use crate::{
    ai::{AIHandler, Incantationers, AI},
    commands::{self, turn::DirectionTurn},
    elevation::{Config, Inventory},
    move_towards_broadcast::{backtrack_eject, turn_towards_broadcast},
    tcp::{
        command_handle::{self, CommandError, CommandHandler, DirectionMessage, ResponseResult},
        TcpClient,
    },
};

use core::fmt;
use std::fmt::{Display, Formatter};

use async_trait::async_trait;

use log::info;
use zappy_macros::Bean;

use super::Listeners;

const QUEENS_IDS: [usize; 4] = [2, 1, 4, 3];

#[derive(Debug, Clone, Default)]
struct LookInfo {
    nb_player: usize,
    inv: Inventory,
}

#[derive(Debug, Clone, Bean)]
pub struct Queen {
    pub info: AI,
    inv: Inventory,
    look: LookInfo,
    requirement: Config,
    can_move: bool,
}

#[async_trait]
impl Incantationers for Queen {
    async fn handle_eject(
        client: &mut TcpClient,
        res: Result<ResponseResult, CommandError>,
    ) -> Result<ResponseResult, CommandError> {
        if let Ok(ResponseResult::Eject(ref dir)) = res {
            if backtrack_eject(client, dir.clone()).await {
                let response = client.check_response().await?;
                client.handle_response(response).await?;
            }
        }
        res
    }
}

#[async_trait]
impl Listeners for Queen {
    async fn handle_message(&mut self) -> Result<ResponseResult, CommandError> {
        let mut can_move = false;
        self.analyse_messages(&mut can_move).await?;
        if can_move {
            self.set_can_move(true);
        }
        Ok(ResponseResult::OK)
    }
}

impl Queen {
    /// Creates a new [`Queen`].
    fn new(info: AI) -> Self {
        Self {
            info,
            inv: Default::default(),
            look: Default::default(),
            requirement: zappy_json::create_from_file::<Config>("config.json").unwrap(),
            can_move: false,
        }
    }

    /**
        Move [`queen`] at level 4,
        we assume that all the queens have the same direction
    */
    async fn move_queen_first_step(&mut self) -> Result<(), CommandError> {
        if self.info.p_id == 2 | 4 {
            return Ok(());
        }
        // Check au niveau de broadcast correctement, check que la queen en face peut.
        let mut cli = self.info.client.lock().await;
        let _ = commands::move_up::move_up(&mut cli).await?;
        let val =
            commands::broadcast::broadcast(&mut cli, format!("{} mr", self.info.p_id).as_str())
                .await?;
        let _ = Queen::handle_eject(&mut cli, Ok(val)).await?;
        Ok(())
    }

    /**
        Move [`queen`] at level 6,
        we will move queen's direction and then reunite them in a single tile
    */
    async fn move_queen_second_step(&mut self) -> Result<(), CommandError> {
        match self.info.p_id {
            1 | 2 => {
                // Check que les queens en face peut
                let mut cli = self.info.client.lock().await;
                let _ = commands::move_up::move_up(&mut cli).await?;
                let _ = commands::broadcast::broadcast(
                    &mut cli,
                    format!("{} mr", self.info.p_id).as_str(),
                )
                .await?;
            }
            3 | 4 => {
                // Check que les queens en face peut
                let mut cli = self.info.client.lock().await;
                let _ = commands::move_up::move_up(&mut cli).await?;
                commands::turn::turn(&mut cli, DirectionTurn::Left).await?;
                commands::turn::turn(&mut cli, DirectionTurn::Left).await?;
                commands::broadcast::broadcast(&mut cli, format!("{} ml", self.info.p_id).as_str())
                    .await?;
            }
            _ => (),
        }
        Ok(())
    }

    async fn check_move_elevation(&mut self) -> Result<(), command_handle::CommandError> {
        match self.info.level {
            // Move it somewhere else because we have to check for each queen.
            4 => self.move_queen_first_step().await,
            6 => self.move_queen_second_step().await,
            _ => Ok(()),
        }
    }

    async fn incantate(&mut self) -> Result<(), command_handle::CommandError> {
        {
            let mut cli = self.info.client.lock().await;
            commands::broadcast::broadcast(&mut cli, "Incantation !").await?;
            let val = commands::incantation::incantation(&mut cli).await;
            Queen::handle_eject(&mut cli, val).await?
        };
        Ok(())
    }

    async fn check_enough_food(&mut self, min: usize) -> Result<(), command_handle::CommandError> {
        if *self.inv.food() >= min || *self.look.inv.food() == 0 {
            return Ok(());
        }
        let mut cli = self.info.client.lock().await;
        if let Ok(ResponseResult::OK) = commands::take_object::take_object(&mut cli, "food").await {
            self.inv.set_food(self.inv.food() + 1);
        }
        Ok(())
    }

    async fn fork_servant(&mut self) -> Result<(), command_handle::CommandError> {
        let mut cli = self.info.client.lock().await;
        commands::fork::fork(&mut cli).await?;
        // PlaceHolder for Knight creation
        commands::broadcast::broadcast(&mut cli, self.info().p_id().to_string().as_str()).await?;
        info!("I as the queen ({}), bestow my life uppon you\n", 0);

        commands::fork::fork(&mut cli).await?;
        // PlaceHolder for Bot creation
        commands::broadcast::broadcast(&mut cli, self.info().p_id().to_string().as_str()).await?;

        commands::fork::fork(&mut cli).await?;
        // PlaceHolder for Bot creation
        commands::broadcast::broadcast(&mut cli, self.info().p_id().to_string().as_str()).await?;
        info!("Miserable peasants... Serve me.\n");

        Ok(())
    }

    fn check_requirement(&self) -> bool {
        let idx = self.info.level - 1;
        let require = &self.requirement.elevation[idx];
        let r_inv = require.inv();
        let look = &self.look;

        look.nb_player >= *require.nb_players()
            && look.inv.food() >= r_inv.food()
            && look.inv.linemate() >= r_inv.linemate()
            && look.inv.deraumere() >= r_inv.deraumere()
            && look.inv.sibur() >= r_inv.sibur()
            && look.inv.mendiane() >= r_inv.mendiane()
            && look.inv.phiras() >= r_inv.phiras()
            && look.inv.thystame() >= r_inv.thystame()
    }

    fn convert_to_look_info(&mut self, vec: Vec<String>) {
        self.look.inv.clear();

        let inv: &mut Inventory = &mut self.look.inv;

        for elem in vec.iter() {
            match elem.as_str() {
                "player" => self.look.nb_player += 1,
                "food" => inv.set_food(inv.food() + 1),
                "linemate" => inv.set_linemate(inv.linemate() + 1),
                "deraumere" => inv.set_deraumere(inv.deraumere() + 1),
                "sibur" => inv.set_sibur(inv.sibur() + 1),
                "mendiane" => inv.set_mendiane(inv.mendiane() + 1),
                "phiras" => inv.set_phiras(inv.phiras() + 1),
                "thystame" => inv.set_thystame(inv.thystame() + 1),
                _ => (),
            }
        }
    }

    /// Transform Inventory info to exploit them later.
    fn convert_to_inv(&mut self, vec: Vec<(String, i32)>) {
        for elem in vec.iter() {
            match elem.0.as_str() {
                "food" => self.inv.set_food(elem.1 as usize),
                "linemate" => self.inv.set_linemate(elem.1 as usize),
                "deraumere" => self.inv.set_deraumere(elem.1 as usize),
                "sibur" => self.inv.set_sibur(elem.1 as usize),
                "mendiane" => self.inv.set_mendiane(elem.1 as usize),
                "phiras" => self.inv.set_phiras(elem.1 as usize),
                "thystame" => self.inv.set_thystame(elem.1 as usize),
                _ => (),
            }
        }
    }

    async fn handle_message_content(
        &self,
        client: &mut TcpClient,
        id: usize,
        dir: DirectionMessage,
        msg: &str,
        can_move: &mut bool,
    ) -> Result<ResponseResult, CommandError> {
        if msg.starts_with("lvl ") {
            if let Ok(lvl) = msg.split_at(3).1.parse::<i32>() {
                if (lvl == 4 && id == QUEENS_IDS[self.info().p_id - 1])
                    || (lvl == 6
                        && ((id == 1 | 2 && self.info().cli_id == 3 | 4)
                            || (id == 3 | 4 && self.info().cli_id == 1 | 2)))
                {
                    *can_move = true;
                }
            }
        } else if msg == "Done" {
            turn_towards_broadcast(client, dir).await?;
        }
        Ok(ResponseResult::OK)
    }

    async fn analyse_messages(
        &mut self,
        can_move: &mut bool,
    ) -> Result<ResponseResult, CommandError> {
        let mut client = self.info().client().lock().await;
        while let Some((dir, msg)) = client.pop_message() {
            info!(
                "Knight [Queen {}]: handling message: {}",
                self.info().cli_id,
                msg
            );
            if !msg.contains(' ') || msg.len() < 2 {
                continue;
            }
            if let Some(idex) = msg.trim_end_matches('\n').find(' ') {
                let content = msg.split_at(idex);
                if let Ok(id) = content.0.parse::<usize>() {
                    self.handle_message_content(&mut client, id, dir, content.1, can_move)
                        .await?;
                }
            }
        }
        Ok(ResponseResult::OK)
    }
}

#[async_trait]
impl AIHandler for Queen {
    fn init(info: AI) -> Self {
        // After this the queen must turn to the direction of message "Done"
        Self::new(info)
    }

    async fn update(&mut self) -> Result<(), command_handle::CommandError> {
        self.fork_servant().await?;
        loop {
            let val = {
                let mut cli = self.info.client.lock().await;
                let res = commands::look_around::look_around(&mut cli).await;
                Queen::handle_eject(&mut cli, res).await
            };
            if let Ok(ResponseResult::Tiles(vec)) = val {
                self.convert_to_look_info(vec[0].clone());
            }

            let val = {
                let mut cli = self.info.client.lock().await;
                let res = commands::inventory::inventory(&mut cli).await;
                Queen::handle_eject(&mut cli, res).await
            };
            if let Ok(ResponseResult::Inventory(vec)) = val {
                self.convert_to_inv(vec);
            }

            self.check_enough_food(3).await?;

            if self.check_requirement() {
                info!("Ai {} is incanting", self.info.cli_id);
                self.incantate().await?;
            }
        }
    }
}

impl Display for Queen {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Queen => {}", self.info)
    }
}
