/// *********************************************************************
/// Now we define our Actor's.
/// An Actor is anything in the game world.
/// We're not *quite* making a real entity-component system but it's
/// pretty close.  For a more complicated game you would want a
/// real ECS, but for this it's enough to say that all our game objects
/// contain pretty much the same data.
/// **********************************************************************
use crate::controls::InputState;

type Point2 = nalgebra::Point2<f32>;
type Vector2 = nalgebra::Vector2<f32>;

#[derive(Debug)]
pub enum ActorType {
  Player,
  Fruit,
}

#[derive(Debug)]
pub struct Actor {
  pub tag: ActorType,
  pub pos: Point2,
  pub velocity: Vector2,
  pub bbox_size: f32,
  pub life: f32,
  pub speed: f32,
}

pub mod player {
  use super::{Actor, ActorType, InputState};
  use ggez::nalgebra as na;
  type Point2 = nalgebra::Point2<f32>;
  type Vector2 = nalgebra::Vector2<f32>;

  pub fn create_player(sw: f32, sh: f32) -> Actor {
    let bbox_size = 60.0;
    let padding_bottom = 5.0;
    let init_pos = Point2::new(sw / 2.0, sh - (bbox_size / 2.0) - padding_bottom);
    Actor {
      tag: ActorType::Player,
      pos: init_pos,
      velocity: na::zero(),
      bbox_size,
      life: 1.0,
      speed: 10.0,
    }
  }

  const PLAYER_SPEED: f32 = 2.0;

  pub fn player_handle_input(actor: &mut Actor, input: &InputState, dt: f32) {
    let v = Vector2::new(PLAYER_SPEED * input.xaxis, 0.0);
    actor.velocity += v;
  }
}

pub mod fruit {
  use super::{Actor, ActorType};

  type Point2 = nalgebra::Point2<f32>;
  type Vector2 = nalgebra::Vector2<f32>;

  pub fn create_fruits(n: u32, sw: f32, sh: f32) -> Vec<Actor> {
    (0..n).map(|_| create_fruit(sw, sh)).collect()
  }

  fn create_fruit(sw: f32, sh: f32) -> Actor {
    let x = rand::random::<f32>() * sw;
    let pos = Point2::new(x, 0.0);
    let falling_speed = 100.0;

    Actor {
      tag: ActorType::Fruit,
      pos,
      velocity: Vector2::new(0.0, falling_speed),
      bbox_size: 2.0,
      life: 1.0,
      speed: falling_speed,
    }
  }
}