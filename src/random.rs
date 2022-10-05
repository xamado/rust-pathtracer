
use rand::Rng;
use crate::math::float3;

pub fn random_f32() -> f32
{
    let mut rng = rand::thread_rng();
    rng.gen::<f32>()
}

pub fn random_point_in_unit_sphere() -> float3
{
    let mut rng = rand::thread_rng();

    loop {
        let p: float3 = float3 {
            x: rng.gen_range(-1.0..1.0),
            y: rng.gen_range(-1.0..1.0),
            z: rng.gen_range(-1.0..1.0),
        };

        if p.sqrLength() < 1.0 {
            return p;
        }
    }
}

pub fn random_point_in_unit_hemisphere(normal: float3) -> float3
{
    let p: float3 = random_unit_vector();
    if float3::dot(p, normal) > 0.0 {
        return p;
    }
    else {
        return p * -1.0;
    }
}

pub fn random_unit_vector() -> float3
{
    return random_point_in_unit_sphere().normalize();   
}

