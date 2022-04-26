use piston_window::*;

use crate::sprite::{Sprite, SpriteEvent};
use crate::libs::Rect;
use crate::collider::{WeaponCollider, Interact};
use crate::object::Object;

#[derive(Default)]
pub struct Weapon {
    is_exist: bool,
    sprite: Sprite,
    rect: Rect,
    distance: f64,
    acceleration: f64,
    collider: WeaponCollider,
    collisions: Vec<Object>,
}

pub struct WeaponCell {
    sprite: Sprite,
    rect: Rect,
    weapons: Vec<Weapon>,
    first_free_cell_idx: usize,
}

impl WeaponCell {

    const MAX_SIZE: usize = 6;

    pub fn new(sprite: Sprite, rect: Rect) -> WeaponCell {

        WeaponCell { 
            first_free_cell_idx: 0,
            sprite: sprite,
            rect: rect,
            weapons: Vec::with_capacity(WeaponCell::MAX_SIZE),
        }
    }

    pub fn add(&mut self, new_weapon: Weapon) {

        if self.first_free_cell_idx < WeaponCell::MAX_SIZE {
            self.weapons[self.first_free_cell_idx] = new_weapon;
        }
    }

    pub fn update(&mut self, idx: usize, new_weapon: Weapon) {
        if idx < WeaponCell::MAX_SIZE {
            self.weapons[idx] = new_weapon;
        }
    }

    pub fn remove(&mut self, idx: usize) {
        if idx < WeaponCell::MAX_SIZE {
            self.weapons[idx] = Default::default();
        }
    }
}

impl Weapon {

    const DISTANCE: f64 = 150.0;
    pub const ACCELERATION: f64 = 3.0;

    pub fn new(sprite: Sprite, rect: Rect) -> Weapon {
        Weapon { 
            is_exist: false,
            sprite: sprite,
            rect: rect,
            distance: 0.0, 
            acceleration: 0.0,
            collider: WeaponCollider { interact: None },
            collisions: Vec::new(),
        }
    }

    pub fn default() -> Self {
        Weapon { 
            is_exist: false,
            distance: 0.0, 
            acceleration: 0.0,
            sprite: Default::default(),
            rect: Default::default(),
            collider: WeaponCollider::default(),
            collisions: Vec::new(),
        }
    }

    pub fn set(&mut self, x: f64, y: f64, acc: f64, object: Vec<Object>) {
        self.is_exist = true;
        self.rect.x = x + acc;
        self.rect.y = y;
        self.acceleration = acc;
        self.distance = Weapon::DISTANCE;
        self.collisions = object;
    }

    pub fn update(&mut self) {

        if self.collider.collision(&self.rect, &self.collisions) {
            if let Some((interact, _pos)) = &self.collider.interact {
                match interact {
                    Interact::BOTTOM => {
                        self.is_exist = false;
                    },
                    Interact::TOP => {
                        self.is_exist = false;
                    },
                    Interact::RIGHT => {
                        self.is_exist = false;
                    },
                    Interact::LEFT => {
                        self.is_exist = false;
                    }
                }
            }
        }

        self.rect.x += self.acceleration;
        self.distance -= Weapon::ACCELERATION;

        if self.distance <= 0.0 {
            self.is_exist = false;
        }

    }

    pub fn is_exist(&self) -> bool {
        self.is_exist
    }
    pub fn _set_not_exist(&mut self) {
        self.is_exist = false;
    }

    pub fn camera_coors_offset(&mut self, x: f64, y: f64) {
        if self.is_exist() {
            self.rect.x += x;
            self.rect.y += y;
        }
    }
}

impl SpriteEvent for Weapon {
    fn render(&mut self, event: &Event, window: &mut PistonWindow) {

        self.update();

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

impl SpriteEvent for WeaponCell {
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