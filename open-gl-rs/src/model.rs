use std::collections::HashMap;

use image::{ImageBuffer, Rgba};

use crate::{mesh, shader};

pub struct Model {
    meshes: Vec<mesh::Mesh>,
}

impl Model {
    pub fn load(importer: &assimp::Importer, path: &str) -> anyhow::Result<Model> {
        let scene = match importer.read_file(path) {
            Ok(scene) => scene,
            Err(s) => {
                anyhow::bail!(String::from(s))
            }
        };

        if scene.is_incomplete() {
            anyhow::bail!("failed to load scene");
        }
        let directory = path.split_at(path.rfind('/').unwrap()).0;

        let mut meshes: Vec<mesh::Mesh> = vec![];
        let mut img_cache: HashMap<String, mesh::Texture> = HashMap::new();
        process_node(
            &scene.root_node(),
            &scene,
            &mut meshes,
            directory,
            &mut img_cache,
        );

        Ok(Model { meshes })
    }

    pub fn draw(&self, shader: &shader::Shader) -> anyhow::Result<()> {
        self.meshes.iter().try_for_each(|mesh| mesh.draw(shader))
    }
}

fn process_node(
    node: &assimp::Node,
    scene: &assimp::Scene,
    meshes: &mut Vec<mesh::Mesh>,
    dir: &str,
    img_cache: &mut HashMap<String, mesh::Texture>,
) {
    println!("process_node: {:p} name={}", node.to_raw(), node.name());
    node.meshes().iter().for_each(|mesh| {
        let mesh = scene.mesh(*mesh as usize).unwrap();
        meshes.push(process_mesh(&mesh, &scene, dir, img_cache).unwrap());
    });

    node.child_iter()
        .for_each(|child| process_node(&child, scene, meshes, dir, img_cache));
}

fn process_mesh(
    mesh: &assimp::Mesh,
    scene: &assimp::Scene,
    dir: &str,
    img_cache: &mut HashMap<String, mesh::Texture>,
) -> anyhow::Result<mesh::Mesh> {
    let vertices: Vec<mesh::Vertex> = mesh
        .vertex_iter()
        .enumerate()
        .map(|(i, vertex)| {
            let position = glm::vec3(vertex.x, vertex.y, vertex.z);
            let normal_vec = mesh.get_normal(i as u32).unwrap();
            let normal = glm::vec3(normal_vec.x, normal_vec.y, normal_vec.z);

            let tex_coords = if mesh.has_texture_coords(0) {
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

    let mut indices = Vec::new();
    for face in mesh.face_iter() {
        for i in face.indices() {
            indices.push(*i);
        }
    }

    let material = scene.material(mesh.material_index as usize).unwrap();
    let diffuse_maps =
        load_material_textures(&material, assimp::AiTextureType::Diffuse, dir, img_cache)?;
    let specular_maps =
        load_material_textures(&material, assimp::AiTextureType::Specular, dir, img_cache)?;
    let textures = diffuse_maps.into_iter().chain(specular_maps).collect();

    Ok(mesh::Mesh::new(vertices, indices, textures))
}

fn load_material_textures(
    mat: &assimp::Material,
    tex_type: assimp::AiTextureType,
    dir: &str,
    img_cache: &mut HashMap<String, mesh::Texture>,
) -> anyhow::Result<Vec<mesh::Texture>> {
    let mut textures = Vec::new();

    for i in 0..mat.texture_count(tex_type) {
        let path = mat
            .get_texture(tex_type, i)
            .expect("texture should be defined");
        let path = std::path::Path::join(std::path::Path::new(dir), path);
        println!("path={:#?}", path);
        let path_str = path.to_str().expect("path should be defined").to_string();

        let texture = img_cache.entry(path_str).or_insert_with(|| {
            println!("start loading image");
            let img = image::ImageReader::open(&path)
                .unwrap()
                .decode()
                .unwrap()
                .into_rgba8();
            println!("loaded image");

            mesh::Texture {
                id: create_texture(&img).unwrap(),
                texture_type: match tex_type {
                    assimp::AiTextureType::Diffuse => mesh::TextureType::Diffuse,
                    assimp::AiTextureType::Specular => mesh::TextureType::Specular,
                    _ => panic!("unknown texture type"),
                },
            }
        });

        textures.push(texture.clone());
    }

    Ok(textures)
}

fn create_texture(img: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> anyhow::Result<u32> {
    // todo!("identify and fix memory leak likely in this method");

    println!("create texture start");
    let mut tex: u32 = 0;
    unsafe { gl::GenTextures(1, &mut tex) };
    unsafe {
        gl::BindTexture(gl::TEXTURE_2D, tex);
    }

    unsafe {
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_S,
            gl::REPEAT.try_into().unwrap(),
        )
    };
    unsafe {
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_T,
            gl::REPEAT.try_into().unwrap(),
        )
    };
    unsafe {
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::LINEAR_MIPMAP_LINEAR.try_into().unwrap(),
        )
    };
    unsafe {
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MAG_FILTER,
            gl::LINEAR.try_into().unwrap(),
        )
    };

    // TODO determine RGBA vs. RGA dynamically
    unsafe {
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA.try_into().unwrap(),
            img.width().try_into().unwrap(),
            img.height().try_into().unwrap(),
            0,
            gl::RGBA.try_into().unwrap(),
            gl::UNSIGNED_BYTE,
            img.as_raw().as_ptr() as *const _,
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }
    unsafe {
        gl::BindTexture(gl::TEXTURE_2D, 0);
    }

    println!("create texture end");
    Ok(tex)
}
