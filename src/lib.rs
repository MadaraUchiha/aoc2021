use anyhow::{anyhow, Result};
use std::ops::{Add, Mul, Neg, Sub};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Vec3(pub isize, pub isize, pub isize);

impl Vec3 {
    pub fn min(a: &Self, b: &Self) -> Self {
        Self(
            isize::min(a.0, b.0),
            isize::min(a.1, b.1),
            isize::min(a.2, b.2),
        )
    }
    pub fn max(a: &Self, b: &Self) -> Self {
        Self(
            isize::max(a.0, b.0),
            isize::max(a.1, b.1),
            isize::max(a.2, b.2),
        )
    }

    pub fn square_distance(&self, other: &Self) -> usize {
        ((self.0 - other.0).pow(2) + (self.1 - other.1).pow(2) + (self.2 - other.2).pow(2))
            .try_into()
            .unwrap()
    }
}

impl FromStr for Vec3 {
    type Err = anyhow::Error;
    fn from_str(input: &str) -> Result<Self> {
        let mut parts = input.split(',');
        let x = parts.next().ok_or(anyhow!("No x"))?.parse::<isize>()?;
        let y = parts.next().ok_or(anyhow!("No y"))?.parse::<isize>()?;
        let z = parts.next().ok_or(anyhow!("No z"))?.parse::<isize>()?;

        Ok(Self(x, y, z))
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> <Self as Add<Self>>::Output {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Mul<isize> for Vec3 {
    type Output = Self;
    fn mul(self, scalar: isize) -> <Self as Mul<isize>>::Output {
        Self(self.0 * scalar, self.1 * scalar, self.2 * scalar)
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> <Self as Neg>::Output {
        self * -1
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> <Self as Sub<Self>>::Output {
        self + (-other)
    }
}

impl<'a, 'b> Add<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;
    fn add(self, other: &'b Vec3) -> Vec3 {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl<'a, 'b> Mul<isize> for &'a Vec3 {
    type Output = Vec3;
    fn mul(self, scalar: isize) -> Vec3 {
        Vec3(self.0 * scalar, self.1 * scalar, self.2 * scalar)
    }
}

impl<'a, 'b> Neg for &'a Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        self * -1
    }
}

impl<'a, 'b> Sub<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;
    fn sub(self, other: &'b Vec3) -> Vec3 {
        self + &(-other)
    }
}

// Vec2

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Vec2(pub isize, pub isize);

impl Vec2 {
    pub fn min(a: &Self, b: &Self) -> Self {
        Self(isize::min(a.0, b.0), isize::min(a.1, b.1))
    }
    pub fn max(a: &Self, b: &Self) -> Self {
        Self(isize::max(a.0, b.0), isize::max(a.1, b.1))
    }

    pub fn square_distance(&self, other: &Self) -> usize {
        ((self.0 - other.0).pow(2) + (self.1 - other.1).pow(2))
            .try_into()
            .unwrap()
    }
}

impl FromStr for Vec2 {
    type Err = anyhow::Error;
    fn from_str(input: &str) -> Result<Self> {
        let mut parts = input.split(',');
        let x = parts.next().ok_or(anyhow!("No x"))?.parse::<isize>()?;
        let y = parts.next().ok_or(anyhow!("No y"))?.parse::<isize>()?;

        Ok(Self(x, y))
    }
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, other: Self) -> <Self as Add<Self>>::Output {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl Mul<isize> for Vec2 {
    type Output = Self;
    fn mul(self, scalar: isize) -> <Self as Mul<isize>>::Output {
        Self(self.0 * scalar, self.1 * scalar)
    }
}

impl Neg for Vec2 {
    type Output = Self;
    fn neg(self) -> <Self as Neg>::Output {
        self * -1
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, other: Self) -> <Self as Sub<Self>>::Output {
        self + (-other)
    }
}

impl<'a, 'b> Add<&'b Vec2> for &'a Vec2 {
    type Output = Vec2;
    fn add(self, other: &'b Vec2) -> Vec2 {
        Vec2(self.0 + other.0, self.1 + other.1)
    }
}

impl<'a, 'b> Mul<isize> for &'a Vec2 {
    type Output = Vec2;
    fn mul(self, scalar: isize) -> Vec2 {
        Vec2(self.0 * scalar, self.1 * scalar)
    }
}

impl<'a, 'b> Neg for &'a Vec2 {
    type Output = Vec2;
    fn neg(self) -> Vec2 {
        self * -1
    }
}

impl<'a, 'b> Sub<&'b Vec2> for &'a Vec2 {
    type Output = Vec2;
    fn sub(self, other: &'b Vec2) -> Vec2 {
        self + &(-other)
    }
}
