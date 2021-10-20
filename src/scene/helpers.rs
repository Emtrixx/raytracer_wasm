use glam::{Vec3};
use crate::scene::*;

//Ray
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray {
        let fov_adjustment = (to_radians(scene.fov) / 2.).tan() ;
        let aspect_ratio = (scene.width as f32) / (scene.height as f32);
        let viewport_x = ((((x as f32 + 0.5) / scene.width as f32) * 2.0 - 1.0) * aspect_ratio) * fov_adjustment;
        let viewport_y = (1.0 - ((y as f32 + 0.5) / scene.height as f32) * 2.0) * fov_adjustment;
        
        Ray {
            origin: Vec3::ZERO, 
            direction: Vec3::new(viewport_x, viewport_y, -1.0).normalize(),
        }
    }
}

// Intersection
pub struct Intersection<'a> {
    pub distance: f32,
    pub element: &'a Element,
}

pub fn to_radians(x: f32) -> f32 {
    x / 57.296
}