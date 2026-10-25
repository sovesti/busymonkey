use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug)]
pub struct Vector(pub f32, pub f32);

impl Vector {
    pub fn from_coords(coords: (i32, i32)) -> Self {
        Self(coords.0 as f32, coords.1 as f32)
    }

    pub fn into_coords(self) -> (i32, i32) {
        (self.0.round() as i32, self.1.round() as i32)
    }

    pub fn rotate(self, angle: f32) -> Self {
        Self(
            self.0 * angle.cos() + self.1 * angle.sin(),
            self.1 * angle.cos() - self.0 * angle.sin(),
        )
    }

    pub fn unit(self) -> Self {
        self / self.len()
    }

    pub fn dot(self, other: Self) -> f32 {
        self.0 * other.0 + self.1 * other.1
    }

    pub fn sqlen(self) -> f32 {
        self.0 * self.0 + self.1 * self.1
    }

    pub fn len(self) -> f32 {
        f32::hypot(self.0, self.1)
    }

    pub fn clamp_len(self, max_len: f32) -> Self {
        let len = self.len();
        if len <= max_len {
            self
        } else {
            self * (max_len / len)
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl MulAssign<f32> for Vector {
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs;
        self.1 *= rhs;
    }
}

impl DivAssign<f32> for Vector {
    fn div_assign(&mut self, rhs: f32) {
        self.0 /= rhs;
        self.1 /= rhs;
    }
}

impl Add for Vector {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Vector {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul<f32> for Vector {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl Div<f32> for Vector {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs)
    }
}
