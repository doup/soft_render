use triangle::Triangle;
use vec3f::Vec3f;

use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use wavefront_obj::obj;

pub fn load_model(path: PathBuf) -> Vec<Triangle> {
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
                let uvx_1 = vtx_1.1.unwrap();
                let uvx_2 = vtx_2.1.unwrap();
                let uvx_3 = vtx_3.1.unwrap();

                model.push(Triangle {
                    vertices: [
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
                    ],
                    uv: [
                        Vec3f::new(
                            object.tex_vertices[uvx_1].u,
                            object.tex_vertices[uvx_1].v,
                            object.tex_vertices[uvx_1].w
                        ),
                        Vec3f::new(
                            object.tex_vertices[uvx_2].u,
                            object.tex_vertices[uvx_2].v,
                            object.tex_vertices[uvx_2].w
                        ),
                        Vec3f::new(
                            object.tex_vertices[uvx_3].u,
                            object.tex_vertices[uvx_3].v,
                            object.tex_vertices[uvx_3].w
                        ),
                    ]
                });
            },
            _ => {}
        }
    }

    model
}
