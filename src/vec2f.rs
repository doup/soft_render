use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Vec2f {
    pub x: f64,
    pub y: f64,
}

impl Vec2f {
    pub fn new(x: f64, y: f64) -> Vec2f {
        Vec2f { x: x, y: y }
    }

    pub fn new_zero() -> Vec2f {
        Vec2f { x: 0.0, y: 0.0 }
    }

    pub fn dot(&self, v: &Vec2f) -> f64 {
        self.x * v.x + self.y * v.y
    } 

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&self) -> Vec2f {
        let length = self.length();

        if length > 0.0 {
            Vec2f {
                x: self.x / length,
                y: self.y / length,
            }
        } else {
            self.clone()
        }
    }
}

impl Add for Vec2f {
    type Output = Vec2f;

    fn add(self, v: Vec2f) -> Vec2f {
        Vec2f {
            x: self.x + v.x,
            y: self.y + v.y,
        }
    }
}

impl Sub for Vec2f {
    type Output = Vec2f;

    fn sub(self, v: Vec2f) -> Vec2f {
        Vec2f {
            x: self.x - v.x,
            y: self.y - v.y,
        }
    }
}

// fn main() {
//     let a = Vec2f::new(2.0, 1.0, 0.0);
//     let b = Vec2f::new(3.0, 0.0, 0.0);
//     let zero = Vec2f::new_zero();
//     let x = Vec2f::new(1.0, 0.0, 0.0);
//     let y = Vec2f::new(0.0, 1.0, 0.0);

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
