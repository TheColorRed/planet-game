use godot::classes::image::Format;
use godot::classes::{INode2D, Image, ImageTexture, Node2D, RenderingServer, Sprite2D, Timer};
use godot::prelude::*;
use textures::TextureBuilder;
use utilities::Rng;

use crate::CelestialBody;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Planet {
  #[export]
  /// The name of the planet.
  name: GString,
  #[export]
  /// The size of the planet.
  size: i32,
  /// The sprite of where the planet will be drawn.
  sprite: Gd<Sprite2D>,
  /// The random number generator.
  rng: Rng,
  /// The base node.
  base: Base<Node2D>,
}

#[godot_api]
impl INode2D for Planet {
  fn init(base: Base<Node2D>) -> Self {
    Self {
      name: "Planet".into(),
      base,
      size: 100,
      sprite: Sprite2D::new_alloc(),
      rng: Rng::default(),
    }
  }

  fn enter_tree(&mut self) {
    // Create the sprite and add it as a child of the base node.
    let sprite: Gd<Sprite2D> = self.sprite.clone().upcast();
    self.base_mut().add_child(&sprite);

    // Setup the random number generator.
    self.rng = Rng::new(self.name.to_string().as_str());

    // Create the planet texture.
    self.create_texture();
  }
}

impl CelestialBody for Planet {
  fn create_texture(&mut self) {
    let mut builder = TextureBuilder::new(self.size, self.size);
    // builder.ellipse_from_center(0x00FF00);
    builder.ellipse_from_center(self.rng.next_color());

    let image = Image::create_from_data(
      builder.width(),
      builder.height(),
      false,
      Format::RGBA8,
      &PackedByteArray::from(builder.data()),
    );

    let mut rendering_server = RenderingServer::singleton();
    match image {
      Some(image) => {
        rendering_server.texture_2d_create(&image);
        let texture = ImageTexture::create_from_image(&image);
        match texture {
          Some(texture) => self.sprite.set_texture(&texture),
          None => godot_warn!("Failed to load planet texture from rendering server"),
        }
      }
      None => godot_warn!("Failed to create planet image"),
    }
  }
}
