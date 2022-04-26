use piston_window::*;
use std::path::PathBuf;

pub trait SpriteEvent {
    fn render(&mut self, event: &Event, window: &mut PistonWindow);
}

#[derive(Default, Clone)]
pub struct Sprite {
    texture: Vec<G2dTexture>,
}

impl Sprite {
    pub fn load_texture(path: PathBuf, window: &mut PistonWindow, flip: Flip) -> G2dTexture {
        Texture::from_path(&mut window.create_texture_context(),
            path, flip, &TextureSettings::new()).unwrap()
    }

    pub fn new(texture: G2dTexture) -> Sprite {
        Sprite {
            texture: vec![texture]
        }
    }

    pub fn get_texture(&self, idx: usize) -> &G2dTexture {
        &self.texture[idx]
    }

    pub fn add_texture(&mut self, texture: G2dTexture) {
        self.texture.push(texture);
    }
  
}