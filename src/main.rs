extern crate ggez;
extern crate mint;

use ggez::audio;
use ggez::audio::SoundSource;
use ggez::conf;
use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::graphics;
use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};

use std::env;
use std::path;

mod assets;
use assets::Assets;
mod controls;
use controls::InputState;
mod actors;
use actors::Actor;


struct MainState {
    player: Actor,
    fruits: Vec<Actor>,
    level: i32,
    score: i32,
    assets: Assets,
    screen_width: f32,
    screen_height: f32,
    input: InputState,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        println!("Game resource path: {:?}", ctx.filesystem);

        print_instructions();

        let assets = Assets::new(ctx)?;

        let screen_width = ctx.conf.window_mode.width;
        let screen_height = ctx.conf.window_mode.height;

        let player = actors::player::create_player();
        let fruits = actors::fruit::create_fruits(1, screen_width, screen_height);

        Ok(MainState {
            player,
            fruits,
            level: 0,
            score: 0,
            assets,
            screen_width: ctx.conf.window_mode.width,
            screen_height: ctx.conf.window_mode.height,
            input: InputState::default(),
        })
    }
}

fn print_instructions() {
    println!("");
    println!("Eat the apples!");
    println!("Press L/R to move the eater");
    println!("");
    println!("Esc to finish game");
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Left => {
                self.input.xaxis = -1.0;
            }
            KeyCode::Right => {
                self.input.xaxis = 1.0;
            }
            KeyCode::Escape => ggez::quit(ctx),
            _ => (), // Do nothing
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        match keycode {
            KeyCode::Left | KeyCode::Right => {
                self.input.xaxis = 0.0;
            }
            _ => (), // Do nothing
        }
    }
}

fn main() -> GameResult {
    if cfg!(debug_assertions) && env::var("yes_i_really_want_debug_mode").is_err() {
        eprintln!(
            "Note: Release mode will improve performance greatly.\n    \
             e.g. use `cargo run --example text --release`"
        );
    }
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let (ctx, event_loop) = &mut ContextBuilder::new("falling_boys", "aaron")
        .window_setup(conf::WindowSetup::default().title("Falling Boys"))
        .window_mode(
            conf::WindowMode::default()
                // .dimensions(640.0, 480.0)
                .dimensions(1200.0, 800.0)
                .resizable(true),
        )
        .add_resource_path(resource_dir)
        .build()
        .unwrap();

    let mut state = MainState::new(ctx)?;
    event::run(ctx, event_loop, &mut state)
}

