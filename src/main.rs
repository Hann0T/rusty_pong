use ggez;
use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};

struct MainState {}

impl MainState {
    pub fn new() -> Self {
        MainState {}
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Pong", "Hann0t");
    let (ctx, event_loop) = cb.build()?;

    graphics::set_window_title(&ctx, "PONG");

    let state = MainState::new();

    event::run(ctx, event_loop, state);

    Ok(())
}
