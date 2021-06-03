extern crate rand;

use std::ops;
use std::default::Default;


const G: f64 = 6.67e-11;
const DT: f64 = 10000.;

pub const XMAX: f64 = 800.;
pub const YMAX: f64 = 800.;
pub const ZMAX: f64 = 800.;

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

        for i in 0..self.bodies.len()-1 {
            let mass1 = self.bodies[i].mass;
            for j in i+1..self.bodies.len() {
                let mass2 = self.bodies[j].mass;

                //Calculate and store differences between particle positions
                interactions.push(self.bodies[i].pos-self.bodies[j].pos);
                
                //Calculate and store magnitudes
                mags.push(interactions[i].get_scalar());
                
                //Calculate and apply new velocities
                self.bodies[i].vel -= interactions[k] * (mass2 * mags[k]) * G;
                self.bodies[j].vel += interactions[k] * (mass1 * mags[k]) * G;
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

    pub fn new_with_tuple((x,y,z): (f64,f64,f64)) -> Self {
        Self(x,y,z)
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

/////////////////Under Construction//////////////////////



#[derive(Clone, Debug, Default)]
pub struct OctTree {
    pub current_body: Vec<Body>,
    pub sub_trees: Vec<OctTree>,

    pub center: Vec3D,
    pub center_mass: Vec3D,
    
    pub total_mass: f64,
    pub width: f64,
    pub num_bodies: f64,
}

impl OctTree {
    pub fn new(width: f64, center: Vec3D) -> Self {     
        Self {current_body: Vec::with_capacity(1), sub_trees: Vec::with_capacity(8), center, center_mass: center, total_mass: 0.0, width, num_bodies: 0.}
    }

    pub fn add_body(&mut self, _body: Body) {
        
    }

    pub fn find_subtree(&mut self, body: Body) {
        let coord_mod = [(1.,1.,1.),(1.,1.,-1.),(1.,-1.,1.),(1.,-1.,-1.),(-1.,1.,1.),(-1.,1.,-1.),(-1.,-1.,1.),(-1.,-1.,-1.)].to_vec();
        
        if self.sub_trees.is_empty() {
            for modifier in coord_mod {
                let mut sub_tree_center = Vec3D::new_with_tuple(modifier);
                sub_tree_center = sub_tree_center*(self.center+(self.width/4.));
                
                self.sub_trees.push(OctTree::new(self.width / 2., sub_tree_center));
            }
        }

        for i in 0..8 {
            if f64::abs(self.sub_trees[i].center.0 - body.pos.0) <= (self.sub_trees[i].width / 2.)
            && f64::abs(self.sub_trees[i].center.1 - body.pos.1) <= (self.sub_trees[i].width / 2.)
            && f64::abs(self.sub_trees[i].center.2 - body.pos.2) <= (self.sub_trees[i].width / 2.) {
                self.sub_trees[i].add_body(body);
            } 
        }
    }
}