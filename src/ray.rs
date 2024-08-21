#![allow(dead_code)]
use crate::vec3::{Point3, Vec3};

pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    // Creates a Ray with an (o)rigin and a (d)irection
    pub fn new(o: Point3, d: Vec3) -> Self {
        Self {
            orig: o,
            dir: d,
        }
    }


    pub const fn origin(&self) -> Point3 {
        self.orig
    }

    pub const fn direction(&self) -> Vec3 {
        self.orig
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t*self.dir
    }

}
