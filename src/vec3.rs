use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct Vec3 {
    e0: f32,
    e1: f32,
    e2: f32,
}

impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
        Vec3 { e0, e1, e2 }
    }

    pub fn x(self) -> f32 {
        self.e0
    }

    pub fn y(self) -> f32 {
        self.e1
    }

    pub fn z(self) -> f32 {
        self.e2
    }

    pub fn r(self) -> f32 {
        self.e0
    }

    pub fn g(self) -> f32 {
        self.e1
    }

    pub fn b(self) -> f32 {
        self.e2
    }

    pub fn make_unit_vector(&mut self) -> Vec3 {
        let length = 1.0 / (self.e0.powi(2) + self.e1.powi(2) + self.e2.powi(2)).sqrt();
        self.e0 *= length;
        self.e1 *= length;
        self.e2 *= length;
        *self
    }

    pub fn dot(a: Vec3, b: Vec3) -> f32 {
        a.e0 * b.e0 + a.e1 * b.e1 + a.e2 * b.e2
    }

    pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
        Vec3 {
            e0: a.e1 * b.e2 - a.e2 * b.e1,
            e1: -(a.e0 * b.e2 - a.e2 * b.e0),
            e2: a.e0 * b.e1 - a.e1 * b.e0,
        }
    }

    pub fn len(self) -> f32 {
        (self.e0.powi(2) + self.e1.powi(2) + self.e2.powi(2)).sqrt()
    }

    pub fn squared_len(self) -> f32 {
        self.e0.powi(2) + self.e1.powi(2) + self.e2.powi(2)
    }

    pub fn unit_vector(vec: Vec3) -> Vec3 {
        vec / vec.len()
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {} {}", self.e0, self.e1, self.e2)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            e0: -self.e0,
            e1: -self.e1,
            e2: -self.e2,
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            e0: self.e0 + other.e0,
            e1: self.e1 + other.e1,
            e2: self.e2 + other.e2,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            e0: self.e0 - other.e0,
            e1: self.e1 - other.e1,
            e2: self.e2 - other.e2,
        }
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            e0: self.e0 / other.e0,
            e1: self.e1 / other.e1,
            e2: self.e2 / other.e2,
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            e0: self.e0 * other.e0,
            e1: self.e1 * other.e1,
            e2: self.e2 * other.e2,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Vec3 {
        Vec3 {
            e0: self.e0 * other,
            e1: self.e1 * other,
            e2: self.e2 * other,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            e0: self * other.e0,
            e1: self * other.e1,
            e2: self * other.e2,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f32) -> Vec3 {
        Vec3 {
            e0: self.e0 / other,
            e1: self.e1 / other,
            e2: self.e2 / other,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Vec3 {
            e0: self.e0 + other.e0,
            e1: self.e1 + other.e1,
            e2: self.e2 + other.e2,
        };
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        *self = Vec3 {
            e0: self.e0 - other.e0,
            e1: self.e1 - other.e1,
            e2: self.e2 - other.e2,
        };
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Vec3) {
        *self = Vec3 {
            e0: self.e0 / other.e0,
            e1: self.e1 / other.e1,
            e2: self.e2 / other.e2,
        };
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other: f32) {
        *self = Vec3 {
            e0: self.e0 * (1.0 / other),
            e1: self.e1 * (1.0 / other),
            e2: self.e2 * (1.0 / other),
        };
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        *self = Vec3 {
            e0: self.e0 * other.e0,
            e1: self.e1 * other.e1,
            e2: self.e2 * other.e2,
        };
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) {
        *self = Vec3 {
            e0: self.e0 * other,
            e1: self.e1 * other,
            e2: self.e2 * other,
        };
    }
}
