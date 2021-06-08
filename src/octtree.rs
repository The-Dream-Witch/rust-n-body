use crate::body::*;
use crate::vec3d::*;

pub const XMAX: f64 = 1920.;
pub const YMAX: f64 = 1080.;
pub const ZMAX: f64 = 800.;

const G: f64 = 6.67e-11;
const THETA: f64 = 0.5;

#[derive(Clone, Debug, Default)]
///This struct is utilized as the basis for all OctTrees, and is also used as its children
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
    ///Creates a new OctTree based on the provided width and center values
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

    ///Adds a new body to a given octtree node
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

    ///Finds the appropriate subtree for a given body
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
                && f64::abs(self.sub_trees[i].center.1 - body.pos.1) <= self.sub_trees[i].width
                && f64::abs(self.sub_trees[i].center.2 - body.pos.2) <= self.sub_trees[i].width
            {
                self.sub_trees[i].add_body(body);
            }
        }
    }

    ///Travels down the octtree until it runs into an octtree node which contains only one body, which is not the body currently being updated,
    ///OR if the width of the current octtree node, squared, is less than theta squared * the distance between the center of mass of the given node,
    ///and the body being updated.
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
