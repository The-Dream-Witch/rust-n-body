use crate::vec3d::*;

pub const XMAX: f64 = 1920.;
pub const YMAX: f64 = 1080.;
pub const ZMAX: f64 = 800.;

///This is the struct defining what a 'body' / 'particle' / 'planet' / 'star' is; it makes use of the previously established 3D vectors, as well as an f64 to represent the mass.
#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Body {
    pub pos: Vec3D,
    pub vel: Vec3D,
    pub mass: f64,
}

impl Body {
    ///Instantiates a new body using pseudo-randomized values for its mass, and its position and velocity vectors
    pub fn new() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        Self {
            pos: Vec3D::new_pos(),
            vel: Vec3D::new_vel(),
            mass: rng.gen_range(1..100) as f64,
        }
    }
    ///Checks to see if a body has gone out of bounds; if so, the body is placed at the edge of the boundary in question, and its velocity in that direction is inverted.
    pub fn bounds(&mut self) {
        if self.pos.0 < 0. {
            self.pos.0 = 0.;
            self.vel.0 *= -1.;
        } else if self.pos.0 > XMAX {
            self.pos.0 = XMAX;
            self.vel.0 *= -1.;
        }

        if self.pos.1 < 0. {
            self.pos.1 = 0.;
            self.vel.1 *= -1.;
        } else if self.pos.1 > YMAX {
            self.pos.1 = YMAX;
            self.vel.1 *= -1.;
        }

        if self.pos.2 < 0. {
            self.pos.2 = 0.;
            self.vel.2 *= -1.;
        } else if self.pos.2 > ZMAX {
            self.pos.2 = ZMAX;
            self.vel.2 *= -1.;
        }
    }
}

///Just a default
impl Default for Body {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod body_tests {
    use crate::body::*;

    #[test]
    pub fn bounds_test() {
        let mut test_body = Body::new();

        test_body.pos.0 += XMAX;
        test_body.pos.1 += YMAX;
        test_body.pos.2 += ZMAX;

        assert!(test_body.pos.0 > XMAX && test_body.pos.1 > YMAX && test_body.pos.2 > ZMAX);
        test_body.bounds();
        assert!(test_body.pos.0 <= XMAX && test_body.pos.1 <= YMAX && test_body.pos.2 <= ZMAX);
    }
}
