use crate::{mesh, shader};

pub struct Model {
    meshes: Vec<mesh::Mesh>,
    dir: String,
}

impl Model {
    pub fn load(path: &String) -> anyhow::Result<Model> {
        let mut importer = assimp::Importer::new();
        importer.triangulate(true);
        importer.flip_uvs(true);

        let scene = match importer.read_file(path) {
            Ok(s) => s,
            Err(s) => {
                let msg = s.clone();
                anyhow::bail!(msg)
            }
        };

        if scene.is_incomplete() {
            anyhow::bail!("failed to load scene");
        }
        let directory = path.split_at(path.rfind('/').unwrap()).0;

        let mut meshes: Vec<mesh::Mesh> = vec![];
        process_node(&scene.root_node(), &scene, &mut meshes);
        todo!()
    }

    pub fn draw(&self, shader: &shader::Shader) -> anyhow::Result<()> {
        self.meshes.iter().try_for_each(|mesh| mesh.draw(shader))
    }
}

fn process_node(node: &assimp::Node, scene: &assimp::Scene, meshes: &mut Vec<mesh::Mesh>) {
    // load meshes in to vec
    node.meshes().iter().for_each(|mesh| {
        let mesh = scene.mesh(*mesh as usize).unwrap();
        meshes.push(process_mesh(&mesh, &scene));
    });

    // recursively process child nodes
    node.child_iter()
        .for_each(|child| process_node(&child, scene, meshes));
}

fn process_mesh(mesh: &assimp::Mesh, scene: &assimp::Scene) -> mesh::Mesh {
    let mut vertices: Vec<mesh::Vertex> = mesh
        .vertex_iter()
        .enumerate()
        .map(|(i, vertex)| {
            let position = glm::vec3(vertex.x, vertex.y, vertex.z);
            let normal_vec = mesh.get_normal(i as u32).unwrap();
            let normal = glm::vec3(normal_vec.x, normal_vec.y, normal_vec.z);

            let tex_coords = if mesh.has_texture_coords(i) {
                let tex_coords = mesh.get_texture_coord(0, i as u32).unwrap();
                glm::vec2(tex_coords.x, tex_coords.y)
            } else {
                glm::vec2(0.0, 0.0)
            };

            mesh::Vertex {
                position,
                normal,
                tex_coords,
            }
        })
        .collect();

    let indices = mesh.face_iter().map(|face| {
        face
    }).collect();

    let textures;
    if mesh.material_index >= 0 {
        textures = mesh.texture_coords_iter(todo!()).map(|vec3d| todo!());
    }

    mesh::Mesh::new(vertices, indices, textures)
}

fn load_material_textures(
    mat: &assimp::Material,
    tex_type: &assimp::Texture,
    tex_type_name: &str,
) -> Vec<mesh::Texture> {
    todo!()
}
