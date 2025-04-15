use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket},
    thread,
    time::Duration,
};

fn main() -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind("127.0.0.1:34255")?;
        let addr = SocketAddr::from(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 34254));
    
        for _ in 0..10 {
            let buf = b"bim bim bim";
            socket.send_to(buf, &addr)?;
            thread::sleep(Duration::from_secs(1));
        }

        let buf = b"exit";
        socket.send_to(buf, &addr)?;
    }

    Ok(())
}
