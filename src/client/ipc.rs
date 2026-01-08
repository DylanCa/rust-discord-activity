use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use log::debug;
use serde_json::{json, Value};
use std::error::Error;
use std::io::{Read, Write};

#[cfg(unix)]
use std::env::var;

#[cfg(unix)]
use std::os::unix::net::UnixStream;
#[cfg(windows)]
use std::fs::OpenOptions;
#[cfg(windows)]
use std::os::windows::fs::OpenOptionsExt;

use std::path::PathBuf;

use crate::models::client::{commands::Commands, payload::OpCode, payload::Payload};
use crate::models::error::Error as ErrorMsg;
use crate::models::error::Error::DiscordNotFound;

/// Client used to communicate with Discord through IPC.
pub struct DiscordClient {
    /// ID of Discord Application, see <https://discord.com/developers> for more info
    pub id: String,

    /// Boolean stating if Client is connected to Discord App.
    pub is_connected: bool,

    /// Socket of Client Connection (Unix socket on Unix, Named pipe on Windows).
    #[cfg(unix)]
    socket: Option<UnixStream>,
    #[cfg(windows)]
    socket: Option<std::fs::File>,
}

impl DiscordClient {
    /// Used to instantiate a new Discord Client.
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            is_connected: false,
            socket: None,
        }
    }

    /// Tries to enable a connection to the Discord Application.
    #[cfg(unix)]
    pub fn connect(&mut self) -> Result<(), ErrorMsg> {
        let path = self.fetch_process_pathbuf().join("discord-ipc-0");

        match UnixStream::connect(&path) {
            Ok(socket) => {
                self.socket = Some(socket);
                self.handshake().expect("Could not handshake.");
                self.is_connected = true;
                Ok(())
            }
            Err(_) => {
                self.is_connected = false;
                Err(DiscordNotFound)
            }
        }
    }

    /// Tries to enable a connection to the Discord Application (Windows version using named pipes).
    #[cfg(windows)]
    pub fn connect(&mut self) -> Result<(), ErrorMsg> {
        // Try discord-ipc-0 through discord-ipc-9
        for i in 0..10 {
            let pipe_name = format!(r"\\.\pipe\discord-ipc-{}", i);
            
            match OpenOptions::new()
                .read(true)
                .write(true)
                .custom_flags(0x40000000) // FILE_FLAG_OVERLAPPED
                .open(&pipe_name)
            {
                Ok(socket) => {
                    self.socket = Some(socket);
                    match self.handshake() {
                        Ok(_) => {
                            self.is_connected = true;
                            return Ok(());
                        }
                        Err(e) => {
                            debug!("Handshake failed for {}: {:?}", pipe_name, e);
                            self.socket = None;
                            continue;
                        }
                    }
                }
                Err(e) => {
                    debug!("Failed to open {}: {:?}", pipe_name, e);
                    continue;
                }
            }
        }
        
        self.is_connected = false;
        Err(DiscordNotFound)
    }

    pub fn send_payload(&mut self, payload: Payload) -> Result<(u32, Value), Box<dyn Error>> {
        let payload = json!({
            "cmd": Commands::SetActivity.as_string(),
            "args": {
                "pid": std::process::id(),
                payload.event_name: payload.event_data,
            },
            "nonce": uuid::Uuid::new_v4().to_string(),
        });

        Ok(self.send(payload, OpCode::MESSAGE as u8)?)
    }

    #[cfg(unix)]
    fn socket(&mut self) -> &mut UnixStream {
        self.socket.as_mut().unwrap()
    }

    #[cfg(windows)]
    fn socket(&mut self) -> &mut std::fs::File {
        self.socket.as_mut().unwrap()
    }

    #[cfg(unix)]
    fn fetch_process_pathbuf(&mut self) -> PathBuf {
        let mut path = String::new();

        for key in ["XDG_RUNTIME_DIR", "TMPDIR", "TMP"] {
            match var(key) {
                Ok(val) => {
                    path = val;
                    break;
                }
                _ => continue,
            }
        }

        PathBuf::from(path)
    }

    #[cfg(windows)]
    fn fetch_process_pathbuf(&mut self) -> PathBuf {
        // Not used on Windows, but kept for API compatibility
        PathBuf::new()
    }

    fn handshake(&mut self) -> Result<(u32, Value), Box<dyn Error>> {
        let payload = json!({ "v": 1, "client_id": self.id});

        Ok(self.send(payload, OpCode::HANDSHAKE as u8)?)
    }

    fn send(&mut self, payload: Value, opcode: u8) -> Result<(u32, Value), Box<dyn Error>> {
        let payload = payload.to_string();
        let mut data: Vec<u8> = Vec::new();

        data.write_u32::<LittleEndian>(opcode as u32)?;
        data.write_u32::<LittleEndian>(payload.len() as u32)?;
        data.write_all(payload.as_bytes())?;

        self.socket().write_all(&data)?;
        Ok(self.recv()?)
    }

    fn recv(&mut self) -> Result<(u32, Value), Box<dyn Error>> {
        let mut buf = [0; 2048];

        let byte_count = self.socket().read(&mut buf)?;
        let (op, payload) = self.extract_payload(&buf[..byte_count])?;
        let json_data = serde_json::from_str::<Value>(&payload)?;

        debug!("{:?}", json_data);

        Ok((op, json_data))
    }

    fn extract_payload(&mut self, mut data: &[u8]) -> Result<(u32, String), Box<dyn Error>> {
        let opcode = data.read_u32::<LittleEndian>()?;
        let payload_len = data.read_u32::<LittleEndian>()? as usize;
        let mut payload = String::with_capacity(payload_len);
        data.read_to_string(&mut payload)?;

        Ok((opcode, payload))
    }
}
