extern crate rand;

use std::ops;
use std::default::Default;


//const G: f64 = 6.67e11;
const DT: f64 = 0.01;

#[derive(Clone,Debug)]
pub struct Nbodies {
    pub bodies: Vec<Body>,
}

impl Nbodies {
    pub fn new(n: u32) -> Self {
        let mut newbodies: Vec<Body> = Vec::new();

        for _ in 0..n {
            let newbody = Body::new();
            newbodies.push(newbody);
        }
        Self {bodies: newbodies}
    }
    
    pub fn next(&mut self) {
        let mut interactions: Vec<Vec3D> = Vec::new();
        let mut mags: Vec<f64> = Vec::new();
        let mut k = 0;

        for i in 0..self.bodies.len() {
            let mass1 = self.bodies[i].mass;
            for j in i+1..self.bodies.len() {
                let mass2 = self.bodies[j].mass;

                //Calculate and store differences between particle positions
                interactions.push(self.bodies[i].pos-self.bodies[j].pos);
                
                //Calculate and store magnitudes
                mags.push(interactions[i].get_scalar());
                
                //Calculate and apply new velocities
                self.bodies[i].vel -= interactions[k] * (mass2 * mags[k]);
                self.bodies[j].vel += interactions[k] * (mass1 * mags[k]);
                k +=1;
            }
        }
    
        //Compute new positions
        for i in 0..self.bodies.len() {
            let vel = self.bodies[i].vel;
            self.bodies[i].pos += vel * DT;
            self.bodies[i].pos.bounds();
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Vec3D(pub f64, pub f64, pub f64);

impl Vec3D {
    pub fn sum_sqrs(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    fn get_scalar(&self) -> f64 {
        let sumsqr = self.sum_sqrs();
        DT / ((sumsqr * sumsqr).sqrt())
    }

    pub fn new() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
    
        Self(rng.gen_range(0.0..800.0), rng.gen_range(0.0..800.0), rng.gen_range(0.0..800.0))
    }

    pub fn bounds(&mut self) {
        if self.0 < 0. {
           self.0 = 800.;
        } else if self.0 > 800. {
            self.0 = 0.;
        }

        if self.1 < 0. {
            self.1 = 800.;
        } else if self.1 > 800. {
            self.1 = 0.;
        }

        if self.2 < 0. {
            self.2 = 800.;
        } else if self.2 > 800. {
            self.2 = 0.;
        }
    }
}

impl Default for Vec3D {
    fn default() -> Vec3D {
        Vec3D(0.,0.,0.)
    }
}

impl ops::Add for &Vec3D {
    type Output = Vec3D;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3D(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
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

#[derive(Clone, Debug, Copy)]
pub struct Body {
    pub pos: Vec3D,
    pub vel: Vec3D,
    pub mass: f64,
}

impl Body {
    pub fn new() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        Self{pos: Vec3D::new(), vel: Vec3D::default(), mass: rng.gen_range(1..10) as f64}
    }
}

impl Default for Body {
    fn default() -> Self {
        Self::new()
    }
}

