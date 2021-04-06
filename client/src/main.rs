use ggez::{
    graphics, Context, ContextBuilder, GameResult,
    event::{self, EventHandler},
};

use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};

fn main() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("fika", "David Ström")
        .build()
        .expect("Failed to create context!");

    let socket = UdpSocket::bind("localhost:8081").expect("Failed to connect");
    socket.connect("localhost:8080").expect("Couldn't connect to server!");
    let hello = "Hello, world!";
    socket.send(hello.as_bytes()).expect("Couldn't send message!");
    //let mut client = FikaClient::new(String::from("David"), socket);
    //event::run(&mut ctx, &mut event_loop, &mut client).expect("Failed to start fika!");
}

struct FikaClient {
    id: String,
    cone: Cone,
    socket: UdpSocket,
}

struct Cone {
    pos: (i32, i32),
    dir: f32,
    angle: f32,
}

impl FikaClient {
    pub fn new(id: String, socket: UdpSocket) -> FikaClient {
        let cone = Cone {
            pos: (0, 0),
            dir: 0.0,
            angle: 10.0,
        };
        FikaClient{ id, cone, socket }
    }
}

impl EventHandler for FikaClient {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        //TODO: poll audio.
        //TODO: poll network updates.
        //TODO: update configuration.
        //TODO: play audio.
        //TODO: draw configuration.
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        graphics::present(ctx)
    }
}
