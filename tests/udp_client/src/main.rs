use std::{
    io::Error, net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket}, thread, time::Duration
};

fn main() -> std::io::Result<()> {
    {
        thread::spawn(move || -> Result<(), Error> {
            for _ in 0..10 {
                let socket = UdpSocket::bind("127.0.0.1:34255")?;
                let addr = SocketAddr::from(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 34254));
                let buf = b"0|0|name|alan";
                socket.send_to(buf, &addr)?;
                thread::sleep(Duration::from_secs(1));
                let mut buf = [0; 1024];
                let (amt, _) =  socket.recv_from(&mut buf)?;
                let msg = String::from_utf8_lossy(&buf[..amt]);
                println!("{}", msg);
            }
            Ok(())
        });
        
        thread::spawn(move || -> Result<(), Error> {
            for _ in 0..10 {
                let socket = UdpSocket::bind("127.0.0.1:34256")?;
                let addr = SocketAddr::from(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 34254));
                let buf = b"0|0|name|nahanna";
                socket.send_to(buf, &addr)?;
                thread::sleep(Duration::from_secs(1));
                let mut buf = [0; 1024];
                let (amt, _) =  socket.recv_from(&mut buf)?;
                let msg = String::from_utf8_lossy(&buf[..amt]);
                println!("{}", msg);
            }
            Ok(())
        });

        thread::sleep(Duration::from_secs(15));
    }

    Ok(())
}
