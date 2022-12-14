extern crate minifb;

mod math;
mod geo;
mod random;

use std::rc::Rc;

use minifb::{Key, Window, WindowOptions };
use math::{float3, ray};
use geo::{Sphere, RayHit, RayPayload, Scene, LambertianMaterial, MetallicMaterial, DielectricMaterial };

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

pub struct Camera
{
    pub viewport_height: f32,
    pub viewport_width: f32,
    pub focal_length: f32,
    pub origin: float3,
    pub forward: float3,
    pub right: float3,
    pub up: float3,

    pub horizontal: float3,
    pub vertical: float3,
    pub corner: float3,
}

fn create_camera(eye: float3, lookat: float3, up: float3, vfov: f32, aspect: f32) -> Camera
{
    let theta = f32::to_radians(vfov);
    let h = f32::tan(theta * 0.5);
    let viewport_height = 2.0 * h;
    let viewport_width = aspect * viewport_height;

    let f = (eye - lookat).normalize();
    let r = float3::cross(up, f);
    let u = float3::cross(f, r);

    let horizontal = viewport_width * r;
    let vertical = viewport_height * u;

    Camera {
        origin: eye,
        forward: f,
        right: r,
        up: u,
        viewport_width,
        viewport_height,
        focal_length: 1.0,
        horizontal: viewport_width * r,
        vertical: viewport_height * u,
        corner: eye - horizontal * 0.5 - vertical * 0.5 - f,
    }
}

fn ray_color(r: ray, payload: &mut RayPayload, w: &Scene, depth: u32) {
    if depth <= 0 {
        payload.attenuation = float3 { x: 0.0, y: 0.0, z: 0.0 };
        return;
    };

    let mut hit: RayHit = RayHit { 
        t: 0.0, 
        point: float3{ x: 0.0, y: 0.0, z: 0.0 },
        normal: float3 {x: 0.0, y: 0.0, z: 0.0},
        object: None,
        front: false,
    };

    let max_distance: f32 = 100000.0;

    if w.hit(r, max_distance, &mut hit) {
        let hit_object = hit.object.as_ref().unwrap();
        let material = &hit_object.material;

        let mut scattered_ray = ray {
            origin: float3 { x: 0.0, y: 0.0, z: 0.0 },
            direction: float3 { x: 0.0, y: 0.0, z: 0.0 }
        };

        if material.scatter(r, &hit, payload, &mut scattered_ray)
        {
            ray_color(scattered_ray, payload, w, depth - 1);
            return;
        }
        else
        {
            payload.attenuation = float3 { x: 0.0, y: 0.0, z: 0.0 };
            return;
        }
    }
    else
    {
        let c1 = float3 { x: 1.0, y: 1.0, z: 1.0 };
        let c2 = float3 { x: 0.5, y: 0.7, z: 1.0 };

        let unit_direction = r.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        payload.attenuation = payload.attenuation * ((1.0 - t) * c1 + t * c2);
    }
}

fn color_to_u32(c: float3) -> u32 {
    let r: u32 = (c.x * 255.0) as u32;
    let g: u32 = (c.y * 255.0) as u32;
    let b: u32 = (c.z * 255.0) as u32;

    return r << 16 | g << 8 | b;
}

