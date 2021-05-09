use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};

use ggez::{
    graphics, Context, ContextBuilder, GameResult,
    event::{self, EventHandler},
};

use crate::board::Cone;

pub type ClientId = String;

/// Buddy represents a character on the board.
pub struct Avatar {
    pub id: ClientId,
    pub cone: Cone,
}

pub struct FikaClient {
    pub avatar: Avatar,
    socket: UdpSocket,
}

impl FikaClient {
    pub fn new(id: ClientId, server_socket_addr: String) -> FikaClient {
        let client_socket = UdpSocket::bind("localhost:8081").expect("Failed to connect");

        client_socket.connect(server_socket_addr).expect("Couldn't connect to server!");
        client_socket.send("Hello!".as_bytes()).expect("Couldn't send message!");

        let cone = Cone {
            pos: (0.0, 0.0),
            dir: 0.0,
            angle: 10.0,
        };

        FikaClient{ avatar: Avatar{ id, cone }, socket: client_socket }
    }
}
