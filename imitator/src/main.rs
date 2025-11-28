use std::io::{Read, Write};
use std::net::TcpListener;

pub enum Command {
    TurnOff,
    TurnOn,
    IsEnabled,
    GetPower,
    Unknown,
}

impl From<u8> for Command {
    fn from(val: u8) -> Self {
        match val {
            0 => Self::TurnOff,
            1 => Self::TurnOn,
            2 => Self::IsEnabled,
            3 => Self::GetPower,
            _ => Self::Unknown,
        }
    }
}

impl From<Command> for u8 {
    fn from(cmd: Command) -> Self {
        match cmd {
            Command::TurnOff => 0,
            Command::TurnOn => 1,
            Command::IsEnabled => 2,
            Command::GetPower => 3,
            Command::Unknown => 255,
        }
    }
}

pub enum Response {
    Ok,
    Enabled,
    Disabled,
    Power(f32),
    Unknown,
}

impl From<[u8; 5]> for Response {
    fn from(bytes: [u8; 5]) -> Self {
        match bytes {
            [0, ..] => Self::Ok,
            [1, ..] => Self::Enabled,
            [2, ..] => Self::Disabled,
            [3, ..] => {
                let mut buf = [0u8; 4];
                buf.copy_from_slice(&bytes[1..]);
                Self::Power(f32::from_be_bytes(buf))
            }
            _ => Self::Unknown,
        }
    }
}

impl From<Response> for [u8; 5] {
    fn from(resp: Response) -> Self {
        let mut buffer = [0u8; 5];
        match resp {
            Response::Ok => {}
            Response::Enabled => buffer[0] = 1,
            Response::Disabled => buffer[0] = 2,
            Response::Power(pwr) => {
                buffer[0] = 3;
                buffer[1..].copy_from_slice(&pwr.to_be_bytes())
            }
            Response::Unknown => buffer[0] = 255,
        };
        buffer
    }
}

fn main() {
    let mut args = std::env::args();
    args.next().unwrap();

    let server_address = args.next().unwrap_or_else(|| "127.0.0.1:7890".into());

    let listener = TcpListener::bind(server_address).expect("can't bind tcp listener");

    let mut smart_socket = SmartSocket::default();

    while let Some(connection) = listener.incoming().next() {
        let mut stream = match connection {
            Ok(conn) => conn,
            Err(err) => {
                println!("can't receive connection: {err}");
                continue;
            }
        };

        let peer = stream
            .peer_addr()
            .map(|a| a.to_string())
            .unwrap_or_else(|_| "unknown".into());
        println!("Peer '{peer}' connected");

        let mut in_buffer = [0u8];
        while stream.read_exact(&mut in_buffer).is_ok() {
            let response = smart_socket.process_command(in_buffer[0].into());
            let response_buf: [u8; 5] = response.into();
            if stream.write_all(&response_buf).is_err() {
                break;
            };
        };

        println!("Connection with {peer} lost. Waiting for new connections...");
    }
}

#[derive(Default)]
struct SmartSocket {
    enabled: bool,
}

impl SmartSocket {
    fn process_command(&mut self, cmd: Command) -> Response {
        match cmd {
            Command::TurnOn => {
                self.enabled = true;
                Response::Ok
            }
            Command::TurnOff => {
                self.enabled = false;
                Response::Ok
            }
            Command::IsEnabled => {
                if self.enabled {
                    Response::Enabled
                } else {
                    Response::Disabled
                }
            }
            Command::GetPower => {
                if self.enabled {
                    Response::Power(220.5)
                } else {
                    Response::Power(0.0)
                }
            }
            Command::Unknown => {
                println!("Unknown command received");
                Response::Unknown
            }
        }
    }
}
