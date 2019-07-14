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
use actors::{Actor};


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

        let player = player::create_player();
        let rocks = rock::create_rocks(1, player.pos, 100.0, 250.0);

        Ok(MainState {
            player: Actor,
            fruits: Vec<Actor>,
            level: i32,
            score: i32,
            assets: Assets,
            screen_width: ctx.conf.window_mode.width,
            screen_height: ctx.conf.window_mode.height,
            input: InputState,
        })
    }
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
        // match keycode {
        //     KeyCode::Up => {
        //         self.input.yaxis = 1.0;
        //     }
        //     KeyCode::Left => {
        //         self.input.xaxis = -1.0;
        //     }
        //     KeyCode::Right => {
        //         self.input.xaxis = 1.0;
        //     }
        //     KeyCode::Space => {
        //         self.input.fire = true;
        //     }
        //     KeyCode::P => {
        //         let img = graphics::screenshot(ctx).expect("Could not take screenshot");
        //         img.encode(ctx, graphics::ImageFormat::Png, "/screenshot.png")
        //             .expect("Could not save screenshot");
        //     }
        //     KeyCode::Escape => ggez::quit(ctx),
        //     _ => (), // Do nothing
        // }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        // match keycode {
        //     KeyCode::Up => {
        //         self.input.yaxis = 0.0;
        //     }
        //     KeyCode::Left | KeyCode::Right => {
        //         self.input.xaxis = 0.0;
        //     }
        //     KeyCode::Space => {
        //         self.input.fire = false;
        //     }
        //     _ => (), // Do nothing
        // }
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

