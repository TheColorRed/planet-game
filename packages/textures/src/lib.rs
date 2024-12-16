pub struct TextureBuilder {
  width: i32,
  height: i32,
  data: Vec<u8>,
}

impl TextureBuilder {
  pub fn new(width: i32, height: i32) -> Self {
    let data = vec![0; (width * height * 4) as usize];
    Self {
      width,
      height,
      data,
    }
  }

  pub fn width(&self) -> i32 {
    self.width
  }

  pub fn height(&self) -> i32 {
    self.height
  }

  pub fn data(&self) -> &[u8] {
    &self.data
  }

  pub fn fill(&mut self, color: u32) {
    let red = ((color >> 16) & 0xFF) as u8;
    let green = ((color >> 8) & 0xFF) as u8;
    let blue = (color & 0xFF) as u8;
    let alpha = 0xFF;
    for i in 0..self.data.len() / 4 {
      self.data[i * 4] = red;
      self.data[i * 4 + 1] = green;
      self.data[i * 4 + 2] = blue;
      self.data[i * 4 + 3] = alpha;
    }
  }

  /// Add a circle method to the TextureBuilder struct that takes an x, y, radius, and color as arguments.
  /// - The x and y arguments are the center of the circle.
  /// - The radius argument is the radius of the circle.
  /// - The color argument is a u32 value that represents the color of the circle formatted as 0xRRGGBB or 0xRRGGBBAA.
  pub fn ellipse(&mut self, x: i32, y: i32, x_radius: i32, y_radius: i32, color: u32) {
    let red = ((color >> 16) & 0xFF) as u8;
    let green = ((color >> 8) & 0xFF) as u8;
    let blue = (color & 0xFF) as u8;
    let alpha = 0xFF;
    let samples = 4; // Number of samples per pixel for supersampling
    let inv_samples = 1.0 / samples as f32;

    for i in 0..self.width {
      for j in 0..self.height {
        let mut red_acc = 0 as u32;
        let mut green_acc = 0 as u32;
        let mut blue_acc = 0 as u32;
        let mut alpha_acc = 0 as u32;

        for si in 0..samples {
          for sj in 0..samples {
            let sub_i = i as f32 + (si as f32 + 0.5) * inv_samples;
            let sub_j = j as f32 + (sj as f32 + 0.5) * inv_samples;
            let dx = sub_i - x as f32;
            let dy = sub_j - y as f32;
            let y_radius = y_radius as f32;
            let x_radius = x_radius as f32;

            if dx * dx * y_radius * y_radius + dy * dy * x_radius * x_radius
              <= x_radius * x_radius * y_radius * y_radius
            {
              red_acc += red as u32;
              green_acc += green as u32;
              blue_acc += blue as u32;
              alpha_acc += alpha as u32;
            }
          }
        }

        let index = (i + j * self.width) as usize;
        self.data[index * 4] = (red_acc / (samples * samples)) as u8;
        self.data[index * 4 + 1] = (green_acc / (samples * samples)) as u8;
        self.data[index * 4 + 2] = (blue_acc / (samples * samples)) as u8;
        self.data[index * 4 + 3] = (alpha_acc / (samples * samples)) as u8;
      }
    }
  }

  pub fn ellipse_from_center(&mut self, color: u32) {
    let x = self.width / 2;
    let y = self.height / 2;
    let x_radius = self.width / 2;
    let y_radius = self.height / 2;
    self.ellipse(x, y, x_radius, y_radius, color);
  }
}
