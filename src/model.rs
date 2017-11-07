use vec3f::Vec3f;

use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use wavefront_obj::obj;

pub fn load_model(path: PathBuf) -> Vec<[Vec3f; 3]> {
    // Load OBJ
    let mut file = File::open(path).expect("Unable to open the file");
    let mut obj_string = String::new();
    file.read_to_string(&mut obj_string).expect("Unable to read the file");

    let obj = obj::parse(obj_string).unwrap();
    let object = &obj.objects[0];
    let shapes = &object.geometry[0].shapes;

    let mut model = vec![];

    for shape in shapes {
        match shape.primitive {
            obj::Primitive::Triangle(vtx_1, vtx_2, vtx_3) => {
                model.push([
                    Vec3f::new(
                        object.vertices[vtx_1.0].x,
                        object.vertices[vtx_1.0].y,
                        object.vertices[vtx_1.0].z
                    ),
                    Vec3f::new(
                        object.vertices[vtx_2.0].x,
                        object.vertices[vtx_2.0].y,
                        object.vertices[vtx_2.0].z
                    ),
                    Vec3f::new(
                        object.vertices[vtx_3.0].x,
                        object.vertices[vtx_3.0].y,
                        object.vertices[vtx_3.0].z
                    ),
                ]);
            },
            _ => {}
        }
    }

    model
}
