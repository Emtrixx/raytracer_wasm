mod elements;
mod lights;
mod color;
mod helpers;

pub use elements::*;
pub use lights::*;
pub use color::*;
pub use helpers::*;

//Scene
pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f32,
    pub elements: Vec<Element>,
    pub lights: Vec<Light>,
}

impl Scene {
    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        let mut intersection: Option<Intersection> = None;
        let mut nearest = f32::INFINITY;
        for element in &self.elements {
            if let Some(dist) = element.intersect(ray) {
                if dist < nearest {
                    nearest = dist;
                    intersection = Some(Intersection {
                        element,
                        distance: nearest
                    });
                }
            } 
        }
        intersection
    }
}