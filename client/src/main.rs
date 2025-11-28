use std::error::Error;
use std::env;

use client::{Command, SmartSocketClient}; // имя пакета = name из Cargo.toml

fn main() -> Result<(), Box<dyn Error>> {
    // берём адрес сервера из аргументов или по умолчанию 127.0.0.1:7890
    let server_address = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:7890".to_string());

    println!("Connecting to {server_address}...");

    let mut client = SmartSocketClient::new(server_address)?;
    println!("Connected.");

    // 1. Включаем розетку
    let resp = client.run_command(Command::TurnOn)?;
    println!("TurnOn -> {resp}");

    // 2. Проверяем, включена ли
    let resp = client.run_command(Command::IsEnabled)?;
    println!("IsEnabled -> {resp}");

    // 3. Смотрим мощность
    let resp = client.run_command(Command::GetPower)?;
    println!("GetPower -> {resp}");

    // 4. Выключаем
    let resp = client.run_command(Command::TurnOff)?;
    println!("TurnOff -> {resp}");

    Ok(())
}
