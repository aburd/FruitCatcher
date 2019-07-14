/// **********************************************************************
/// The `InputState` is exactly what it sounds like, it just keeps track of
/// the user's input state so that we turn keyboard events into something
/// state-based and device-independent.
/// **********************************************************************
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