use ffi::AiCamera;

use crate::math::vector3::Vector3D;

define_type_and_iterator_indirect! {
    /// Camera type
    struct Camera(&AiCamera)
    /// Camera iterator type.
    struct CameraIter
}

impl<'a> Camera<'a> {
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn position(&self) -> Vector3D {
        Vector3D::from_raw(&self.position)
    }

    pub fn up(&self) -> Vector3D {
        Vector3D::from_raw(&self.up)
    }

    pub fn look_at(&self) -> Vector3D {
        Vector3D::from_raw(&self.look_at)
    }

    pub fn horizontal_fov(&self) -> f32 {
        self.horizontal_fov
    }

    pub fn clip_plane_near(&self) -> f32 {
        self.clip_plane_near
    }

    pub fn clip_plane_far(&self) -> f32 {
        self.clip_plane_far
    }

    pub fn aspect(&self) -> f32 {
        self.aspect
    }
}
