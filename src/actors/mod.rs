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
}

pub mod player {
  use super::{Actor, ActorType, InputState};
  use ggez::nalgebra as na;
  type Point2 = nalgebra::Point2<f32>;
  type Vector2 = nalgebra::Vector2<f32>;

  pub fn create_player() -> Actor {
    Actor {
      tag: ActorType::Player,
      pos: Point2::origin(),
      velocity: na::zero(),
      bbox_size: 2.0,
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
  use ggez::nalgebra as na;
  use rand;
  type Point2 = nalgebra::Point2<f32>;

  pub fn create_fruits(n: u32, sw: f32, sh: f32) -> Vec<Actor> {
    (0..=n).map(|_| create_fruit(sw, sh)).collect()
  }

  fn create_fruit(sw: f32, sh: f32) -> Actor {
    let x = rand::random::<f32>() + sw / 2.0;
    let y = sh + 1.0;

    Actor {
      tag: ActorType::Fruit,
      pos: Point2::new(x, y),
      velocity: na::zero(),
      bbox_size: 2.0,
    }
  }
}