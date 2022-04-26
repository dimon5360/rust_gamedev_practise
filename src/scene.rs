use piston_window::*;

use crate::camera::Camera;
use crate::player::Player;
use crate::object::Object;
use crate::sprite::{Sprite, SpriteEvent};
use crate::libs::{Tilemap, Rect};
use crate::weapon::Weapon;

use std::path::PathBuf;

pub struct Scene {
    player: Player,
    objects: Vec<Object>,
    camera: Camera,
}

impl Scene {

    pub fn new(assets: PathBuf, window: &mut PistonWindow) -> Scene {

        let k = 40.0;

        let tilemap = Tilemap::new(assets.join("map.txt").to_str().unwrap());
        let mut objects = Vec::new();

        let [width, height] = [window.size().width, window.size().height];

        let max_w = tilemap.first().unwrap().len() as f64 * k;
        let max_h = tilemap.len() as f64 * k;   
        
        // static objects
        let ground_texture = Sprite::load_texture(assets.join("ground.png"), window, Flip::None);
        let weapon_texture = Sprite::load_texture(assets.join("weapon.png"), window, Flip::None);
        let weapon_empty_cell_texture = Sprite::load_texture(assets.join("empty_weapon_cell.png"), window, Flip::None);
        let weapon_busy_cell_texture = Sprite::load_texture(assets.join("weapon.png"), window, Flip::None);
        let brick_texture = Sprite::load_texture(assets.join("brick.png"), window, Flip::None);
        let brick2_texture = Sprite::load_texture(assets.join("brick2.png"), window, Flip::None);
        let cloud_texture = Sprite::load_texture(assets.join("cloud.png"), window, Flip::None);
        let player_texture = Sprite::load_texture(assets.join("player.png"), window, Flip::None);
        let player_back_texture = Sprite::load_texture(assets.join("player.png"), window, Flip::Horizontal);
        
        // player's weapon object
        let weapon_rect = Rect::new(0.0, 0.0, 5.0, 0.0, 40.0);
        let weapon_sprite = Sprite::new(weapon_texture);
        let weapon_object = Weapon::new(weapon_sprite, weapon_rect);

        //  player's cells of weapon
        let weapon_cell_rect = Rect::new(200.0, 200.0, 5.0, 0.0, 40.0);
        let mut weapon_cell_sprite = Sprite::new(weapon_empty_cell_texture);
        let weapon_cell_object = Weapon::new(weapon_cell_sprite, weapon_cell_rect);

        // player object
		let mut player_sprite = Sprite::new(player_texture);
		player_sprite.add_texture(player_back_texture);
		let player_rect = Rect::new(0.0, 0.0, 5.0, 0.0, 40.0);
		let player = Player::new(player_sprite, player_rect, weapon_object);

        // fill static objects vector
        for (row, tiles) in tilemap.iter().enumerate() {
            for (col, tile) in tiles.iter().enumerate() {

                if *tile == '1' {
                    let rect = Rect::new(col as f64 * k, row as f64 * k, 0.0, 0.0, 40.0);
                    let ground_sprite = Sprite::new(ground_texture.clone());
                    let object = Object::new(ground_sprite, rect, true);
                    objects.push(object);
                }

                if *tile == '2' {
                    let rect = Rect::new(col as f64 * k, row as f64 * k, 0.0, 0.0, 40.0);
                    let brick_sprite = Sprite::new(brick_texture.clone());
                    let object = Object::new(brick_sprite, rect, true);
                    objects.push(object);
                }

                if *tile == '?' {
                    let rect = Rect::new(col as f64 * k, row as f64 * k, 0.0, 0.0, 40.0);
                    let brick2_sprite = Sprite::new(brick2_texture.clone());
                    let object = Object::new(brick2_sprite, rect, true);
                    objects.push(object);
                }

                if *tile == '@' {
                    let rect = Rect::new(col as f64 * k, row as f64 * k, 0.0, 0.0, 40.0);
                    let cloud_sprite = Sprite::new(cloud_texture.clone());
                    let object = Object::new(cloud_sprite, rect, false);
                    objects.push(object);
                }
            }
        }

        let cam_x = width / 2.0 - 50.0;
        let cam_y = height / 2.0 - 50.0;
    
        let camera = Camera::new(cam_x, cam_y, 100.0, 100.0, max_w, max_h);

        Scene {
            player: player,
            objects: objects,
            camera: camera,
        }
    }

    pub fn update(&mut self, event: & Event, window: &mut PistonWindow) {

        window.draw_2d(event, |_, graphics, _device| {
            clear(color::hex("aaeeffff"), graphics);
        });

        let width = window.size().width;
        let height = window.size().height;

        for object in self.objects.iter_mut().filter(|o| 
            o.rect.x.round() >= 0.0 && o.rect.x.round() <= width && 
            o.rect.y.round() >= 0.0 && o.rect.x.round() <= height) {
                object.render(event, window);
        }

        self.player.render(event, window);
        self.player.key_event(event);

        if let Some(u) = event.update_args() {
            self.player.update(u.dt, &mut self.objects);
            self.camera.update(&mut self.player, &mut self.objects);
        }
    }
}