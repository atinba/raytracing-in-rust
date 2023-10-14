use std::ops::*;

pub type Point = Vec3;
pub type Vector = Vec3;
pub type Color = Vec3;

#[derive(Debug, Default, Clone, PartialEq, Copy)]
pub struct Vec3{
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Self { x, y, z }
    }
    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }

    pub fn sum(&self) -> f32 {
        self.x + self.y + self.z
    }

    pub fn mag(&self) -> f32 {
        self.mag_sq().sqrt()
    }

    pub fn mag_sq(&self) -> f32 {
        self.dot(self)
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        (*self * *other).sum()
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.mag()
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

// impl Neg for Vec3 {
//     type Output = Self;

//     fn neg(self) -> Self {
//         if f32::is_signed() {
//             Self {
//                 x: -self.x,
//                 y: -self.y,
//                 z: -self.z,
//             }
//         } else {
//             panic!("Cannot negate an unsigned type");
//         }
//     }
// }

// impl<f32: AddAssign + Numeric> AddAssign for Vec3 {
//     fn add_assign(&mut self, rhs: Self) {
//         *self = Self {
//             x: self.x + rhs.x,
//             y: self.y + rhs.y,
//             z: self.z + rhs.z,
//         };
//     }
// }

// impl<f32: SubAssign + Numeric> SubAssign for Vec3 {
//     fn sub_assign(&mut self, rhs: Self) {
//         *self = Self {
//             x: self.x - rhs.x,
//             y: self.y - rhs.y,
//             z: self.z - rhs.z,
//         };
//     }
// }

// impl<f32: MulAssign + Numeric> MulAssign for Vec3 {
//     fn mul_assign(&mut self, rhs: Self) {
//         *self = Self {
//             x: self.x * rhs.x,
//             y: self.y * rhs.y,
//             z: self.z * rhs.z,
//         };
//     }
// }

// impl<f32: DivAssign + Numeric> DivAssign for Vec3 {
//     fn div_assign(&mut self, rhs: Self) {
//         *self = Self {
//             x: self.x / rhs.x,
//             y: self.y / rhs.y,
//             z: self.z / rhs.z,
//         };
//     }
// }

// fn main() {
//     let vec1 = Vec3 { x: 1, y: 2, z: 3 };
//     let vec2 = Vec3 { x: 4, y: 5, z: 6 };

//     let added = vec1 + vec2;
//     let subtracted = vec1 - vec2;
//     let multiplied = vec1 * vec2;
//     let divided = vec1 / vec2;
//     let negated = -vec1;

//     println!("Added: {:?}", added);
//     println!("Subtracted: {:?}", subtracted);
//     println!("Multiplied: {:?}", multiplied);
//     println!("Divided: {:?}", divided);
//     println!("Negated: {:?}", negated);

//     // Example with assignment operators
//     let mut vec3 = Vec3 { x: 2, y: 3, z: 4 };
//     vec3 += vec1;
//     println!("After addition assignment: {:?}", vec3);
// }

// impl<TL: Add<TR>, TR> Add<Vec3<TR>> for Vec3<TL> {
//     type Output = Vec3<<TL as Add<TR>>::Output>;
//     fn add(self, other: Vec3<TR>) -> <Self as Add<Vec3<TR>>>::Output {
//         Vec3 {
//             x: self.x + other.x,
//             y: self.y + other.y,
//             z: self.z + other.z,
//         }
//     }
// }

// impl<TL: Sub<TR>, TR> Sub<Vec3<TR>> for Vec3<TL> {
//     type Output = Vec3<<TL as Sub<TR>>::Output>;
//     fn sub(self, other: Vec3<TR>) -> <Self as Sub<Vec3<TR>>>::Output {
//         Vec3 {
//             x: self.x - other.x,
//             y: self.y - other.y,
//             z: self.z - other.z,
//         }
//     }
// }

// impl<TL: Mul<TR>, TR> Mul<Vec3<TR>> for Vec3<TL> {
//     type Output = Vec3<<TL as Mul<TR>>::Output>;
//     fn mul(self, other: Vec3<TR>) -> <Self as Mul<Vec3<TR>>>::Output {
//         Vec3 {
//             x: self.x * other.x,
//             y: self.y * other.y,
//             z: self.z * other.z,
//         }
//     }
// }

// impl Div<Vec3> for Vec3
// where
//     f32: Numeric + Clone,
// {
//     type Output = Self;

//     fn div(self, other: Self) -> Self {
//         Self {
//             x: self.x / other.x,
//             y: self.y / other.y,
//             z: self.z / other.z,
//         }
//     }
// }

// impl Div for Vec3
// where
//     f32: Numeric + Clone,
// {
//     type Output = Self;

//     fn div(self, rhs: f32) -> Self {
//         Self {
//             x: self.x / rhs,
//             y: self.y / rhs,
//             z: self.z / rhs,
//         }
//     }
// }

// impl Neg for Vec3
// where
//     f32: Numeric + Clone,
// {
//     type Output = Self;

//     fn neg(self) -> Self {
//         Self {
//             x: -self.x,
//             y: -self.y,
//             z: -self.z,
//         }
//     }
// }

// impl AddAssign<Vec3> for Vec3
// where
//     f32: Numeric + Clone,
// {
//     fn add_assign(&mut self, other: Self) {
//         *self = Self {
//             x: self.x + other.x,
//             y: self.y + other.y,
//             z: self.y + other.z,
//         };
//     }
// }

// impl SubAssign<Vec3> for Vec3
// where
//     f32: Numeric + Clone,
// {
//     fn sub_assign(&mut self, other: Self) {
//         *self = Self {
//             x: self.x - other.x,
//             y: self.y - other.y,
//             z: self.y - other.z,
//         };
//     }
// }

// impl MulAssign<Vec3> for Vec3
// where
//     f32: Numeric + Clone,
// {
//     fn mul_assign(&mut self, other: Self) {
//         *self = Self {
//             x: self.x * other.x,
//             y: self.y * other.y,
//             z: self.y * other.z,
//         };
//     }
// }

// impl DivAssign for Vec3
// where
//     f32: Numeric + Clone,
// {
//     fn div_assign(&mut self, rhs: f32) {
//         *self = Self {
//             x: self.x / rhs,
//             y: self.y / rhs,
//             z: self.y / rhs,
//         };
//     }
// }

// impl DivAssign<Vec3> for Vec3
// where
//     f32: Numeric + Clone,
// {
//     fn div_assign(&mut self, other: Self) {
//         *self = Self {
//             x: self.x / other.x,
//             y: self.y / other.y,
//             z: self.y / other.z,
//         };
//     }
// }
