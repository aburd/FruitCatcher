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
mod util;
use util::{world_to_audio_coords, world_to_screen_coords};


struct MainState {
    player: Actor,
    fruits: Vec<Actor>,
    fruit_drop_wait: std::time::Duration,
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

        let player = actors::player::create_player(screen_width, screen_height);
        let fruits = actors::fruit::create_fruits(1, screen_width, screen_height);

        Ok(MainState {
            player,
            fruits,
            fruit_drop_wait: std::time::Duration::new(6, 0),
            level: 1,
            score: 0,
            assets,
            screen_width: ctx.conf.window_mode.width,
            screen_height: ctx.conf.window_mode.height,
            input: InputState::default(),
        })
    }

    fn handle_collisions(&mut self) {
        for fruit in &mut self.fruits {
            let pdistance = fruit.pos - self.player.pos;
            if pdistance.norm() < (self.player.bbox_size + fruit.bbox_size) {
                fruit.life = 0.0;
                self.score += 1;
            }
        }
    }

    fn remove_dead(&mut self) {
        self.handle_fruit_offscreen();
        self.fruits.retain(|f| f.life > 0.0);
    }

    fn handle_fruit_offscreen(&mut self) {
        for fruit in &mut self.fruits {
            if fruit.pos.y >= self.screen_height {
                fruit.life = 0.0;
            }
        }
    }
}

fn print_instructions() {
    println!("");
    println!("Eat the apples!");
    println!("Press L/R to move the eater");
    println!("");
    println!("Esc to finish game");
}

fn draw_actor(
    assets: &mut Assets,
    ctx: &mut Context,
    actor: &Actor,
    world_coords: (f32, f32),
) -> GameResult {
    let (screen_w, screen_h) = world_coords;
    // let pos = world_to_screen_coords(screen_w, screen_h, actor.pos);
    let pos = actor.pos;
    // Draw debug square
    let rect = graphics::Rect::new(
        pos.x - (actor.bbox_size / 2.0),
        pos.y - (actor.bbox_size / 2.0),
        actor.bbox_size,
        actor.bbox_size,
    );
    let r1 = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, graphics::WHITE)?;
    graphics::draw(ctx, &r1, graphics::DrawParam::default())?;
    // Draw character
    let image = assets.actor_image(actor);
    let drawparams = graphics::DrawParam::new()
        .dest(pos)
        // .rotation(actor.facing as f32)
        .scale(Vector2::new(4.0, 4.0))
        .offset(Point2::new(0.5, 0.5));
    graphics::draw(ctx, image, drawparams)
}

const MAX_PHYSICS_VEL: f32 = 400.0;

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
            self.handle_collisions();
            self.remove_dead();
        }

        self.fruit_drop_wait -= timer::delta(ctx);
        if self.fruit_drop_wait.as_secs() <= 0 {
            let fruit = actors::fruit::create_fruits(1, self.screen_width, self.screen_height);
            self.fruits.extend(fruit);
            self.fruit_drop_wait = std::time::Duration::new(3, 0);
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

        // let level_str = format!("Level: {}", self.level);
        let score_str = format!("Score: {}", self.score);
        // let debug_str = format!("Player Velocity: {:?}", self.player.velocity);
        let debug_str = format!("Player Pos: {:?}", self.player.pos);
        // let debug_str = format!("Fruits: {:?}", self.fruits);
        // let level_display = graphics::Text::new(level_str);
        let score_display = graphics::Text::new(score_str);
        let debug_display = graphics::Text::new(debug_str);
        // graphics::draw(ctx, &level_display, (level_dest, 0.0, graphics::WHITE))?;
        graphics::draw(ctx, &score_display, (score_dest, 0.0, graphics::WHITE))?;
        graphics::draw(ctx, &debug_display, (debug_dest, 0.0, graphics::WHITE))?;

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
            KeyCode::Left => controls::handle_left(&mut self.player, &mut self.input),
            KeyCode::Right => controls::handle_right(&mut self.player, &mut self.input),
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

