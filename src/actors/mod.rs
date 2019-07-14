/// *********************************************************************
/// Now we define our Actor's.
/// An Actor is anything in the game world.
/// We're not *quite* making a real entity-component system but it's
/// pretty close.  For a more complicated game you would want a
/// real ECS, but for this it's enough to say that all our game objects
/// contain pretty much the same data.
/// **********************************************************************
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