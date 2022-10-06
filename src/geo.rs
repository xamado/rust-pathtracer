use crate::math::{float3, ray};
use crate::random;

use std::rc::Rc;

#[derive(Clone)]
pub struct RayHit {
    pub t: f32,
    pub point: float3,
    pub normal: float3,
    pub object: Option<Rc<Sphere>>,
    pub front: bool,
}

pub struct RayPayload {
    pub attenuation: float3,
}

pub struct Sphere {
    pub center: float3,
    pub radius: f32,
    pub material: Box<dyn Material>
}

pub struct Scene {
    pub objects: Vec<Rc<Sphere>>
}

pub trait Material
{
    fn scatter(&self, ray_in: ray, hit: &RayHit, payload: &mut RayPayload, ray_out: &mut ray) -> bool;
}

#[derive(Copy, Clone)]
pub struct LambertianMaterial {
    pub albedo: float3,
    pub roughness: f32,
}

pub struct MetallicMaterial {
    pub albedo: float3,
    pub roughness: f32,
}

pub struct DielectricMaterial {
    pub refraction_index: f32,
}

impl DielectricMaterial
{
    fn refract(dir: float3, normal: float3, ir: f32) -> float3
    {
        let cos_theta: f32 = f32::min(float3::dot(dir * -1.0, normal), 1.0);
        let perpendicular: float3 = ir * (dir + normal * cos_theta);
        let parallel: float3 = -f32::sqrt(f32::abs(1.0 - perpendicular.sqrLength())) * normal;
        return perpendicular + parallel;
    }

    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        // schlick's fresnel
        let mut r0: f32 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        return r0 + (1.0 - r0) * f32::powi(1.0 - cosine, 5);
    }
}

impl Material for LambertianMaterial
{
    fn scatter(&self, _ray_in: ray, hit: &RayHit, payload: &mut RayPayload, ray_out: &mut ray) -> bool
    {
        *ray_out = ray {
            origin: hit.point,
            direction: hit.normal + random::random_point_in_unit_hemisphere(hit.normal)
        };
        
        payload.attenuation = payload.attenuation * self.albedo;
        
        return true;
    }
}

impl Material for MetallicMaterial
{
    fn scatter(&self, r: ray, hit: &RayHit, payload: &mut RayPayload, ray_out: &mut ray) -> bool
    {
        let reflected: float3 = float3::reflect(r.direction, hit.normal);

        ray_out.origin = hit.point;
        ray_out.direction = reflected + self.roughness * random::random_point_in_unit_sphere();

        payload.attenuation = payload.attenuation * self.albedo;

        return float3::dot(ray_out.direction, hit.normal) > 0.0;
    }
}

impl Material for DielectricMaterial 
{
    fn scatter(&self, r: ray, hit: &RayHit, payload: &mut RayPayload, ray_out: &mut ray) -> bool
    {
        let refraction: f32 = if hit.front { (1.0 / self.refraction_index) } else { self.refraction_index };

        let mut dir: float3 = r.direction.normalize();

        let cos_theta: f32 = f32::min(float3::dot(dir * -1.0, hit.normal), 1.0);
        let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract: bool = refraction * sin_theta > 1.0;

        if cannot_refract || DielectricMaterial::reflectance(cos_theta, refraction) > random::random_f32() {
            dir = float3::reflect(r.direction, hit.normal);
        }
        else {
            dir = DielectricMaterial::refract(dir, hit.normal, refraction);
        }

        ray_out.origin = hit.point;
        ray_out.direction = dir;
        
        return true;
    }
}


impl Scene {
    pub fn hit(&self, r: ray, max_distance: f32, closest_hit: &mut RayHit) -> bool {
        let mut ray_hit_something: bool = false;
        let mut min_t: f32 = max_distance;

        for o in &self.objects {
            let mut hit: RayHit = RayHit { 
                t: 0.0, 
                point: float3{ x: 0.0, y: 0.0, z: 0.0 },
                normal: float3 {x: 0.0, y: 0.0, z: 0.0},
                object: None,
                front: false,
            };

            if o.hit(r, min_t, &mut hit) {
                ray_hit_something = true;

                if hit.t < min_t {
                    min_t = hit.t;
                    *closest_hit = hit;

                    (*closest_hit).object = Some(Rc::clone(o));
                }
            }
        }

        return ray_hit_something;
    }
}

impl Sphere {
    pub fn hit(&self, r: ray, max_distance: f32, hit: &mut RayHit) -> bool {
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

        let point = r.origin + r.direction * root;
        let normal: float3 = (point - self.center) * (1.0 / self.radius);

        let front: bool = float3::dot(r.direction, normal) < 0.0;

        (*hit).t = root;
        (*hit).point = point;
        (*hit).normal = if front == true { normal } else { normal * -1.0 };
        (*hit).front = front;

        return true;
    }
}
