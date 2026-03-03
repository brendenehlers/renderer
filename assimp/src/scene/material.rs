use std::ffi::CString;
use std::os::raw::{c_float, c_int, c_uint};
use std::ptr;

use ffi::*;

use crate::math::color4::Color4D;

define_type_and_iterator_indirect! {
    /// Material type
    struct Material(&AiMaterial)
    /// Material iterator type.
    struct MaterialIter
}

impl<'a> Material<'a> {
    pub fn num_properties(&self) -> u32 {
        self.num_properties
    }

    pub fn get_color(&self, key: &str, tex_type: c_uint, tex_index: c_uint) -> Option<Color4D> {
        let c_key = CString::new(key).ok()?;
        let mut color = AiColor4D { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };
        let result = unsafe {
            aiGetMaterialColor(self.to_raw(), c_key.as_ptr(), tex_type, tex_index, &mut color)
        };
        if result == AiReturn::Success {
            Some(Color4D::from_raw(&color))
        } else {
            None
        }
    }

    pub fn get_float(&self, key: &str, tex_type: c_uint, tex_index: c_uint) -> Option<f32> {
        let c_key = CString::new(key).ok()?;
        let mut value: c_float = 0.0;
        let mut max: c_uint = 1;
        let result = unsafe {
            aiGetMaterialFloatArray(
                self.to_raw(), c_key.as_ptr(), tex_type, tex_index, &mut value, &mut max,
            )
        };
        if result == AiReturn::Success {
            Some(value)
        } else {
            None
        }
    }

    pub fn get_integer(&self, key: &str, tex_type: c_uint, tex_index: c_uint) -> Option<i32> {
        let c_key = CString::new(key).ok()?;
        let mut value: c_int = 0;
        let mut max: c_uint = 1;
        let result = unsafe {
            aiGetMaterialIntegerArray(
                self.to_raw(), c_key.as_ptr(), tex_type, tex_index, &mut value, &mut max,
            )
        };
        if result == AiReturn::Success {
            Some(value)
        } else {
            None
        }
    }

    pub fn get_string(&self, key: &str, tex_type: c_uint, tex_index: c_uint) -> Option<String> {
        let c_key = CString::new(key).ok()?;
        let mut ai_string = AiString::default();
        let result = unsafe {
            aiGetMaterialString(
                self.to_raw(), c_key.as_ptr(), tex_type, tex_index, &mut ai_string,
            )
        };
        if result == AiReturn::Success {
            Some(ai_string.as_ref().to_owned())
        } else {
            None
        }
    }

    pub fn texture_count(&self, tex_type: AiTextureType) -> u32 {
        unsafe { aiGetMaterialTextureCount(self.to_raw(), tex_type) }
    }

    pub fn get_texture(&self, tex_type: AiTextureType, index: c_uint) -> Option<String> {
        let mut path = AiString::default();
        let result = unsafe {
            aiGetMaterialTexture(
                self.to_raw(),
                tex_type,
                index,
                &mut path,
                ptr::null(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
            )
        };
        if result == AiReturn::Success {
            Some(path.as_ref().to_owned())
        } else {
            None
        }
    }
}
