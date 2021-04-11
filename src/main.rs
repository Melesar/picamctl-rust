use crate::lobby::picamctl::Lobby;
use crate::communication::picamctl::start_listening;

mod communication;
mod lobby;
mod camera;

fn main() {
    let lobby = Lobby::new();
    if let Err(er) = start_listening(lobby) {
        eprintln!("Error running a program: {}", er);
    }
}
