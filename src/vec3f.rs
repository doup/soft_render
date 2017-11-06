use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Vec3f {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3f {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3f {
        Vec3f { x: x, y: y, z: z }
    }

    pub fn new_zero() -> Vec3f {
        Vec3f { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn cross(&self, v: &Vec3f) -> Vec3f {
        Vec3f {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        }
    } 

    pub fn dot(&self, v: &Vec3f) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    } 

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Vec3f {
        let length = self.length();

        if length > 0.0 {
            Vec3f {
                x: self.x / length,
                y: self.y / length,
                z: self.z / length,
            }
        } else {
            self.clone()
        }
    } 
}

impl Add for Vec3f {
    type Output = Vec3f;

    fn add(self, v: Vec3f) -> Vec3f {
        Vec3f {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }
}

impl Sub for Vec3f {
    type Output = Vec3f;

    fn sub(self, v: Vec3f) -> Vec3f {
        Vec3f {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
        }
    }
}

// fn main() {
//     let a = Vec3f::new(2.0, 1.0, 0.0);
//     let b = Vec3f::new(3.0, 0.0, 0.0);
//     let zero = Vec3f::new_zero();
//     let x = Vec3f::new(1.0, 0.0, 0.0);
//     let y = Vec3f::new(0.0, 1.0, 0.0);

//     println!("A = {:?}", a);
//     println!("B = {:?}", b);
//     println!("Zero = {:?}", zero);
//     println!("Zero.clone() = {:?}", zero.clone());
//     println!("A.length() = {:?}", a.length());
//     println!("A.normalize().length() = {:?}", a.normalize().length());
//     println!("A.dot(B) = {:?}", a.dot(&b));
//     println!("X.cross(Y) = {:?}", x.cross(&y));
//     println!("A + B = {:?}", a + b);
//     println!("A + B = {:?}", a - b);
// }
