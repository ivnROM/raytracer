use super::Vec3;
use std::ops::{self, Index, IndexMut};
use std::fmt::Display;

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.e[0],
            1 => &self.e[1],
            2 => &self.e[2],
            _ => panic!("Trying to access Vec3: Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.e[0],
            1 => &mut self.e[1],
            2 => &mut self.e[2],
            _ => panic!("Trying to access Vec3: Index out of bounds"),
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        {
            Self {
                e: [-self.e[0],-self.e[1],-self.e[2]]
            }
        }
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            e: [self.e[0] + rhs.e[0], self.e[1] + rhs.e[1], self.e[2] + rhs.e[2]]
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            e: [self.e[0] - rhs.e[0], self.e[1] - rhs.e[1], self.e[2] - rhs.e[2]]
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, factor: f64) -> Self::Output {
        Self {
            e: [self.e[0] * factor, self.e[1] * factor, self.e[2] * factor]
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, factor: Vec3) -> Self::Output {
        Vec3 {
            e: [factor.e[0] * self, factor.e[1] * self, factor.e[2] * self]
        }
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, factor: Self) -> Self::Output {
        Self {
            e: [self.e[0] * factor.e[0], self.e[1] * factor.e[1], self.e[2] * factor.e[2]]
        }
    }
}


impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, f: f64) -> Self::Output {
        Self {
            e: [self.e[0] / f, self.e[1] / f, self.e[2] / f]
        }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

