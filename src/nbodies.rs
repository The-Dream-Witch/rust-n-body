use crate::body::*;
use crate::octtree::*;
use crate::vec3d::*;

pub const XMAX: f64 = 1920.;
pub const YMAX: f64 = 1080.;
pub const ZMAX: f64 = 800.;

const G: f64 = 6.67e-11;
const DT: f64 = 1000.;

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