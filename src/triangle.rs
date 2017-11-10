use vec3f::Vec3f;

pub struct Triangle {
    pub vertices: [Vec3f; 3],
    pub uv: [Vec3f; 3],
}

impl Triangle {
    pub fn normal(&self) -> Vec3f {
        let a = self.vertices[2] - self.vertices[0];
        let b = self.vertices[1] - self.vertices[0];
        
        a.cross(&b).normalize()
    }
}
