use piston_window::*;

use crate::sprite::{Sprite, SpriteEvent};
use crate::libs::Rect;

#[derive(Clone)]
pub struct Object {
    sprite: Sprite,
    pub rect: Rect,
    pub solid: bool
}

impl Object {
    pub fn new(sprite: Sprite, rect: Rect, solid: bool) -> Object {
        Object {
            sprite: sprite,
            rect: rect,
            solid: solid
        }
    }
}

impl SpriteEvent for Object {

    fn render(&mut self, event: &Event, window: &mut PistonWindow) {

        let texture = self.sprite.get_texture(0);
        let rect = &self.rect;

        window.draw_2d(event, |context, graphics, _device| {
            image(texture, 
                context.trans(rect.x, rect.y)
                .scale(rect.scale / texture.get_width() as f64, rect.scale / texture.get_height() as f64)
                .transform, graphics
            );        
        });
    }
}