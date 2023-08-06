mod conway;
use conway::MainState;
use ggez::event;
use ggez::GameResult;

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Conway's Game of Life", "Tom");
    let (ctx, event_loop) = cb.build()?;

    let xmax = 400;
    let ymax = 400;
    let probability = 0.7;

    let state = MainState::new(xmax, ymax, probability)?;
    event::run(ctx, event_loop, state)
}
