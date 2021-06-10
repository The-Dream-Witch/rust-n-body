use crate::body::*;
use crate::constants::*;
use crate::octtree::*;
use crate::vec3d::*;

pub const DT: f64 = 5.;

#[derive(Clone, Debug)]
///Struct which contains a vector to a set of bodies, as well as an octtree
pub struct Nbodies {
    pub bodies: Vec<Body>,
    pub tree: Vec<OctTree>,
}

impl Nbodies {
    ///Instantiates a new nbodies struct with a custom-defined number of bodies
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

    ///As the name says, this updates the bodies contained in the nbodies vector, and does so using the octtree.
    pub fn update_with_tree(&mut self) {
        self.tree.push(OctTree::new(XMAX, Vec3D::default()));

        for i in 0..self.bodies.len() - 1 {
            self.tree[0].find_subtree(self.bodies[i]);
        }

        for i in 0..self.bodies.len() - 1 {
            self.tree[0].update_body(&mut self.bodies[i]);
            self.bodies[i].bounds();
        }

        self.tree.clear();
    }

    ///Updates the values for the bodies using the brute-force, loop-based, algorithm.
    pub fn update_naive(&mut self) {
        if self.bodies.len() <= 1 {
            return;
        }

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

#[cfg(test)]
mod nbodies_tests {
    use crate::nbodies::*;

    #[test]
    ///Test created to check that the tree algorithm won't update a body using that body as a secondary body
    pub fn naive_self_test() {
        let mut nbodies = Nbodies::new(1);
        let original_body = nbodies.bodies[0].clone();

        nbodies.update_naive();
        assert!(nbodies.bodies[0] == original_body);
    }

    #[test]
    ///Test created to check that the tree algorithm won't update a body using that body as a secondary body
    pub fn tree_self_test() {
        let mut nbodies = Nbodies::new(1);
        let original_body = nbodies.bodies[0].clone();

        nbodies.update_with_tree();
        assert!(nbodies.bodies[0] == original_body);
    }
}
