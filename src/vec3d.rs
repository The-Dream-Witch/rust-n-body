extern crate rand;

use std::default::Default;
use std::ops;

///Const values defining the maximum width, height, and depth
pub const XMAX: f64 = 1920.;
pub const YMAX: f64 = 1080.;
pub const ZMAX: f64 = 800.;

const DT: f64 = 1000.;

#[derive(Clone, Copy, Debug, PartialEq)]
///The Vec3D struct is used for position and velocity vectors within the rest of the program, and its many implementations allow operations to occur (+, *, +=, etc)
///between it and another Vec3D, or between it and an f64.
pub struct Vec3D(pub f64, pub f64, pub f64);

impl Vec3D {
    ///Returns the sum of the squares of each of the fields (X,Y, and Z)
    pub fn sum_sqrs(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }
    ///Returns an f64 which is DT (delta time) over the distance ()
    pub fn get_scalar(&self) -> f64 {
        let sumsqr = self.sum_sqrs();
        DT / ((sumsqr).sqrt())
    }
    ///Creates a new vec3d to be used as a position vector; its values are pseudo-randomly generated,
    ///and can range anywhere within the available screen real estate, as well as anywhere within the pre-defined depth of the z axis.
    pub fn new_pos() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        Self(
            rng.gen_range(0.0..XMAX),
            rng.gen_range(0.0..YMAX),
            rng.gen_range(0.0..ZMAX),
        )
    }
    ///Creates a new vec3d to be used as a velocity vector; its values are pseudo-randomly generated,
    ///and are constrained to a range between -0.5 and 0.5; anything faster than that and the simulation is too chaotic to watch.
    pub fn new_vel() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        Self(
            rng.gen_range(-0.5..0.5),
            rng.gen_range(-0.5..0.5),
            rng.gen_range(-0.5..0.5),
        )
    }
    ///Allows creation of a custom-defined Vec3D
    pub fn new_with_tuple((x, y, z): (f64, f64, f64)) -> Self {
        Self(x, y, z)
    }
}

///Default implementation for Vec3D; sets all values to zero.
impl Default for Vec3D {
    fn default() -> Vec3D {
        Vec3D(0., 0., 0.)
    }
}

///Implementation allowing for the addition of two Vec3Ds
impl ops::Add for Vec3D {
    type Output = Vec3D;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3D(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

///Implementation allowing for the addition of an f64 across all values of a Vec3D
impl ops::Add<f64> for Vec3D {
    type Output = Vec3D;
    fn add(self, rhs: f64) -> Self::Output {
        Vec3D(self.0 + rhs, self.1 + rhs, self.2 + rhs)
    }
}

///Implementation allowing for the subtraction of two Vec3Ds
impl ops::Sub for Vec3D {
    type Output = Vec3D;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3D(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

///Implementation allowing for the multiplication of an f64 across all values of a Vec3D
impl ops::Mul<f64> for Vec3D {
    type Output = Vec3D;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3D(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

///Implementation allowing for the use of the += operator between two Vec3Ds
impl ops::AddAssign for Vec3D {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

///Implementation allowing for the use of the -= operator between two Vec3Ds
impl ops::SubAssign for Vec3D {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}
///Implementation allowing for the use of the * operator between two Vec3Ds
impl ops::Mul<Vec3D> for Vec3D {
    type Output = Vec3D;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3D(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}
