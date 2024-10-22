//
// EPITECH PROJECT, 2024
// incantation
// File description:
// incantation command
//

#![allow(dead_code)]

use crate::tcp::{
    command_handle::{CommandError, CommandHandler, ResponseResult},
    TcpClient,
};

#[allow(unused_imports)]
use log::{debug, error, info, warn};

pub fn get_current_level(level_str: &str) -> Result<usize, CommandError> {
    level_str
        .split_once(": ")
        .and_then(|(_, l)| l.parse::<usize>().ok())
        .ok_or(CommandError::InvalidResponse)
}

pub async fn handle_incantation(client: &mut TcpClient) -> Result<ResponseResult, CommandError> {
    loop {
        let response = client
            .get_response()
            .await
            .ok_or(CommandError::NoResponseReceived)?;
        let res = client.handle_response(response).await?;
        if let ResponseResult::Incantation(_) = res {
            return Ok(res);
        }
        if res == ResponseResult::KO {
            return Ok(res);
        }
    }
}

pub async fn wait_for_incantation(client: &mut TcpClient) -> Result<ResponseResult, CommandError> {
    debug!("Waiting for incantation...");

    let response = client
        .check_response()
        .await
        .ok_or(CommandError::NoResponseReceived);
    if let Err(CommandError::NoResponseReceived) = response {
        let response = client.check_dead("Incantation\n").await?;
        return client.handle_response(response).await;
    }
    let res = client.handle_response(response.unwrap()).await?;
    if res == ResponseResult::Elevating {
        return handle_incantation(client).await;
    }
    Ok(res)
}

pub async fn incantation(client: &mut TcpClient) -> Result<ResponseResult, CommandError> {
    debug!("Incantation...");

    let mut response = client.check_dead("Incantation\n").await?;
    loop {
        let res = client.handle_response(response).await?;
        if res == ResponseResult::Elevating {
            return handle_incantation(client).await;
        }
        if res == ResponseResult::KO {
            return wait_for_incantation(client).await;
        }
        debug!("[{}] Incantation result: {:?}", client.id(), res);
        response = client
            .check_response()
            .await
            .ok_or(CommandError::NoResponseReceived)?;
    }
}
