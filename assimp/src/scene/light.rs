use ffi::{AiLight, AiLightSourceType};

use crate::math::color3::Color3D;
use crate::math::vector2::Vector2D;
use crate::math::vector3::Vector3D;

define_type_and_iterator_indirect! {
    /// Light type
    struct Light(&AiLight)
    /// Light iterator type.
    struct LightIter
}

impl<'a> Light<'a> {
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn light_type(&self) -> AiLightSourceType {
        self.light_type
    }

    pub fn position(&self) -> Vector3D {
        Vector3D::from_raw(&self.position)
    }

    pub fn direction(&self) -> Vector3D {
        Vector3D::from_raw(&self.direction)
    }

    pub fn up(&self) -> Vector3D {
        Vector3D::from_raw(&self.up)
    }

    pub fn attenuation_constant(&self) -> f32 {
        self.attenuation_constant
    }

    pub fn attenuation_linear(&self) -> f32 {
        self.attenuation_linear
    }

    pub fn attenuation_quadratic(&self) -> f32 {
        self.attenuation_quadratic
    }

    pub fn color_diffuse(&self) -> Color3D {
        Color3D::from_raw(&self.color_diffuse)
    }

    pub fn color_specular(&self) -> Color3D {
        Color3D::from_raw(&self.color_specular)
    }

    pub fn color_ambient(&self) -> Color3D {
        Color3D::from_raw(&self.color_ambient)
    }

    pub fn angle_inner_cone(&self) -> f32 {
        self.angle_inner_cone
    }

    pub fn angle_outer_cone(&self) -> f32 {
        self.angle_outer_cone
    }

    pub fn size(&self) -> Vector2D {
        Vector2D::from_raw(&self.size)
    }
}
