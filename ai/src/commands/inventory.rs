//
// EPITECH PROJECT, 2024
// inventory
// File description:
// inventory command
//

#![allow(dead_code)]

use crate::tcp::{
    command_handle::{CommandError, CommandHandler, ResponseResult},
    TcpClient,
};

#[allow(unused_imports)]
use log::{debug, error, info, warn};

pub fn read_inventory_output(raw: String) -> Vec<(String, i32)> {
    let tmp = raw.trim_matches(|c| c == '[' || c == ']' || c == '\n');
    let inventory: Vec<(String, i32)> = tmp.split(',').fold(
        Vec::<(String, i32)>::new(),
        |mut acc: Vec<(String, i32)>, tile| {
            if let Some(idex) = tile.rfind(' ') {
                let (item, quantity) = tile.split_at(idex);
                acc.push((
                    item.trim().to_string(),
                    quantity.trim().parse::<i32>().unwrap_or(0),
                ));
            }
            acc
        },
    );

    debug!("Inventory: {:?}", inventory);
    inventory
}

pub async fn inventory(client: &mut TcpClient) -> Result<ResponseResult, CommandError> {
    let mut response = client.check_dead("Inventory\n").await?;
    loop {
        let res = client.handle_response(response).await?;
        if let ResponseResult::Inventory(_) = res {
            return Ok(res);
        }
        response = client
            .check_response()
            .await
            .ok_or(CommandError::NoResponseReceived)?;
    }
}

#[cfg(test)]
pub mod tests_inventory {
    use super::read_inventory_output;

    #[test]
    fn output_reading() {
        let res: Vec<(String, i32)> = read_inventory_output(
            "[food 9,linemate 0,deraumere 0,sibur 0,mendiane 0,phiras 0,thystame 0]\n".to_string(),
        );
        let cmp: Vec<(String, i32)> = vec![
            ("food".to_string(), 9),
            ("linemate".to_string(), 0),
            ("deraumere".to_string(), 0),
            ("sibur".to_string(), 0),
            ("mendiane".to_string(), 0),
            ("phiras".to_string(), 0),
            ("thystame".to_string(), 0),
        ];
        assert_eq!(cmp, res);
    }

    #[test]
    fn output_reading_empty() {
        let res: Vec<(String, i32)> = read_inventory_output("[]\n".to_string());
        let cmp: Vec<(String, i32)> = vec![];
        assert_eq!(cmp, res);
    }
}
