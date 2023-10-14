use std::ops::*;

pub type Point = Vec3<f64>;
pub type Vector = Vec3<f64>;
pub type Color = Vec3<u8>;

pub trait Numeric:
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + PartialEq
    + Sized
    + Copy
    + Clone
{
    fn is_signed() -> bool;
    fn sqrt(self) -> Self;
}

impl Numeric for u8 {
    fn is_signed() -> bool {
        false
    }

    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

impl Numeric for f64 {
    fn is_signed() -> bool {
        true
    }

    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Copy)]
pub struct Vec3<T: Numeric> {
    x: T,
    y: T,
    z: T,
}

impl<T: Numeric> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Self { x, y, z }
    }
    pub fn x(&self) -> T {
        self.x
    }

    pub fn y(&self) -> T {
        self.y
    }

    pub fn z(&self) -> T {
        self.z
    }

    pub fn sum(&self) -> T {
        self.x + self.y + self.z
    }

    pub fn mag(&self) -> T {
        self.mag_sq().sqrt()
    }

    pub fn mag_sq(&self) -> T {
        self.dot(self)
    }

    pub fn dot(&self, other: &Vec3<T>) -> T {
        (*self * *other).sum()
    }

    pub fn cross(&self, other: &Vec3<T>) -> Vec3<T> {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn unit_vector(&self) -> Vec3<T> {
        *self / self.mag()
    }
}

impl<T: Numeric> Add for Vec3<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Numeric> Sub for Vec3<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: Numeric> Mul for Vec3<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<T: Numeric> Mul<T> for Vec3<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T: Numeric> Div for Vec3<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl<T: Numeric> Div<T> for Vec3<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

// impl<T: Numeric> Neg for Vec3<T> {
//     type Output = Self;

//     fn neg(self) -> Self {
//         if T::is_signed() {
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

// impl<T: AddAssign + Numeric> AddAssign for Vec3<T> {
//     fn add_assign(&mut self, rhs: Self) {
//         *self = Self {
//             x: self.x + rhs.x,
//             y: self.y + rhs.y,
//             z: self.z + rhs.z,
//         };
//     }
// }

// impl<T: SubAssign + Numeric> SubAssign for Vec3<T> {
//     fn sub_assign(&mut self, rhs: Self) {
//         *self = Self {
//             x: self.x - rhs.x,
//             y: self.y - rhs.y,
//             z: self.z - rhs.z,
//         };
//     }
// }

// impl<T: MulAssign + Numeric> MulAssign for Vec3<T> {
//     fn mul_assign(&mut self, rhs: Self) {
//         *self = Self {
//             x: self.x * rhs.x,
//             y: self.y * rhs.y,
//             z: self.z * rhs.z,
//         };
//     }
// }

// impl<T: DivAssign + Numeric> DivAssign for Vec3<T> {
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

// impl<T> Div<Vec3<T>> for Vec3<T>
// where
//     T: Numeric + Clone,
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

// impl<T> Div<T> for Vec3<T>
// where
//     T: Numeric + Clone,
// {
//     type Output = Self;

//     fn div(self, rhs: T) -> Self {
//         Self {
//             x: self.x / rhs,
//             y: self.y / rhs,
//             z: self.z / rhs,
//         }
//     }
// }

// impl<T> Neg for Vec3<T>
// where
//     T: Numeric + Clone,
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

// impl<T> AddAssign<Vec3<T>> for Vec3<T>
// where
//     T: Numeric + Clone,
// {
//     fn add_assign(&mut self, other: Self) {
//         *self = Self {
//             x: self.x + other.x,
//             y: self.y + other.y,
//             z: self.y + other.z,
//         };
//     }
// }

// impl<T> SubAssign<Vec3<T>> for Vec3<T>
// where
//     T: Numeric + Clone,
// {
//     fn sub_assign(&mut self, other: Self) {
//         *self = Self {
//             x: self.x - other.x,
//             y: self.y - other.y,
//             z: self.y - other.z,
//         };
//     }
// }

// impl<T> MulAssign<Vec3<T>> for Vec3<T>
// where
//     T: Numeric + Clone,
// {
//     fn mul_assign(&mut self, other: Self) {
//         *self = Self {
//             x: self.x * other.x,
//             y: self.y * other.y,
//             z: self.y * other.z,
//         };
//     }
// }

// impl<T> DivAssign<T> for Vec3<T>
// where
//     T: Numeric + Clone,
// {
//     fn div_assign(&mut self, rhs: T) {
//         *self = Self {
//             x: self.x / rhs,
//             y: self.y / rhs,
//             z: self.y / rhs,
//         };
//     }
// }

// impl<T> DivAssign<Vec3<T>> for Vec3<T>
// where
//     T: Numeric + Clone,
// {
//     fn div_assign(&mut self, other: Self) {
//         *self = Self {
//             x: self.x / other.x,
//             y: self.y / other.y,
//             z: self.y / other.z,
//         };
//     }
// }
