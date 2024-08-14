mod conway;
use conway::MainState;
use ggez::event;
use ggez::GameResult;

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Conway's Game of Life", "user");
    let (ctx, event_loop) = cb.build()?;

    let xmax = 50;
    let ymax = 50;
    let probability: f64 = 0.4;
    let scale: f32 = 8.0;

    let state = MainState::new(xmax, ymax, probability, scale)?;
    event::run(ctx, event_loop, state)
}
