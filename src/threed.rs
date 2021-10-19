use std::ops;
use std::fmt::{Display, Formatter};
use js_sys::Math::sqrt;
use wasm_bindgen::__rt::core::ops::{BitXor, Div};

#[derive(Clone, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self::new(0., 0., 0.)
    }

    pub fn right() -> Self {
        Self::new(1., 0., 0.)
    }

    pub fn up() -> Self {
        Self::new(0., 1., 0.)
    }

    pub fn forward() -> Self {
        Self::new(0., 0., 1.)
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

    pub fn is_zero(&self, eps: f64) -> bool {
        self.x.abs() < eps && self.y.abs() < eps && self.z.abs() < eps
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn flipped(&self) -> Self {
        Vec3::new(-self.x, -self.y, -self.z)
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        let (a1, a2, a3) = (self.x, self.y, self.z);
        let (b1, b2, b3) = (other.x, other.y, other.z);
        Vec3::new(
            a2 * b3 - a3 * b2,
            a3 * b1 - a1 * b3,
            a1 * b2 - a2 * b1,
        )
    }

    pub fn norm(&self, other: &Vec3) -> Vec3 {
        let mut cross = self.cross(other);
        let mag = cross.mag();
        if mag == 0. || mag == 1. {
            return cross;
        }
        cross.scale_uniform_mut(1. / mag)
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
        x * x + y * y + z * z
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
            s,
        )
    }

    pub fn bezier3(a: &Vec3, b: &Vec3, c: &Vec3, d: &Vec3, s: f64) -> Vec3 {
        Self::lerp(
            &Self::bezier2(a, b, c, s),
            &Self::bezier2(b, c, d, s),
            s,
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

impl ops::Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        self.clone().add_vec_mut(rhs)
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

impl ops::Mul<f64> for &Vec3 {
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
        self.clone().scale_uniform_mut(1. / rhs)
    }
}

impl ops::Div<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: &Vec3) -> Self::Output {
        self.clone().scale_mut(1. / rhs.x, 1. / rhs.y, 1. / rhs.z)
    }
}

impl ops::BitXor<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn bitxor(self, rhs: &Vec3) -> Self::Output {
        self.cross(rhs)
    }
}

#[derive(Clone)]
pub struct Basis {
    axes: Vec<Vec3>,
}

impl Basis {
    pub fn new(x: Vec3, y: Vec3, z: Vec3) -> Self {
        Self {
            axes: vec![x, y, z],
        }
    }

    pub fn identity() -> Self {
        Self::new(Vec3::right(), Vec3::up(), Vec3::forward())
    }

    pub fn project(&self, local_vec: &Vec3) -> Vec3 {
        Vec3::zero()
            .sadd_vec_mut(local_vec.x, &self.axes[0])
            .sadd_vec_mut(local_vec.y, &self.axes[1])
            .sadd_vec_mut(local_vec.z, &self.axes[2])
    }

    pub fn unproject(&self, global_vec: &Vec3) -> Vec3 {
        Vec3::new(
            global_vec.dot(&self.axes[0]) / self.axes[0].mag2(),
            global_vec.dot(&self.axes[1]) / self.axes[1].mag2(),
            global_vec.dot(&self.axes[2]) / self.axes[2].mag2()
        )
    }
}

#[derive(Clone)]
pub struct Frame {
    origin: Vec3,
    basis: Basis,
}

impl Frame {
    pub fn new(origin: Vec3, x: Vec3, y: Vec3, z: Vec3) -> Self {
        Self::from_basis(origin, Basis::new(x, y, z))
    }

    pub fn from_basis(origin: Vec3, basis: Basis) -> Self {
        Self {
            origin,
            basis
        }
    }

    pub fn identity() -> Self {
        Self::from_basis(Vec3::zero(), Basis::identity())
    }

    pub fn project(&self, local_point: &Vec3) -> Vec3 {
        self.basis.project(local_point).add_vec_mut(&self.origin)
    }

    pub fn unproject(&self, global_point: &Vec3) -> Vec3 {
        self.basis.unproject(&(global_point - &self.origin))
    }
}

#[derive(Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction: direction.unit() }
    }

    pub fn sample(&self, s: f64) -> Vec3 {
        self.origin.clone().sadd_vec_mut(s, &self.direction)
    }
}

impl Display for Ray {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ray({} -> {})", self.origin, self.direction)
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
