use std::ops;
use std::fmt::{Display, Formatter};
use js_sys::Math::sqrt;
use wasm_bindgen::__rt::core::ops::{BitXor, Div};

#[derive(Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z}
    }

    pub fn zero() -> Self {
        Self::new(0., 0., 0.)
    }

    pub fn set(mut self, x: f64, y: f64, z: f64) -> Self {
        self.x = x;
        self.y = y;
        self.z = z;
        self
    }

    pub fn add_mut(mut self, x: f64, y: f64, z: f64) -> Self {
        self.x += x;
        self.y += y;
        self.z += z;
        self
    }

    pub fn add_vec_mut(mut self, v: &Vec3) -> Self {
        self.add_mut(v.x, v.y, v.z)
    }

    pub fn sadd_vec_mut(mut self, scale: f64, v: &Vec3) -> Self {
        self.add_mut(scale * v.x, scale * v.y, scale * v.z)
    }

    pub fn scale_mut(mut self, x: f64, y: f64, z: f64) -> Self {
        self.x *= x;
        self.y *= y;
        self.z *= z;
        self
    }

    pub fn scale_uniform_mut(mut self, s: f64) -> Self {
        self.scale_mut(s, s, s)
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    // TODO unit test.
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

    pub fn rz90(&self) -> Vec3 {
        Vec3::new(-self.y, self.x, self.z)
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

    pub fn lerp(a: &Vec3, b: &Vec3, s: f64) -> Vec3 {
        a.clone()
            .scale_uniform_mut(1. - s)
            .sadd_vec_mut(s, b)
    }

    pub fn bezier2(a: &Vec3, b: &Vec3, c: &Vec3, s: f64) -> Vec3 {
        Self::lerp(
            &Self::lerp(a, b, s),
            &Self::lerp(b, c, s),
            s
        )
    }

    pub fn bezier3(a: &Vec3, b: &Vec3, c: &Vec3, d: &Vec3, s: f64) -> Vec3 {
        Self::lerp(
            &Self::bezier2(a, b, c, s),
            &Self::bezier2(b, c, d, s),
            s
        )
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
        self.clone().add_vec_mut(rhs)
    }
}

impl ops::Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        self.clone().sadd_vec_mut(-1., rhs)
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
        rhs.clone().scale_uniform_mut(self)
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
        self.clone().scale_uniform_mut(1./rhs)
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


#[cfg(test)]
mod tests {
    use crate::threed::*;

    #[test]
    fn subtraction() {
        assert_eq!("<1, 2, 3>", (&Vec3::new(4., 5., 6.) - &Vec3::new(3., 3., 3.)).to_string());
    }
}
