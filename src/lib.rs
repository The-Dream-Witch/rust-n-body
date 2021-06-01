use std::f64::consts::PI;
use std::ops;
use std::default::Default;

extern crate rand;

//const G: f64 = 6.67e11;
const DT: f64 = 0.1;

#[derive(Clone, Copy, Debug)]
pub struct Vec3D(pub f64, pub f64, pub f64);

#[derive(Clone, Debug, Copy)]
pub struct Body {
    pub pos: Vec3D,
    pub vel: Vec3D,
    pub mass: f64,
}

#[derive(Clone,Debug)]
pub struct Nbodies {
    pub bodies: Vec<Body>,
}

impl Vec3D {
    pub fn sum_sqrs(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    fn scalar(&self) -> f64 {
        let sumsqr = self.sum_sqrs();
        DT / (sumsqr * sumsqr.sqrt())
    }

    pub fn new() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
    
        Self(rng.gen_range(0.0..800.0), rng.gen_range(0.0..800.0), 0.0)
    }

}

impl Body {
    pub fn new() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        Self{pos: Vec3D::new(), vel: Vec3D::default(), mass: rng.gen_range(0.0..1.0)}
    }
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
        
        for i in 0..self.bodies.len() {
            for j in i+1..self.bodies.len() {
                interactions.push(self.bodies[i].pos-self.bodies[j].pos);
                mags.push(interactions[i].scalar());
            }
        }
    
        let mut k = 0;
    
        for i in 0..self.bodies.len() {
            let mass1 = self.bodies[i].mass;
            for j in i+1..self.bodies.len() {
    
                let mass2 = self.bodies[j].mass;
                
                self.bodies[i].vel -= interactions[k] * (mass2 * mags[k]);
                self.bodies[j].vel += interactions[k] * (mass1 * mags[k]);
                k +=1;
            }
        }
    
        for i in 0..self.bodies.len() {
            let vel = self.bodies[i].vel;
            self.bodies[i].pos += vel * DT;
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

/*pub fn distance(pos1: Vec3D, pos2: Vec3D) -> f64{
    let dx = pos1.0-pos2.0;
    let dy = pos1.1-pos2.1;
    let dz = pos1.2-pos2.2;

    (dx*dx+dy*dy+dz*dz).sqrt()
}*/