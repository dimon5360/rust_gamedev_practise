use piston_window::*;

use crate::player::Player;
use crate::object::Object;

pub struct Camera {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
    pub max_w: f64,
    pub max_h: f64,
}

impl Camera {

    pub fn new(x: f64, y: f64, w: f64, h: f64, max_w: f64, max_h: f64) -> Camera {
        Camera {
            x: x, y: y, w: w, h: h, max_w: max_w, max_h: max_h
        }
    }

    pub fn _show(&mut self, event: &Event, window: &mut PistonWindow) {
        window.draw_2d(event, |context, graphics, _device| {
            Rectangle::new_border([1.0, 0.0, 0.0, 1.0], 1.0)
                .draw([self.x, self.y, self.w, self.h],
                    &DrawState::default(), 
                    context.transform, graphics
                );
        });
    }

    pub fn update(&mut self, player: &mut Player, objects: &mut Vec<Object>) {

        if player.rect.x <= self.x {
            player.rect.x = self.x;
            for object in objects.iter_mut() {
                object.rect.x -= player.vel.x;
            }
            player.weapon.camera_coors_offset(player.vel.x, 0.0);
        }

        if player.rect.y <= self.y {
            player.rect.y = self.y;
            for object in objects.iter_mut() {
                object.rect.y -= player.vel.y;
            }
            player.weapon.camera_coors_offset(0.0, -player.vel.y);
        }

        if player.rect.x + player.rect.scale >= self.x + self.w {
            player.rect.x = self.x + self.w - player.rect.scale;
            for object in objects.iter_mut() {
                object.rect.x -= player.vel.x;
            }
            player.weapon.camera_coors_offset(player.vel.x, 0.0);
        }

        if player.rect.y + player.rect.scale >= self.y + self.h {
            player.rect.y = self.y + self.h - player.rect.scale;
            for object in objects.iter_mut() {
                object.rect.y -= player.vel.y;
            }
            player.weapon.camera_coors_offset(0.0, -player.vel.y);
        }
    }
}