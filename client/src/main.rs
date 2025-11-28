use client::{SmartSocketClient, Command};
use std::io::{self, Write};

fn main() {
    let mut client = SmartSocketClient::new("127.0.0.1:7890".into())
        .expect("can't connect to server");

    println!("Подключено к умной розетке!");
    println!("Команды:");
    println!("  on      – включить");
    println!("  off     – выключить");
    println!("  status  – проверить состояние");
    println!("  power   – получить мощность");
    println!("  exit    – выйти");
    println!();

    loop {
        print!("> ");
        io::stdout().flush().unwrap(); // чтобы отображалось сразу

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let command = input.trim().to_lowercase();

        let cmd = match command.as_str() {
            "on" => Command::TurnOn,
            "off" => Command::TurnOff,
            "status" => Command::IsEnabled,
            "power" => Command::GetPower,
            "exit" => {
                println!("Выход...");
                break;
            }
            _ => {
                println!("Неизвестная команда. Попробуй: on/off/status/power/exit");
                continue;
            }
        };

        match client.run_command(cmd) {
            Ok(response) => println!("Ответ: {}", response),
            Err(err) => println!("Ошибка: {}", err),
        }
    }
}
