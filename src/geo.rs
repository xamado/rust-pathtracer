use crate::math::{float3, ray};

#[derive(Debug, Copy, Clone)]
pub struct ray_hit {
    pub t: f32,
    pub point: float3,
    pub normal: float3,
}

pub struct sphere {
    pub center: float3,
    pub radius: f32,
}

pub struct scene {
    pub objects: Vec<sphere>
}

impl scene {
    pub fn hit(&self, r: ray, max_distance: f32, closest_hit: &mut ray_hit) -> bool {
        let mut ray_hit_something: bool = false;
        let mut min_t: f32 = max_distance;

        let mut hit: ray_hit = ray_hit { 
            t: 0.0, 
            point: float3{ x: 0.0, y: 0.0, z: 0.0 },
            normal: float3 {x: 0.0, y: 0.0, z: 0.0},
        };

        for o in &self.objects {
            if o.hit(r, min_t, &mut hit) {
                ray_hit_something = true;

                if hit.t < min_t {
                    min_t = hit.t;
                    *closest_hit = hit;
                }
            }
        }

        return ray_hit_something;
    }
}

impl sphere {
    pub fn hit(&self, r: ray, max_distance: f32, hit: &mut ray_hit) -> bool {
        let min_distance: f32 = 0.001;

        let oc: float3 = r.origin - self.center;
        let a:f32 = r.direction.sqrLength();
        let half_b: f32 = float3::dot(oc, r.direction);
        let c: f32 = oc.sqrLength() - self.radius * self.radius;
        let d: f32 = half_b*half_b - a*c;

        if d < min_distance {
            return false;
        }
        
        // find the nearest root
        let sqrtd = f32::sqrt(d);
        let mut root: f32 = (-half_b - sqrtd) / a;
        if root < min_distance || max_distance< root {
            root = (-half_b + sqrtd) / a;
            if root < min_distance || max_distance < root {
                return false;
            }
        }

        let t: f32 = (-half_b - f32::sqrt(d)) / a;

        (*hit).t = t;
        (*hit).point = r.origin + r.direction * t;
        (*hit).normal = (hit.point - self.center) * (1.0 / self.radius);

        return true;
    }
}
