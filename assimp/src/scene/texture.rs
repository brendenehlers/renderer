use std::ffi::CStr;
use std::os::raw::c_char;

use ffi::{AiTexture, AiTexel};

define_type_and_iterator_indirect! {
    /// Texture type.
    struct Texture(&AiTexture)
    /// Texture iterator type.
    struct TextureIter
}

impl<'a> Texture<'a> {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn format_hint(&self) -> &str {
        unsafe {
            CStr::from_ptr(self.format_hint.as_ptr() as *const c_char)
                .to_str()
                .unwrap_or("")
        }
    }

    /// Returns true if the texture is stored in a compressed format (e.g. PNG, JPEG).
    /// When compressed, `height` is 0 and `width` contains the size in bytes.
    pub fn is_compressed(&self) -> bool {
        self.height == 0
    }

    /// Returns the texture data as a slice of texels.
    /// For compressed textures, `width` is the buffer size in bytes.
    /// For uncompressed textures, the length is `width * height`.
    pub fn data(&self) -> &[AiTexel] {
        let len = if self.is_compressed() {
            self.width as usize
        } else {
            (self.width * self.height) as usize
        };
        unsafe { std::slice::from_raw_parts(self.data, len) }
    }
}
