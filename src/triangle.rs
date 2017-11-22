use cgmath::InnerSpace;
use cgmath::Vector3;

pub struct Triangle {
    pub vertices: [Vector3<f64>; 3],
    pub uv: [Vector3<f64>; 3],
}

impl Triangle {
    pub fn normal(&self) -> Vector3<f64> {
        let a = self.vertices[2] - self.vertices[0];
        let b = self.vertices[1] - self.vertices[0];
        
        a.cross(b).normalize()
    }
}
