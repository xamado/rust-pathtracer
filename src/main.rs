extern crate minifb;

mod math;
mod geo;
mod random;

use minifb::{Key, Window, WindowOptions };
use math::{float3, ray};
use geo::{sphere, ray_hit, scene};
use random::*;

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

fn ray_color(r: ray, w: &scene, depth: u32) -> float3 {
    if depth <= 0 {
        return float3 { x: 0.0, y: 0.0, z: 0.0 }
    };

    let mut hit: ray_hit = ray_hit { 
        t: 0.0, 
        point: float3{ x: 0.0, y: 0.0, z: 0.0 },
        normal: float3 {x: 0.0, y: 0.0, z: 0.0},
    };

    let max_distance: f32 = 100000.0;

    if w.hit(r, max_distance, &mut hit) {
        let target = hit.point + hit.normal + random_point_in_unit_hemisphere(hit.normal);

        let child_ray = ray {
            origin: hit.point, 
            direction: target - hit.point
        };

        return 0.5 * ray_color(child_ray, w, depth - 1);

        // return 0.5 * (hit.normal + float3 { x: 1.0, y: 1.0, z: 1.0 });
    }

    let c1 = float3 { x: 1.0, y: 1.0, z: 1.0 };
    let c2 = float3 { x: 0.5, y: 0.7, z: 1.0 };

    let unit_direction = r.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * c1 + t * c2;
}

fn color_to_u32(c: float3) -> u32 {
    let r: u32 = (c.x * 255.0) as u32;
    let g: u32 = (c.y * 255.0) as u32;
    let b: u32 = (c.z * 255.0) as u32;

    return r << 16 | g << 8 | b;
}

fn main() {
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
    let viewport_height: f32 = 2.0;
    let viewport_width: f32 = aspect * viewport_height;
    let focalLength: f32 = 1.0;
    let max_bounces: u32 = 5;
    let samples_per_pixel: u32 = 10;

    let origin: float3 = float3 { x: 0.0, y: 0.0, z: 0.0 };

    let corner = float3 {
        x: origin.x - viewport_width * 0.5,
        y: origin.y - viewport_height * 0.5,
        z: origin.z - focalLength
    };

    let world: scene = scene {
        objects: vec![
            sphere {
                center: float3 { x: 0.0, y: 0.0, z: -1.0 },
                radius: 0.5
            },
            sphere {
                center: float3 { x: 0.0, y: -100.5, z: -1.0 },
                radius: 100.0
            },
        ]
    };        


    while window.is_open() && !window.is_key_down(Key::Escape) {
        for j in 0..HEIGHT {
            for i in 0..WIDTH {
                let mut color: float3 = float3 { x: 0.0, y: 0.0, z: 0.0 };

                //for _ in 0..samples_per_pixel {
                    let u = ((i as f32) + random::random_f32()) / ((WIDTH-1) as f32);
                    let v = 1.0 - ((j as f32) + random::random_f32()) / ((HEIGHT-1) as f32);

                    let r = ray {
                        origin: origin,
                        direction: float3 {
                            x: corner.x + u * viewport_width - origin.x,
                            y: corner.y + v * viewport_height - origin.y,
                            z: corner.z - origin.z
                        }
                    };

                    color = color + ray_color(r, &world, max_bounces);
                //}           
    
                let prev_color: float3 = accum_buffer[j * WIDTH + i];
                // let frame_color: u32 = color_to_u32(color * (1.0 / (samples_per_pixel as f32)));
                // let frame_color: float3 = color as f64;

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
