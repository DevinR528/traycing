use std::ops::{Add, AddAssign, Deref, Div, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};

pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub xyz: [f32; 3],
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { xyz: [x, y, z] }
    }

    pub fn x(&self) -> f32 {
        self[Axis::X]
    }

    pub fn y(&self) -> f32 {
        self[Axis::Y]
    }

    pub fn z(&self) -> f32 {
        self[Axis::Z]
    }

    pub fn r(&self) -> f32 {
        self[Axis::X]
    }

    pub fn g(&self) -> f32 {
        self[Axis::Y]
    }

    pub fn b(&self) -> f32 {
        self[Axis::Z]
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        use Axis::*;
        Self {
            xyz: [self[X] + rhs[X], self[Y] + rhs[Y], self[Z] + rhs[Z]],
        }
    }
}
impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        use Axis::*;
        Self {
            xyz: [self[X] - rhs[X], self[Y] - rhs[Y], self[Z] - rhs[Z]],
        }
    }
}
impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        use Axis::*;
        Self {
            xyz: [self[X] * rhs[X], self[Y] * rhs[Y], self[Z] * rhs[Z]],
        }
    }
}
impl Div for Vec3 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        use Axis::*;
        Self {
            xyz: [self[X] / rhs[X], self[Y] / rhs[Y], self[Z] / rhs[Z]],
        }
    }
}
impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        use Axis::*;
        Self {
            xyz: [-self[X], -self[Y], -self[Z]],
        }
    }
}
impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        use Axis::*;
        self[X] += rhs[X];
        self[Y] += rhs[Y];
        self[Z] += rhs[Z];
    }
}
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        use Axis::*;
        self[X] -= rhs[X];
        self[Y] -= rhs[Y];
        self[Z] -= rhs[Z];
    }
}
impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        use Axis::*;
        self[X] *= rhs[X];
        self[Y] *= rhs[Y];
        self[Z] *= rhs[Z];
    }
}
impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        use Axis::*;
        self[X] *= rhs;
        self[Y] *= rhs;
        self[Z] *= rhs;
    }
}

impl Index<Axis> for Vec3 {
    type Output = f32;
    fn index(&self, index: Axis) -> &Self::Output {
        match index {
            Axis::X => &self.xyz[0],
            Axis::Y => &self.xyz[1],
            Axis::Z => &self.xyz[2],
        }
    }
}
impl IndexMut<Axis> for Vec3 {
    fn index_mut(&mut self, index: Axis) -> &mut Self::Output {
        match index {
            Axis::X => &mut self.xyz[0],
            Axis::Y => &mut self.xyz[1],
            Axis::Z => &mut self.xyz[2],
        }
    }
}

impl Deref for Vec3 {
    type Target = [f32];

    fn deref(&self) -> &Self::Target {
        &self.xyz
    }
}
#[allow(clippy::float_cmp)]
#[cfg(test)]
mod test {
    use super::{Axis::*, *};

    #[test]
    fn construct_add() {
        let a = Vec3::new(1., 2., 3.);
        let b = Vec3::new(3., 2., 1.);

        let c = a + b;

        assert_eq!(c[X], 4.0)
    }
}
