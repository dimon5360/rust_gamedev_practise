
use crate::object::Object;
use crate::libs::Rect;

pub enum Interact {
    LEFT, RIGHT, TOP, BOTTOM,
}

trait Collision {
    fn collision(&mut self, object_rect: &Rect, target: &Vec<Object>) -> bool;
}

pub struct PlayerCollider {
    pub interact: Option<(Interact, f64)>
}

#[derive(Default)]
pub struct WeaponCollider {
    pub interact: Option<(Interact, f64)>
}

impl PlayerCollider {

    pub fn collision(&mut self, object_rect: &Rect, target: &Vec<Object>) -> bool {
        
        for collision_object in target.iter() {
            if collision_object.solid {
                
                if object_rect.right() > collision_object.rect.left() && 
                   object_rect.left() < collision_object.rect.right() &&
                   object_rect.bottom() > collision_object.rect.top() && 
                   object_rect.top() < collision_object.rect.bottom(){
                       
                    let _x = object_rect.center().x - collision_object.rect.center().x;
                    let _y = object_rect.center().y - collision_object.rect.center().y;
                    
                    if _y * _y > _x * _x {
                        if _y > 0.0 {
                            self.interact = Some((Interact::TOP, collision_object.rect.bottom()));
                            return true;
                        } if _y < 0.0 {
                            self.interact = Some((Interact::BOTTOM, collision_object.rect.top() - object_rect.scale));
                            return true;
                        }
                    } if _y * _y < _x * _x {
                        if _x > 0.0 {
                            self.interact = Some((Interact::LEFT, collision_object.rect.right()));
                            return true;
                        } if _x < 0.0 {
                            self.interact = Some((Interact::RIGHT, collision_object.rect.left() - object_rect.scale));
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
}

impl WeaponCollider {

    pub fn collision(&mut self, object_rect: &Rect, target: &Vec<Object>) -> bool {
        
        for collision_object in target.iter() {
            if collision_object.solid {
                
                if object_rect.right() >= collision_object.rect.left() && 
                   object_rect.left() <= collision_object.rect.right() {
                       
                    let _x = object_rect.center().x - collision_object.rect.center().x;
                    let _y = object_rect.center().y - collision_object.rect.center().y;

                    if  _y * _y < _x * _x {
                        if _x <= 40.0 {
                            self.interact = Some((Interact::LEFT, collision_object.rect.right()));
                            return true;
                        } if _x >= -40.0 {
                            self.interact = Some((Interact::RIGHT, collision_object.rect.left() - object_rect.scale));
                            return true;
                        }
                    }
                }

                if object_rect.bottom() > collision_object.rect.top() && 
                   object_rect.top() < collision_object.rect.bottom() {
                       
                    let _x = object_rect.center().x - collision_object.rect.center().x;
                    let _y = object_rect.center().y - collision_object.rect.center().y;

                    if _y * _y > _x * _x {
                        if _y >= 40.0 {
                            self.interact = Some((Interact::TOP, collision_object.rect.bottom()));
                            return true;
                        } if _y <= -40.0 {
                            self.interact = Some((Interact::BOTTOM, collision_object.rect.top() - object_rect.scale));
                            return true;
                        }
                    } 
                }
            }
        }
        false
    }
}