fn main() {
    let max_bounces: u32 = 15;

    let mut screen_buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut accum_buffer: Vec<float3> = vec![float3 { x: 0.0, y: 0.0, z: 0.0 }; WIDTH * HEIGHT];
    let mut accum_count: u32 = 0;

    let mut window = Window::new(
        "Pathtracer",
        WIDTH,
        HEIGHT,
        WindowOptions::default()
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let aspect: f32 = (WIDTH as f32) / (HEIGHT as f32);
    let mut camera = create_camera(
        float3 { x: -2.0, y: 2.0, z: 1.0 }, 
        float3 { x: 0.0, y: 0.0, z: -1.0 }, 
        float3 { x: 0.0, y: 1.0, z: 0.0 },
        90.0,
        aspect
    );

    let world: Scene = Scene {
        objects: vec![
            Rc::new(Sphere { // ground
                center: float3 { x: 0.0, y: -100.5, z: -1.0 },
                radius: 100.0,
                material: Box::new(LambertianMaterial {
                    albedo: float3 { x: 0.8, y: 0.8, z: 0.0 },
                    roughness: 0.0,
                })
            }),

            Rc::new(Sphere { // center
                center: float3 { x: 0.0, y: 0.0, z: -1.0 },
                radius: 0.5,
                material: Box::new(LambertianMaterial {
                    albedo: float3 { x: 1.0, y: 0.0, z: 0.0 },
                    roughness: 0.0,
                })
            }),

            Rc::new(Sphere { // left
                center: float3 { x: -1.0, y: 0.0, z: -1.0 },
                radius: 0.5,
                material: Box::new(DielectricMaterial { 
                    refraction_index: 1.5
                })                
            }),

            Rc::new(Sphere { // left
                center: float3 { x: -1.0, y: 0.0, z: -1.0 },
                radius: -0.4,
                material: Box::new(DielectricMaterial { 
                    refraction_index: 1.5
                })                
            }),

            Rc::new(Sphere {
                center: float3 { x: 1.0, y: 0.0, z: -1.0 },
                radius: 0.5,
                material: Box::new(MetallicMaterial {
                    albedo: float3 { x: 1.0, y: 1.0, z: 1.0 },
                    roughness: 0.2,
                })
            }),
        ]
    };

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // process input
        if window.is_key_down(Key::S) {
            camera.origin = camera.origin - camera.forward * 0.3;

            // reset accum buffer
            for j in 0..HEIGHT {
                for i in 0..WIDTH {
                    accum_buffer[j * WIDTH + i] = float3 { x: 0.0, y: 0.0, z: 0.0 };
                }
            }
            accum_count = 0;
        }

        for j in 0..HEIGHT {
            for i in 0..WIDTH {
                let mut color: float3 = float3 { x: 0.0, y: 0.0, z: 0.0 };

                // add some random variance to dispatched rays
                let u = ((i as f32) + random::random_f32()) / ((WIDTH-1) as f32);
                let v = 1.0 - ((j as f32) + random::random_f32()) / ((HEIGHT-1) as f32);

                let corner = camera.origin - (camera.right * camera.viewport_width) * 0.5 - (camera.up * camera.viewport_height) * 0.5 - camera.forward;

                let r = ray {
                    origin: camera.origin,
                    // direction: float3 {
                    //     x: corner.x + u * camera.viewport_width - camera.origin.x,
                    //     y: corner.y + v * camera.viewport_height - camera.origin.y,
                    //     z: corner.z - camera.origin.z
                    // }

                    direction: float3 {
                        x: corner.x + u * camera.horizontal.x + v * camera.vertical.x - camera.origin.x,
                        y: corner.y + u * camera.horizontal.y + v * camera.vertical.y - camera.origin.y,
                        z: corner.z + u * camera.horizontal.z + v * camera.vertical.z - camera.origin.z
                    }
                };

                let mut payload: RayPayload = RayPayload {
                    attenuation: float3 { x: 1.0, y: 1.0, z: 1.0 }
                };

                ray_color(r, &mut payload, &world, max_bounces);

                color = color + payload.attenuation;
    
                let prev_color: float3 = accum_buffer[j * WIDTH + i];
                accum_buffer[j * WIDTH + i] = prev_color + color;
            }
        }

        accum_count += 1;

        for j in 0..HEIGHT {
            for i in 0..WIDTH {
                let mut c: float3 = accum_buffer[j * WIDTH + i] * (1.0 / accum_count as f32);

                c.x = f32::sqrt(c.x);
                c.y = f32::sqrt(c.y);
                c.z = f32::sqrt(c.z);

                
                screen_buffer[j * WIDTH + i] = color_to_u32(c);
            }
        }        

        window
            .update_with_buffer(&screen_buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
