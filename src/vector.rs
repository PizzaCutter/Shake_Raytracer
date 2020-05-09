use smath;
use std::ops;

#[derive(Copy, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn Zero() -> Vector3 {
        return Vector3 { x: 0.0, y: 0.0, z: 0.0 };
    }

    pub fn Magnitude(&self) -> f32
    {
        return smath::smath::powf(self.x, 2) + smath::smath::powf(self.y, 2) + smath::smath::powf(self.z, 2);
        return smath::smath::sqrt_fast(smath::smath::powf(self.x, 2) + smath::smath::powf(self.y, 2) + smath::smath::powf(self.z, 2));
    }

    pub fn Normalize(&mut self) 
    {
        let magnitude:f32 = self.Magnitude();
        self.x /= magnitude;
        self.y /= magnitude;
        self.z /= magnitude;
    }

    pub fn Dot(lhs: Vector3, rhs: Vector3) -> f32
    {
        return (lhs.x * rhs.x) + (lhs.y * rhs.y) * (lhs.z * rhs.z);
    }
}

impl ops::Add<Vector3> for Vector3
{
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Vector3
    {
        let outVec = Vector3{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };

        return outVec;
    }
}

impl ops::Sub<Vector3> for Vector3
{
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Vector3
    {
        let outVec = Vector3{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };

        return outVec;
    }
}
