use crate::client::{ClientId, FikaClient, Avatar};

use ggez::{
    graphics, Context, ContextBuilder, GameResult,
    event::{self, EventHandler},
};

use std::collections::HashMap;

pub struct Cone {
    pub pos: (f32, f32),
    pub dir: f32,
    pub angle: f32,
}

pub struct Board {
    pub size: (u32, u32),
    pub client: FikaClient,
    pub avatars: HashMap<ClientId, Avatar>,
}

impl Cone {
    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            [self.pos.0, self.pos.1, 10.0, 10.0].into(),
            [0.0, 1.0, 0.0, 1.0].into(), // Some arbitrary color.
        )?;

        graphics::draw(ctx, &rectangle, (ggez::mint::Point2{ x: 0.0, y: 0.0 },));

        Ok(())
    }
}

impl EventHandler for Board {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        //TODO: poll audio.
        //TODO: poll network updates.
        //TODO: update configuration.
        //TODO: draw configuration.
        //TODO: play audio.
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

        for (_, avatar) in self.avatars.iter() {
            avatar.cone.draw(ctx)?;
        }

        self.client.avatar.cone.draw(ctx)?;

        graphics::present(ctx)
    }
}
