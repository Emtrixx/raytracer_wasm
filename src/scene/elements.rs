use glam::{Vec3};
use crate::scene::*;

//Material
pub struct Material {
    pub color: Color,
    pub albedo: f32,
    pub specular: f32,
    pub reflectivity: f32,
}


//Element
pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f32>;
    fn surface_normal(&self, hit_point: &Vec3) -> Vec3;
}

pub enum Element {
    Sphere(Sphere),
    Plane(Plane),
}

impl Intersectable for Element {
    fn intersect(&self ,ray: &Ray) -> Option<f32> {
        match *self {
            Element::Sphere(ref s) => s.intersect(ray),
            Element::Plane(ref p) => p.intersect(ray),
        }
    }
    fn surface_normal(&self ,hit_point: &Vec3) -> Vec3 {
        match *self {
            Element::Sphere(ref s) => s.surface_normal(hit_point),
            Element::Plane(ref p) => p.surface_normal(hit_point),
        }
    }
}

impl Element {
    pub fn color(&self) -> Color {
        match *self {
            Element::Sphere(ref s) => s.material.color,
            Element::Plane(ref p) => p.material.color,
        }
    }
    pub fn albedo(&self) -> f32 {
        match *self {
            Element::Sphere(ref s) => s.material.albedo,
            Element::Plane(ref p) => p.material.albedo,
        }
    }
    pub fn specular(&self) -> f32 {
        match *self {
            Element::Sphere(ref s) => s.material.specular,
            Element::Plane(ref p) => p.material.specular,
        }
    }
    pub fn reflectivity(&self) -> f32 {
        match *self {
            Element::Sphere(ref s) => s.material.reflectivity,
            Element::Plane(ref p) => p.material.reflectivity,
        }
    }
}


//Sphere
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        //from origin to center of sphere
        let line_to_center = self.center - ray.origin;
        let ray_section_length = line_to_center.dot(ray.direction);
        //Faster than standard Pythagoras: h**2 - adj**2
        let connection_squared = line_to_center.dot(line_to_center) - ray_section_length * ray_section_length;
        let radius_squared = self.radius * self.radius;

        if connection_squared > radius_squared {
            return None;
        }
        
        let thic = ((self.radius * self.radius) - connection_squared).sqrt();
        
        // let connection = connection_squared.sqrt();

        // let right_angle_point = self.center + connection;

        let distance_intersection1 = ray_section_length - thic;
        let distance_intersection2 = ray_section_length + thic;

        if distance_intersection1 < 0. && distance_intersection2 < 0. {
            return None;
        };

        let dist = if distance_intersection1 < distance_intersection2 { distance_intersection1 } else { distance_intersection2 };
        Some(dist)
    }
    fn surface_normal(&self, hit_point: &Vec3) -> Vec3 {
        (*hit_point - self.center).normalize()
    }
}

//Plane
pub struct Plane {
    pub origin: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        let normal = self.normal;
        let denom = normal.dot(ray.direction);
        if denom > 1e-6 {
            let v = self.origin - ray.origin;
            let distance = v.dot(normal) / denom;
            if distance >= 0.0 {
                return Some(distance);
            }
        }
        None
    }
    fn surface_normal(&self, _: &Vec3) -> Vec3 {
        -self.normal
    }
}