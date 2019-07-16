/// **********************************************************************
/// The `InputState` is exactly what it sounds like, it just keeps track of
/// the user's input state so that we turn keyboard events into something
/// state-based and device-independent.
/// **********************************************************************
use crate::actors::Actor;

#[derive(Debug)]
pub struct InputState {
  pub xaxis: f32,
}

impl Default for InputState {
  fn default() -> Self {
    InputState {
      xaxis: 0.0,
    }
  }
}

pub fn handle_left(player: &mut Actor, input: &mut InputState) {
  if player.velocity[0] > 0.0 {
      player.velocity[0] = 0.0;
  }
  input.xaxis = -player.speed;
}

pub fn handle_right(player: &mut Actor, input: &mut InputState) {
  if player.velocity[0] < 0.0 {
      player.velocity[0] = 0.0;
  }
  input.xaxis = player.speed;
}