extern crate rand;

use std::default::Default;
use std::ops;

const G: f64 = 6.67e-11;
const DT: f64 = 1000.;

pub const XMAX: f64 = 1920.;
pub const YMAX: f64 = 1080.;
pub const ZMAX: f64 = 800.;

pub const SOFTENING: f64 = 500.;
pub const THETA: f64 = 0.5;

#[derive(Clone, Debug)]
pub struct Nbodies {
    pub bodies: Vec<Body>,
    pub tree: Vec<OctTree>,
}

impl Nbodies {
    pub fn new(n: u32) -> Self {
        let mut newbodies: Vec<Body> = Vec::new();

        for _ in 0..n {

            newbodies.push(Body::new());
        }

        Self {
            bodies: newbodies,
            tree: Vec::with_capacity(1),
        }
    }

    pub fn update_with_tree(&mut self) {
        self.tree.push(OctTree::new(XMAX, Vec3D::default()));

        for i in 0..self.bodies.len() - 1 {
            self.tree[0].find_subtree(self.bodies[i].clone());
        }

        for i in 0..self.bodies.len() - 1 {
            self.tree[0].update_body(&mut self.bodies[i]);
            self.bodies[i].bounds();
        }

        self.tree.clear();
    }

    pub fn update_naive(&mut self) {
        let mut delta_pos: Vec<Vec3D> = Vec::new();
        let mut mags: Vec<f64> = Vec::new();
        let mut k = 0;

        for i in 0..self.bodies.len() - 1 {
            let mass1 = self.bodies[i].mass;
            for j in i + 1..self.bodies.len() {
                let mass2 = self.bodies[j].mass;

                //Calculate and store differences between particle positions
                delta_pos.push(self.bodies[i].pos - self.bodies[j].pos);

                //Calculate and store magnitudes
                mags.push(delta_pos[i].get_scalar());

                //Calculate and apply new velocities
                self.bodies[i].vel -= delta_pos[k] * (mass2 * mags[k]) * G;
                self.bodies[j].vel += delta_pos[k] * (mass1 * mags[k]) * G;
                k += 1;
            }
        }

        //Compute new positions
        for i in 0..self.bodies.len() {
            let vel = self.bodies[i].vel;
            self.bodies[i].pos += vel * DT;
            self.bodies[i].bounds();
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3D(pub f64, pub f64, pub f64);

impl Vec3D {
    pub fn sum_sqrs(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    fn get_scalar(&self) -> f64 {
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

/*impl PartialEq for Vec3D {
    fn eq(&self, rhs: &Self) -> bool {
        self.0 == rhs.0 && self.1 == rhs.1 && self.2 == rhs.2
    }
}*/

impl ops::Mul<Vec3D> for Vec3D {
    type Output = Vec3D;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3D(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Body {
    pub pos: Vec3D,
    pub vel: Vec3D,
    pub mass: f64,
}

impl Body {
    pub fn new() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        Self {
            pos: Vec3D::new(),
            vel: Vec3D::default(),
            mass: rng.gen_range(1..100) as f64,
        }
        
    }

    pub fn equal(&self, rhs: &Self) -> bool {
        self.pos == rhs.pos && self.vel == rhs.vel && self.mass == rhs.mass
    }

    pub fn bounds(&mut self) {
        if self.pos.0 < 0. {
            self.pos.0 = 0.;
            self.vel.0 = self.vel.0 * -1.;
        } else if self.pos.0 > XMAX {
            self.pos.0 = XMAX;
            self.vel.0 = self.vel.0 * -1.;
        }

        if self.pos.1 < 0. {
            self.pos.1 = 0.;
            self.vel.1 = self.vel.1 * -1.;
        } else if self.pos.1 > YMAX {
            self.pos.1 = YMAX;
            self.vel.1 = self.vel.1 * -1.;
        }

        if self.pos.2 < 0. {
            self.pos.2 = 0.;
            self.vel.2 = self.vel.2 * -1.;
        } else if self.pos.2 > ZMAX {
            self.pos.2 = ZMAX;
            self.vel.2 = self.vel.2 * -1.;
        }
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
    pub num_bodies: u32,
}

impl OctTree {
    pub fn new(width: f64, center: Vec3D) -> Self {
        Self {
            current_body: Vec::with_capacity(1),
            sub_trees: Vec::with_capacity(8),
            center,
            center_mass: center,
            total_mass: 0.0,
            width,
            num_bodies: 0,
        }
    }

    pub fn add_body(&mut self, body: Body) {
        self.center_mass = (self.center_mass * self.total_mass + body.pos * body.mass)
            * (1. / (self.total_mass + body.mass));
        self.total_mass += body.mass;

        self.num_bodies += 1;

        if self.num_bodies == 1 {
            self.current_body.push(body);
        } else {
            self.find_subtree(body);

            if self.num_bodies == 2 {
                let body_to_find = self.current_body.pop().unwrap();
                self.find_subtree(body_to_find);
            }
        }
    }

    pub fn find_subtree(&mut self, body: Body) {
        if self.width / 2. < 1. {
            return;
        }

        let coord_mod = [
            (1., 1., 1.),
            (1., 1., -1.),
            (1., -1., 1.),
            (1., -1., -1.),
            (-1., 1., 1.),
            (-1., 1., -1.),
            (-1., -1., 1.),
            (-1., -1., -1.),
        ]
        .to_vec();

        if self.sub_trees.is_empty() {
            for modifier in coord_mod {
                let mut sub_tree_center = Vec3D::new_with_tuple(modifier);
                sub_tree_center = sub_tree_center * (self.center + (self.width / 4.));

                self.sub_trees
                    .push(OctTree::new(self.width / 2., sub_tree_center));
            }
        }

        for i in 0..8 {
            if f64::abs(self.sub_trees[i].center.0 - body.pos.0) <= self.sub_trees[i].width
                && f64::abs(self.sub_trees[i].center.1 - body.pos.1)
                    <= self.sub_trees[i].width
                && f64::abs(self.sub_trees[i].center.2 - body.pos.2)
                    <= self.sub_trees[i].width
            {
                self.sub_trees[i].add_body(body);
            }
        }
    }

    pub fn update_body(&self, body: &mut Body) {
        let delta_pos = self.center_mass - body.pos;
        let distance_squared = delta_pos.sum_sqrs();
        if (self.num_bodies == 1 && *body != self.current_body[0])
            || ((self.width * self.width) < (THETA * THETA * distance_squared))
        {
            let mag = delta_pos.get_scalar();

            body.vel += delta_pos * self.total_mass * mag * G;
            body.pos += body.vel;

        } else if !self.sub_trees.is_empty() {
            for sub_tree in &self.sub_trees {
                if sub_tree.num_bodies != 0 {
                    sub_tree.update_body(body);
                }
            }
        }
    }
}