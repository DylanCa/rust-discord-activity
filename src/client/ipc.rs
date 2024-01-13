use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use log::debug;
use serde_json::{json, Value};
use std::env::var;
use std::error::Error;
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::path::PathBuf;

use crate::models::client::{commands::Commands, payload::OpCode, payload::Payload};
use crate::models::error::Error as ErrorMsg;
use crate::models::error::Error::DiscordNotFound;

/// Client used to communicate with Discord through IPC.
pub struct DiscordClient {
    /// ID of Discord Application, see https://discord.com/developers for more info
    pub id: String,
    socket: Option<UnixStream>,
}

impl DiscordClient {
    /// Used to instantiate a new Discord Client.
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            socket: None,
        }
    }

    /// Tries to enable a connection to the Discord Application.
    pub fn connect(&mut self) -> Result<(), ErrorMsg> {
        let path = self.fetch_process_pathbuf().join("discord-ipc-0");

        match UnixStream::connect(&path) {
            Ok(socket) => {
                self.socket = Some(socket);
                self.handshake().expect("Could not handshake.");
                Ok(())
            }
            Err(_) => Err(DiscordNotFound),
        }
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

    fn socket(&mut self) -> &mut UnixStream {
        self.socket.as_mut().unwrap()
    }

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
