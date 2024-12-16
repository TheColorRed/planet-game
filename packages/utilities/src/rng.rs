use rand_pcg::Pcg64;
use rand_seeder::Seeder;

pub struct Rng {
  rng: Pcg64,
}

impl Default for Rng {
  fn default() -> Self {
    Self::new("")
  }
}

impl Rng {
  /// Create a new Rng with the given seed.
  pub fn new(seed: &str) -> Self {
    Self {
      rng: Seeder::from(seed).make_rng(),
    }
  }

  /// Generate a random float between 0.0 and 1.0.
  pub fn next_f32(&mut self) -> f32 {
    use rand::Rng;
    self.rng.gen::<f32>()
  }

  /// Generate a random float between min and max.
  pub fn next_f32_range(&mut self, min: f32, max: f32) -> f32 {
    use rand::Rng;
    self.rng.gen_range(min..max)
  }

  /// Generate a random i32.
  pub fn next_i32(&mut self) -> i32 {
    use rand::Rng;
    self.rng.gen::<i32>()
  }

  /// Generate a random i32 between min and max.
  pub fn next_i32_range(&mut self, min: i32, max: i32) -> i32 {
    use rand::Rng;
    self.rng.gen_range(min..max)
  }

  /// Generate a random u8.
  pub fn next_u8(&mut self) -> u8 {
    use rand::Rng;
    self.rng.gen::<u8>()
  }

  pub fn next_color(&mut self) -> u32 {
    let red = self.next_u8() as u32;
    let green = self.next_u8() as u32;
    let blue = self.next_u8() as u32;
    let alpha = 0xFF;
    (red << 16) | (green << 8) | blue | (alpha << 24)
  }
}
