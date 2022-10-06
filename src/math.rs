use std::ops;

#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct float3 {
    pub x: f32,
    pub y: f32,
    pub z: f32

}

#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct ray {
    pub origin: float3,
    pub direction: float3,
}

impl float3 {
    pub fn normalize(self) -> float3 {
        let l = self.length();
        return float3 {
            x: self.x / l,
            y: self.y / l,
            z: self.z / l
        };
    }

    pub fn length(self) -> f32 {
        return f32::sqrt(self.x*self.x + self.y*self.y + self.z*self.z);
    }

    pub fn sqrLength(self) -> f32 {
        return self.x*self.x + self.y*self.y + self.z*self.z;
    }

    pub fn dot(a: float3, b: float3) -> f32 {
        return a.x * b.x + a.y * b.y + a.z * b.z;
    }

    pub fn cross(u: float3, v: float3) -> float3 {
        float3 {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x
        }
    }

    pub fn reflect(v: float3, n: float3) -> float3 {
        return v - 2.0 * float3::dot(v, n) * n;
    }
}

impl ops::Mul<float3> for f32 {
    type Output = float3;

    fn mul(self, _rhs: float3) -> float3 {
        float3 {
            x: self * _rhs.x,
            y: self * _rhs.y,
            z: self * _rhs.z,
        }
    }
}

impl ops::Mul<f32> for float3 {
    type Output = float3;

    fn mul(self, _rhs: f32) -> float3 {
        float3 {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
        }
    }
}

impl ops::Mul<float3> for float3 {
    type Output = float3;

    fn mul(self, rhs: float3) -> float3 {
        float3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Add<float3> for float3 {
    type Output = float3;

    fn add(self, rhs: float3) -> float3 {
        float3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl ops::Sub<float3> for float3 {
    type Output = float3;

    fn sub(self, rhs: float3) -> float3 {
        float3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl ray {

}
