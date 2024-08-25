#![allow(dead_code, unused)]
mod vec3_operations;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    /// Generates a new Vec3 with default values [0, 0, 0]
    pub fn default() -> Self {
        Vec3 {
            e: [0.0, 0.0, 0.0]
        }
    }

    /// Generates a new Vec3 with custom values 
    pub fn new(e: [f64; 3]) -> Self {
        Vec3 {
            e,
        }
    }

    pub fn x(&self) -> &f64 {
        &self.e[0]
    }

    pub fn y(&self) -> &f64 {
        &self.e[1]
    }

    pub fn z(&self) -> &f64 {
        &self.e[2]
    }

    /// Returns length of vector
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Calculates dot product from two vectors
    pub fn dotp(factor1: Self, factor2: Self) -> f64 {
          factor1.e[0] * factor2.e[0]
        + factor1.e[1] * factor2.e[1] 
        + factor1.e[2] * factor2.e[2]
    }

    /// Calculates cross product from two vectors
    pub fn crossp(factor1: Self, factor2: Self) -> Self {
        let result = [
            factor1.e[1] * factor2.e[2] - factor1.e[2] * factor2.e[1],
            factor1.e[2] * factor2.e[0] - factor1.e[0] * factor2.e[2],
            factor1.e[0] * factor2.e[1] - factor1.e[1] * factor2.e[0],
        ];

        Self {
            e: result,
        }
    }

    /// Returns unit vector from the given vector
    pub fn unit_v(vector: Self) -> Self {
        let result = vector / vector.length(); 
        result
    }
    pub fn length_squared(&self) -> f64 {

        self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2]
    }

}

pub type Color = Vec3;
pub type Point3 = Vec3;
