use std::{
    collections::HashMap,
    hash::Hash,
    net::{SocketAddr, SocketAddrV4, UdpSocket},
    sync::Arc,
    thread,
};

pub mod game_message;

use futures::{executor::block_on, lock::Mutex};

pub struct PlayerInfo {
    pub player_id: usize,
    pub player_name: String,
    pub player_ip: SocketAddr,
}

fn main() -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind("127.0.0.1:34254")?;

        let map: HashMap<usize, PlayerInfo> = HashMap::new();
        let map = Mutex::new(map);
        let map = Arc::new(map);
        loop {
            let mut buf = [0; 1024];
            let (amt, src) = socket.recv_from(&mut buf)?;

            thread::spawn(|| {
                
            });
            println!("Received {} bytes from {}", amt, src);
            socket.send_to(&buf[..amt], &src)?;
            let string = String::from_utf8_lossy(&buf[..amt]);
            if string.trim() == "exit" {
                println!("Exiting server...");
                break;
            }
        }
    }

    Ok(())
}

fn handle_message(
    socket: &UdpSocket,
    response_address: &SocketAddr,
    ip_map: &Arc<Mutex<HashMap<usize, PlayerInfo>>>,
    buf: &[u8],
    len: usize,
) {
    let string = String::from_utf8_lossy(buf);
    let player_id = 0; // TODO: Get player ID from message
    let player_name = string.to_string();

    let map = block_on(ip_map.lock());
    let parts = string.split("|").collect::<Vec<&str>>();
    
    let command = parts[0];

}
