//
// EPITECH PROJECT, 2024
// Zappy
// File description:
// main
//

use std::process;

pub mod commands;
pub mod flags;
mod json;
pub mod tcp;

const ERROR_CODE: i32 = 84;
const SUCCESS_CODE: i32 = 0;

#[tokio::main]
async fn main() {
    if open_file(String::from("config.json")).is_err() {
        println!("Error: Cannot open file {}", "config.json");
    }
    match flags::check_flags() {
        Ok(res) => {
            let address: String =
                format!("{}:{}", res.clone().get_machine(), res.clone().get_port());
            match tcp::handle_tcp(address, res.get_name()).await {
                Ok(_) => process::exit(SUCCESS_CODE),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    process::exit(ERROR_CODE);
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            flags::usage();
            process::exit(ERROR_CODE)
        }
    }
}
