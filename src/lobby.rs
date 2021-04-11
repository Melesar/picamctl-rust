
pub mod picamctl {

    use std::net::{SocketAddrV4, UdpSocket};
    use std::collections::HashSet;
    use std::io::Error;

    const RES_SUCCESS: [u8; 1] = [0];
    //Used to exists in the C version of the program, here it's not needed
    //const RES_FULL: [u8; 1] = [1];
    const RES_CONNECTED_ALREADY: [u8; 1] = [2];

    pub struct Lobby {
        clients: HashSet<SocketAddrV4>,
        socket: UdpSocket,
    }

    impl Lobby {
        pub fn new () -> Lobby {
            Lobby { 
                clients: HashSet::new(),
                socket: UdpSocket::bind("127.0.0.1:44322").unwrap(),
            }
        }

        pub fn try_connect_client(&mut self, addr: SocketAddrV4) -> Result<usize, Error> {
            if self.clients.contains(&addr) {
                self.socket.send_to(&RES_CONNECTED_ALREADY[..], &addr)?;
                return Ok(self.clients.len());
            }

            self.socket.send_to(&RES_SUCCESS[..], &addr)?;
            self.clients.insert(addr);
            self.print_clients(&format!("Connected: {}", addr));
            Ok(self.clients.len())
        }

        pub fn try_disconnect_client(&mut self, addr: &SocketAddrV4) -> usize {
            if self.clients.remove(addr) {
                self.print_clients(&format!("Disconnected {}", addr));
            }
            self.clients.len()
        }

        pub fn disconnect_all_clients(&mut self) {
            self.clients.clear();
            self.print_clients("Disconnected all clients");
        }

        fn print_clients(&self, message: &str) {
            println!("{}", message);
            println!("Clients:");
            for address in &self.clients {
                println!("{}", address);
            }
        }
    }
}
