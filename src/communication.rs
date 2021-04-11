pub mod picamctl {

    use std::{io::Error, net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket}};
    use crate::lobby::picamctl::Lobby;
    use crate::camera::picamctl::Camera;

    const PORT: u16 = 8085;

    const CMD_CONNECT: u16 = 0xACDC;
    const CMD_DISCONNECT: u16 = 0xDCAC;
    const CMD_DISCONNECT_ALL: u16 = 0xAAAA;

    pub fn start_listening (mut lobby: Lobby) -> Result<(), Error> {
        let bind_addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), PORT);
        let sock = UdpSocket::bind(bind_addr)?;
        let mut buffer: [u8; 2] = [0, 0];
        let mut camera = Camera::new();
        
        loop {
            let (bytes, received_address) = sock.recv_from(&mut buffer)?;
            let client_address: SocketAddrV4;
            if let SocketAddr::V4(v4) = received_address {
                client_address = v4;
            }
            else { continue; } //IpV6 is not supported

            println!("Received {} bytes from {}", bytes, client_address);
            if bytes != 2 { continue; }
            let cmd = u16::from_be_bytes(buffer);
            println!("Value: {}", cmd);
            match cmd {
                CMD_CONNECT => {
                    match lobby.try_connect_client(client_address) {
                        Ok(size) => {
                            if size > 0 && !camera.is_enabled() {
                                camera.set_enabled(true)?;
                            }
                        },
                        Err(er) => eprintln!("Failed to connect client {}: {}", client_address, er),
                    }
                },
                CMD_DISCONNECT => {
                    if lobby.try_disconnect_client(&client_address) == 0 && camera.is_enabled() {
                        camera.set_enabled(false)?;
                    }
                },
                CMD_DISCONNECT_ALL => {
                    lobby.disconnect_all_clients();
                    camera.set_enabled(false)?;
                },
                _ => continue,
            }
        }
    }
}
