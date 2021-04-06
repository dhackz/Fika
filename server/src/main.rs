use std::net::{UdpSocket, SocketAddr};

fn main() {
    let socket = UdpSocket::bind("localhost:8080").expect("Failed to connect");
    let mut server = FikaServer::new(socket);

    server.start().expect("reee");
}

struct FikaServer {
    socket: UdpSocket,
    connections: Vec<SocketAddr>,
}

impl FikaServer {
    pub fn new(socket: UdpSocket) -> FikaServer  {
        FikaServer { socket, connections: Vec::new() }
    }

    pub fn start(&mut self) -> std::io::Result<()> {
        loop {
            // Wait for new connection...
            let mut buf = [0; 100];
            let (amt, src) = self.socket.recv_from(&mut buf)?;
            if !self.connections.contains(&src) {
                self.connections.push(src);

                println!("New client connected! Got {} bytes from {:?}", amt, src);
                println!("Message: {}", std::str::from_utf8(&buf).expect("Failed to read message!"));

                let buf = &mut buf[..amt];
                buf.reverse();
                self.socket.send_to(buf, &src)?;
            }

        }
    }
}
