// src/lib.rs

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, raytracerwasm!");
}
pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}


// 
// Raytracer
// 
mod utils;
mod scene;
use glam::{Vec3};
use scene::*;


// 
// FUNCTIONS
//

#[wasm_bindgen]
pub struct Frame {
    width: u32,
    height: u32,
    data:  Vec<u8>,
}

#[wasm_bindgen]
impl Frame {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn data(&self) -> *const u8 {
        self.data.as_ptr()
    }
    pub fn create(width: u32, height: u32, brightness: f32, sphere_dx: f32, sphere_dy: f32, sphere_dz: f32) -> Frame {
        set_panic_hook();
        let sphere_red = Element::Sphere( Sphere {
            center: Vec3::new(0. + sphere_dx, -0.5 + sphere_dy, -4.5 + sphere_dz),
            radius: 1.,
            material: Material {
                color: Color {
                    red: 255.,
                    green: 0.,
                    blue: 0.
                },
                albedo: 1.,
                specular: 50.,
                reflectivity: 0.9,
            }
        });
        let sphere_green = Element::Sphere( Sphere {
            center: Vec3::new(2.,1.,-2.6),
            radius: 1.,
            material: Material {
                color: Color {
                    red: 0.,
                    green: 255.,
                    blue: 0.
                },
                albedo: 1.,
                specular: 50.,
                reflectivity: 0.,
            }
        });
        let sphere_blue = Element::Sphere( Sphere {
            center: Vec3::new(-3.,-1.,-5.5),
            radius: 1.,
            material: Material {
                color: Color {
                    red: 0.,
                    green: 0.,
                    blue: 255.
                },
                albedo: 1.,
                specular: 10.,
                reflectivity: 0.2,
            }
        });

        //Plane
        let plane = Element::Plane(Plane {
            origin: Vec3::new(0.,-4.,0.),
            normal: Vec3::new(0.,-1.,0.),
            material: Material {
                color: Color {
                    red: 60.,
                    green: 60.,
                    blue: 60.
                },
                albedo: 1.,
                specular: -1.,
                reflectivity: 0.,
            }
        });

        let elements = vec![sphere_red, sphere_green, sphere_blue, plane];


        // Lights
        let ambient = Light {
            kind: LightKind::Ambient,
            intensity: 0.06 * brightness,
            color: Color {
                red: 1.,
                green: 1.,
                blue: 1.,
            }
        };

        let point = Light {
            kind: LightKind::Point {
                position: Vec3::new(-1., -1.,-2.5)
            },
            intensity: 6.2 * brightness,
            color: Color {
                red: 1.,
                green: 1.,
                blue: 1.,
            }
        };

        let directional = Light {
            kind: LightKind::Directional {
                direction: Vec3::new(0., -1., -2.).normalize(),
            },
            intensity: 0.8 * brightness,
            color: Color {
                red: 1.,
                green: 1.,
                blue: 1.,
            }
        };

        let lights = vec![ambient, directional, point];

        let scene = Scene {
            width,
            height,
            elements,
            fov: 70.,
            lights,
        };

        let data = render(scene);

        Frame {
            width,
            height,
            data,
        }
    }
}

pub fn render(scene: Scene) -> Vec<u8> {
    let mut image: Vec<u8> = Vec::with_capacity((scene.width * scene.height * 3) as usize);

    let recursion_depth = 3;
    let background = Color{
        red: 0.,
        green: 0.,
        blue: 0.,
    };

    for y in 0..scene.height {
        for x in 0..scene.width {
            let ray = Ray::create_prime(x, y, &scene);
            let intersection = scene.trace(&ray);

            if let Some(inter) = intersection {
                let color = get_color(&scene, &ray, inter, recursion_depth);
                image.push(color.red as u8);
                image.push(color.green as u8);
                image.push(color.blue as u8);
            } else {
                image.push(background.red as u8);
                image.push(background.green as u8);
                image.push(background.blue as u8);
            }
        };
    };
    image
}

fn get_color(scene: &Scene, ray: &Ray, intersection: Intersection, recursion_depth: u32) -> Color {
    let Intersection { element, distance } = intersection;
    let hit_point = ray.origin + (ray.direction * distance);
    let surface_normal = element.surface_normal(&hit_point);

    let mut color = element.color();

    let mut intensity = 0.0;
    for light in &scene.lights {
        color = color.multiply(light.color);
        let mut light_intensity = light.intensity;
        match light.kind {
            LightKind::Ambient => { intensity += light.intensity; }
            LightKind::Point { position } => {
                let mut impact_to_light =  position - hit_point;
                let distance_squared = impact_to_light.dot(impact_to_light);
                light_intensity = light_intensity / distance_squared;
                impact_to_light = impact_to_light.normalize();

                let normal_dot_impact = surface_normal.dot(impact_to_light);

                //Shadow
                let shadow_ray = Ray {
                    origin: hit_point +  (surface_normal * 1e-4),
                    direction: impact_to_light,
                };
                let shadow_intersection = scene.trace(&shadow_ray);
                let in_light =  shadow_intersection.is_none() || shadow_intersection.unwrap().distance.powi(2) > distance_squared;

                if normal_dot_impact > 0. && in_light {
                    intensity += normal_dot_impact * light_intensity;
                };

                //Specular
                if element.specular() != -1. {
                    let light_exit = (2. * surface_normal * impact_to_light.dot(surface_normal) - impact_to_light).normalize();
                    let resemblance = light_exit.dot(-ray.direction);
                    if resemblance > 0. {
                        intensity += light_intensity * (resemblance * resemblance as f32).powf(element.specular());
                    };
                }

            }
            LightKind::Directional { direction } => {
                let impact_to_light = - direction;
                let normal_dot_impact = surface_normal.dot(impact_to_light);

                //Shadow
                let shadow_ray = Ray {
                    origin: hit_point +  (surface_normal * 1e-4),
                    direction: impact_to_light,
                };
                let in_light = scene.trace(&shadow_ray).is_none();

                if normal_dot_impact > 0. && in_light {
                    intensity += normal_dot_impact * light_intensity;
                };


            }
        };
    }
    let light_reflected = element.albedo() / std::f32::consts::PI;
    color = color
        .multiply_scalar(light_reflected)
        .multiply_scalar(intensity)
        .clamp();

    //Reflection
    if recursion_depth > 0 && element.reflectivity() > 0. {
        let ray_exit = ray.direction - (2. * ray.direction.dot(surface_normal) * surface_normal);

        let ray = Ray {
            origin: hit_point + (surface_normal * 1e-5),
            direction:  ray_exit
        };
        let intersection = scene.trace(&ray);

        if let Some(inter) = intersection {
            let reflection_color = get_color(&scene, &ray, inter, recursion_depth - 1).multiply_scalar(element.reflectivity());
            color = color.multiply_scalar(1. - element.reflectivity());
            color = color.add(reflection_color.multiply_scalar(element.reflectivity()));
        } else {
            //TODO get background color
            let background_color = Color {
                red: 40.,
                green: 40.,
                blue: 60.,
            };
            color = color.multiply_scalar(1. - element.reflectivity());
            color = color.add(background_color.multiply_scalar(element.reflectivity()));
        }
    }

    color
}