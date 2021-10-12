use std::ops;
use std::fmt::{Display, Formatter};
use js_sys::Math::sqrt;
use wasm_bindgen::__rt::core::ops::{BitXor, Div};

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z}
    }

    pub fn add_mut(mut self, x: f64, y: f64, z: f64) -> Self {
        self.x += x;
        self.y += y;
        self.z += z;
        self
    }

    pub fn scale_mut(mut self, x: f64, y: f64, z: f64) -> Self {
        self.x *= x;
        self.y *= y;
        self.z *= z;
        self
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.x * other.z - self.z * other.x,
            self.z * other.x - self.x * other.z
        )
    }

    pub fn mag2(&self) -> f64 {
        self.dot(&self)
    }

    pub fn mag(&self) -> f64 {
       sqrt(self.mag2())
    }

    pub fn dist2(&self, other: &Vec3) -> f64 {
        let x = other.x - self.x;
        let y = other.y - self.y;
        let z = other.z - self.z;
        x*x + y*y + z*z
    }

    pub fn dist(&self, other: &Vec3) -> f64 {
        sqrt(self.dist2(other))
    }

    pub fn unit(&self) -> Vec3 {
        let m = self.mag2();
        if m == 1. || m == 0. {
            self.clone()
        } else {
            self / sqrt(m)
        }
    }

    pub fn on_axis(&self, axis: &Vec3) -> Vec3 {
        &(self.dot(axis) * axis) / axis.mag2()
    }

    pub fn off_axis(&self, axis: &Vec3) -> Vec3 {
        self - &self.on_axis(axis)
    }

    pub fn ix(&self) -> f64 {
        self.x as f64
    }

    pub fn iy(&self) -> f64 {
        self.y as f64
    }

    pub fn iz(&self) -> f64 {
        self.z as f64
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}, {}, {}>", self.x, self.y, self.z)
    }
}

impl From<&(f64, f64, f64)> for Vec3 {
    fn from(tup: &(f64, f64, f64)) -> Self {
        Vec3::new(tup.0, tup.1, tup.2)
    }
}

impl From<&(usize, usize, usize)> for Vec3 {
    fn from(tup: &(usize, usize, usize)) -> Self {
        Vec3::new(tup.0 as f64, tup.1 as f64, tup.2 as f64)
    }
}

impl ops::Add<&Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        self.clone().add_mut(rhs.x, rhs.y, rhs.z)
    }
}

impl ops::Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        rhs.clone().add_mut(-rhs.x, -rhs.y, -rhs.z)
    }
}

impl ops::Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        self.clone().scale_mut(rhs, rhs, rhs)
    }
}

impl ops::Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        rhs.clone().scale_mut(self, self, self)
    }
}

impl ops::Mul<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        self.clone().scale_mut(rhs.x, rhs.y, rhs.z)
    }
}

impl ops::Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self.clone().scale_mut(1./rhs, 1./rhs, 1./rhs)
    }
}

impl ops::Div<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: &Vec3) -> Self::Output {
        self.clone().scale_mut(1./rhs.x, 1./rhs.y, 1./rhs.z)
    }
}

impl ops::BitXor<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn bitxor(self, rhs: &Vec3) -> Self::Output {
        self.cross(rhs)
    }
}
