mod planet;

pub use planet::Planet;

pub trait CelestialBody {
  fn create_texture(&mut self);
}
