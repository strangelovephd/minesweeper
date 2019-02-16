#![deny(missing_docs)]
//! A sudoku game.

use piston::{
    event_loop::{Events, EventLoop, EventSettings},
    input::RenderEvent,
    window::WindowSettings,
    };
use opengl_graphics::{OpenGL, Filter, GlGraphics, GlyphCache, TextureSettings};
use glutin_window::GlutinWindow;

mod minesweeper;
mod minesweeper_controller;
mod minesweeper_view;

use minesweeper::{GameMode, Minesweeper};
use minesweeper_controller::MinesweeperController;
use minesweeper_view::{MinesweeperView, MinesweeperViewSettings};

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Sudoku", [512; 2])
        .opengl(opengl)
        .exit_on_esc(true);
    
    let mut window: GlutinWindow = settings.build()
        .expect("Could not create window...");
    
    let mut events = Events::new(EventSettings::new())
        .lazy(true);
    let mut gl = GlGraphics::new(opengl);


    let minesweeper = Minesweeper::new(GameMode::Beginner);
    let mut minesweeper_controller = MinesweeperController::new(minesweeper);
    let minesweeper_view_settings = MinesweeperViewSettings::new();
    let minesweeper_view = MinesweeperView::new(minesweeper_view_settings);

    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let ref mut glyphs = GlyphCache::new("assets/FiraSans-Regular.ttf", (), texture_settings)
        .expect("Could not load font...");

    while let Some(e) = events.next(&mut window) {
        
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::clear;

                clear([1.0, 1.0, 1.0, 0.2], g);

                minesweeper_view.draw(&minesweeper_controller, glyphs, &c, g);
            });
        }
    }
}
