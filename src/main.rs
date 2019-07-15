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

type Point2 = nalgebra::Point2<f32>;
type Vector2 = nalgebra::Vector2<f32>;

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
        let fruits = actors::fruit::create_fruits(2, screen_width, screen_height);

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

/// Translates the world coordinate system, which
/// has Y pointing up and the origin at the center,
/// to the screen coordinate system, which has Y
/// pointing downward and the origin at the top-left,
fn world_to_screen_coords(screen_width: f32, screen_height: f32, point: Point2) -> Point2 {
    let x = point.x + screen_width / 2.0;
    let y = screen_height - (point.y + screen_height / 2.0);
    Point2::new(x, y)
}

/// Translates the world coordinate system to
/// coordinates suitable for the audio system.
fn world_to_audio_coords(screen_width: f32, screen_height: f32, point: Point2) -> [f32; 3] {
    let x = point.x * 2.0 / screen_width;
    let y = point.y * 2.0 / screen_height;
    let z = 0.0;
    [x, y, z]
}

fn draw_actor(
    assets: &mut Assets,
    ctx: &mut Context,
    actor: &Actor,
    world_coords: (f32, f32),
) -> GameResult {
    let (screen_w, screen_h) = world_coords;
    let pos = world_to_screen_coords(screen_w, screen_h, actor.pos);
    let image = assets.actor_image(actor);
    let drawparams = graphics::DrawParam::new()
        .dest(pos)
        // .rotation(actor.facing as f32)
        .scale(Vector2::new(2.0, 2.0))
        .offset(Point2::new(0.5, 0.5));
    graphics::draw(ctx, image, drawparams)
}

const MAX_PHYSICS_VEL: f32 = 250.0;

pub fn update_actor_position(actor: &mut Actor, dt: f32) {
    // Clamp the velocity to the max efficiently
    let norm_sq = actor.velocity.norm_squared();
    if norm_sq > MAX_PHYSICS_VEL.powi(2) {
        actor.velocity = actor.velocity / norm_sq.sqrt() * MAX_PHYSICS_VEL;
    }
    let dv = actor.velocity * (dt);
    actor.pos += dv;
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            let seconds = 1.0 / (DESIRED_FPS as f32);
            actors::player::player_handle_input(&mut self.player, &self.input, seconds);
            update_actor_position(&mut self.player, seconds);
            for f in &mut self.fruits {
                update_actor_position(f, seconds);
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Clear the screen...
        graphics::clear(ctx, graphics::BLACK);

        // Draw actors
        {
            let assets = &mut self.assets;
            let coords = (self.screen_width, self.screen_height);

            let p = &self.player;
            draw_actor(assets, ctx, p, coords).unwrap();

            for f in &self.fruits {
                draw_actor(assets, ctx, f, coords).unwrap();
            }
        }

        // Draw the UI
        // And draw the GUI elements in the right places.
        // let level_dest = Point2::new(10.0, 10.0);
        let score_dest = Point2::new(10.0, 10.0);
        let debug_dest = Point2::new(10.0, 32.0);
        let debug2_dest = Point2::new(10.0, 54.0);

        // let level_str = format!("Level: {}", self.level);
        let score_str = format!("Score: {}", self.score);
        let debug_str = format!("Player Velocity: {:?}", self.player.velocity);
        let debug2_str = format!("Fruits: {:?}", self.fruits);
        // let level_display = graphics::Text::new(level_str);
        let score_display = graphics::Text::new(score_str);
        let debug_display = graphics::Text::new(debug_str);
        let debug2_display = graphics::Text::new(debug2_str);
        // graphics::draw(ctx, &level_display, (level_dest, 0.0, graphics::WHITE))?;
        graphics::draw(ctx, &score_display, (score_dest, 0.0, graphics::WHITE))?;
        graphics::draw(ctx, &debug_display, (debug_dest, 0.0, graphics::WHITE))?;
        graphics::draw(ctx, &debug2_display, (debug2_dest, 0.0, graphics::WHITE))?;

        // Then we flip the screen...
        graphics::present(ctx)?;

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

