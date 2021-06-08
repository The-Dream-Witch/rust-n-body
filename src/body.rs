use crate::vec3d::*;

pub const XMAX: f64 = 1920.;
pub const YMAX: f64 = 1080.;
pub const ZMAX: f64 = 800.;

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