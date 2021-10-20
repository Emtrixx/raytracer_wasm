use crate::scene::*;
use glam::{Vec3};
//Light
pub struct Light {
    pub kind: LightKind,
    pub color: Color,
    pub intensity: f32,
}

pub enum LightKind {
    Ambient,
    Point {
        position: Vec3,
    },
    Directional {
        direction: Vec3,
    }
}