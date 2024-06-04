//
// EPITECH PROJECT, 2024
// Zappy
// File description:
// main
//

use std::process;

use elevation::Elevation;
use env_logger::{Builder, Env};
use log::info;
use zappy_macros::{Bean, Deserialize};

pub mod ai;
pub mod commands;
pub mod elevation;
pub mod flags;
pub mod move_to_tile;
pub mod tcp;

const ERROR_CODE: i32 = 84;
const SUCCESS_CODE: i32 = 0;

#[allow(dead_code)]
#[derive(Debug, Deserialize, Default, Bean)]
struct Resources {
    name: String,
    density: f32,
}

#[allow(dead_code)]
#[rustfmt::skip]
#[derive(Debug, Deserialize, Default, Bean)]
struct Config {
    ressources: Vec::<Resources>,
    elevation: Vec::<Elevation>,
}

#[tokio::main]
async fn main() {
    let env = Env::new().filter("ZAPPY_LOG");
    Builder::from_env(env).init();

    let _ = zappy_json::create_from_file::<Config>("config.json").unwrap();

    match flags::check_flags() {
        Ok(res) => {
            info!("Arguments set:\n{}", res.clone());
            let address: String = format!(
                "{}:{}",
                res.clone().machine(),
                res.clone().port().unwrap_or_default()
            );
            match ai::launch(address, res.name().clone().unwrap_or_default()).await {
                Ok(_) => process::exit(SUCCESS_CODE),
                Err(e) => {
                    eprintln!("Error: {}.", e);
                    process::exit(ERROR_CODE);
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}.", e);
            flags::usage();
            process::exit(ERROR_CODE)
        }
    }
}
