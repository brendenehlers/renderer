use tracing::debug;

use crate::{glm, shader};

#[repr(C)]
pub struct Vertex {
    pub position: glm::Vec3,
    pub normal: glm::Vec3,
    pub tex_coords: glm::Vec2,
}

enum VertexDataType {
    Position,
    Normal,
    TexCoords,
}

#[derive(Clone)]
pub enum TextureType {
    Diffuse,
    Specular,
}

#[derive(Clone)]
pub struct Texture {
    pub id: u32,
    pub texture_type: TextureType,
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub textures: Vec<Texture>,
    vao: u32,
    vbo: u32,
    ebo: u32,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, textures: Vec<Texture>) -> Mesh {
        let _span = tracing::debug_span!(
            "mesh_setup",
            vertex_count = vertices.len(),
            index_count = indices.len(),
            texture_count = textures.len()
        )
        .entered();
        let mut mesh = Mesh {
            vertices,
            indices,
            textures,
            vao: 0,
            vbo: 0,
            ebo: 0,
        };

        setup_mesh(&mut mesh);
        debug!(vao = mesh.vao, vbo = mesh.vbo, ebo = mesh.ebo, "GPU buffers allocated");
        mesh
    }

    pub fn draw(&self, shader: &shader::Shader) -> anyhow::Result<i32> {
        let mut diffuse_nr = 1;
        let mut specular_nr = 1;

        for (i, texture) in self.textures.iter().enumerate() {
            unsafe { gl::ActiveTexture(gl::TEXTURE0 + i as u32) }
            unsafe { gl::BindTexture(gl::TEXTURE_2D, texture.id) }
            let texture_name = match texture.texture_type {
                TextureType::Diffuse => {
                    let str = format!("material.texture_diffuse{}", diffuse_nr);
                    diffuse_nr += 1;
                    str
                }
                TextureType::Specular => {
                    let str = format!("material.texture_specular{}", specular_nr);
                    specular_nr += 1;
                    str
                }
            };

            shader.set_int(&texture_name, i as i32)?;
        }
        unsafe { gl::ActiveTexture(0) };

        // todo find out how to determine this value
        shader.set_float("material.shininess", 32.0)?;

        // draw mesh
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawElements(
                gl::TRIANGLES,
                self.indices.len().try_into().unwrap(),
                gl::UNSIGNED_INT,
                0 as *const _,
            );
            gl::BindVertexArray(0);
        }

        Ok(1)
    }
}

fn setup_mesh(mesh: &mut Mesh) {
    unsafe {
        gl::GenVertexArrays(1, &mut mesh.vao);
        gl::GenBuffers(1, &mut mesh.vbo);
        gl::GenBuffers(1, &mut mesh.ebo);

        gl::BindVertexArray(mesh.vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, mesh.vbo);

        gl::BufferData(
            gl::ARRAY_BUFFER,
            (mesh.vertices.len() * std::mem::size_of::<Vertex>()) as isize,
            mesh.vertices.as_ptr() as usize as *const _,
            gl::STATIC_DRAW,
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, mesh.ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (mesh.indices.len() * std::mem::size_of::<u32>()) as isize,
            mesh.indices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        // vertex positions
        let (index, size, offset) = attrib_ptr_data(VertexDataType::Position);
        gl::EnableVertexAttribArray(index);
        gl::VertexAttribPointer(
            index,
            size,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<Vertex>().try_into().unwrap(),
            offset,
        );

        // normal
        let (index, size, offset) = attrib_ptr_data(VertexDataType::Normal);
        gl::EnableVertexAttribArray(index);
        gl::VertexAttribPointer(
            index,
            size,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<Vertex>().try_into().unwrap(),
            offset,
        );

        // tex coords
        let (index, size, offset) = attrib_ptr_data(VertexDataType::TexCoords);
        gl::EnableVertexAttribArray(index);
        gl::VertexAttribPointer(
            index,
            size,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<Vertex>().try_into().unwrap(),
            offset,
        );

        gl::BindVertexArray(0);
    }
}

fn attrib_ptr_data(t: VertexDataType) -> (u32, i32, *const std::os::raw::c_void) {
    match t {
        VertexDataType::Position => (0, 3, std::mem::offset_of!(Vertex, position) as *const _),
        VertexDataType::Normal => (1, 3, std::mem::offset_of!(Vertex, normal) as *const _),
        VertexDataType::TexCoords => (2, 2, std::mem::offset_of!(Vertex, tex_coords) as *const _),
    }
}
