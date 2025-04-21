use std::{net::SocketAddr, sync::Mutex};

static SERVER_IP: Mutex<[i32; 4]> = Mutex::new([127, 0, 0, 1]);

static SERVER_PORT: Mutex<u16> = Mutex::new(54123);

pub fn get_server_addr() -> SocketAddr {
    let ip = SERVER_IP.lock().unwrap();
    let port = SERVER_PORT.lock().unwrap();
    SocketAddr::from(([ip[0] as u8, ip[1] as u8, ip[2] as u8, ip[3] as u8], *port))
}