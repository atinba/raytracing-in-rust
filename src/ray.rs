use crate::vec3::*;

struct Ray {
    orig: Point,
    dir: Vector,
}

impl Ray {
    fn origin(&self) -> Point {
        self.orig
    }

    fn direction(&self) -> Vector {
        self.dir
    }

    fn at(&self, t: f64) -> Point {
        self.orig + self.dir * t
    }
}