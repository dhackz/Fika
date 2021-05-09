mod board;
mod client;

use crate::board::Board;
use client::{ClientId, FikaClient, Avatar};

use std::collections::HashMap;

use ggez::ContextBuilder;
use ggez::event;

fn main() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("Fika!", "David Ström")
        .build()
        .expect("Failed to create context!");

    let mut client = FikaClient::new(String::from("David"), String::from("localhost:8080"));

    let mut board = Board {
        size: (15, 15),
        avatars: HashMap::<ClientId, Avatar>::new(),
        client,
    };

    event::run(&mut ctx, &mut event_loop, &mut board).expect("Failed to start fika!");
}
