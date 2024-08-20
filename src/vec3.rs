#![allow(dead_code)]

use std::ops;

struct Vec3 {
    e: [f32; 3],
}

impl Vec3 {
    pub fn default() -> Self {
        Vec3 {
            e: [0.0, 0.0, 0.0]
        }
    }

    pub fn x(&self) -> &f32 {
        &self.e[0]
    }

    pub fn y(&self) -> &f32 {
        &self.e[1]
    }

    pub fn z(&self) -> &f32 {
        &self.e[2]
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            e: [self.e[0] + rhs.e[0], self.e[1] + rhs.e[1], self.e[2] + rhs.e[2]]
        }
    }
}
