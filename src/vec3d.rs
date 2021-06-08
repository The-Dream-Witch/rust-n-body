extern crate rand;

use std::default::Default;
use std::ops;

pub const XMAX: f64 = 1920.;
pub const YMAX: f64 = 1080.;
pub const ZMAX: f64 = 800.;

const DT: f64 = 1000.;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3D(pub f64, pub f64, pub f64);

impl Vec3D {
    pub fn sum_sqrs(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn get_scalar(&self) -> f64 {
        let sumsqr = self.sum_sqrs();
        DT / ((sumsqr).sqrt())
    }

    pub fn new() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        Self(
            rng.gen_range(0.0..XMAX),
            rng.gen_range(0.0..YMAX),
            rng.gen_range(0.0..ZMAX),
        )
    }

    pub fn new_with_tuple((x, y, z): (f64, f64, f64)) -> Self {
        Self(x, y, z)
    }
}

impl Default for Vec3D {
    fn default() -> Vec3D {
        Vec3D(0., 0., 0.)
    }
}

impl ops::Add for Vec3D {
    type Output = Vec3D;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3D(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::Add<f64> for Vec3D {
    type Output = Vec3D;
    fn add(self, rhs: f64) -> Self::Output {
        Vec3D(self.0 + rhs, self.1 + rhs, self.2 + rhs)
    }
}

impl ops::Sub for Vec3D {
    type Output = Vec3D;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3D(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::Mul<f64> for Vec3D {
    type Output = Vec3D;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3D(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl ops::AddAssign for Vec3D {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl ops::SubAssign for Vec3D {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl ops::Mul<Vec3D> for Vec3D {
    type Output = Vec3D;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3D(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}
