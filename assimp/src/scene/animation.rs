use ffi::AiAnimation;
use ffi::AiNodeAnim;
use ffi::AiVectorKey;
use ffi::AiQuatKey;
use ffi::AiAnimBehaviour;

use crate::math::vector3::Vector3D;
use crate::math::quaternion::Quaternion;

define_type_and_iterator_indirect! {
    /// Animation type
    struct Animation(&AiAnimation)
    /// Animation iterator type.
    struct AnimationIter
}

define_type_and_iterator_indirect! {
    /// NodeAnim type
    struct NodeAnim(&AiNodeAnim)
    /// NodeAnim iterator type.
    struct NodeAnimIter
}

define_type_and_iterator! {
    /// VectorKey type
    struct VectorKey(&AiVectorKey)
    /// VectorKey iterator type.
    struct VectorKeyIter
}

define_type_and_iterator! {
    /// QuatKey type
    struct QuatKey(&AiQuatKey)
    /// QuatKey iterator type.
    struct QuatKeyIter
}

impl<'a> VectorKey<'a> {
    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn value(&self) -> Vector3D {
        Vector3D::from_raw(&self.value)
    }
}

impl<'a> QuatKey<'a> {
    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn value(&self) -> Quaternion {
        Quaternion::from_raw(&self.value)
    }
}

impl<'a> NodeAnim<'a> {
    pub fn name(&self) -> &str {
        self.node_name.as_ref()
    }

    pub fn num_position_keys(&self) -> u32 {
        self.num_position_keys
    }

    pub fn num_rotation_keys(&self) -> u32 {
        self.num_rotation_keys
    }

    pub fn num_scaling_keys(&self) -> u32 {
        self.num_scaling_keys
    }

    pub fn position_key_iter(&self) -> VectorKeyIter {
        VectorKeyIter::new(self.position_keys, self.num_position_keys as usize)
    }

    pub fn rotation_key_iter(&self) -> QuatKeyIter {
        QuatKeyIter::new(self.rotation_keys, self.num_rotation_keys as usize)
    }

    pub fn scaling_key_iter(&self) -> VectorKeyIter {
        VectorKeyIter::new(self.scaling_keys, self.num_scaling_keys as usize)
    }

    pub fn pre_state(&self) -> AiAnimBehaviour {
        self.pre_state
    }

    pub fn post_state(&self) -> AiAnimBehaviour {
        self.post_state
    }

    pub fn get_position_key(&self, id: usize) -> Option<VectorKey> {
        if id < self.num_position_keys as usize {
            unsafe { Some(VectorKey::from_raw(self.position_keys.offset(id as isize))) }
        } else {
            None
        }
    }

    pub fn get_rotation_key(&self, id: usize) -> Option<QuatKey> {
        if id < self.num_rotation_keys as usize {
            unsafe { Some(QuatKey::from_raw(self.rotation_keys.offset(id as isize))) }
        } else {
            None
        }
    }

    pub fn get_scaling_key(&self, id: usize) -> Option<VectorKey> {
        if id < self.num_scaling_keys as usize {
            unsafe { Some(VectorKey::from_raw(self.scaling_keys.offset(id as isize))) }
        } else {
            None
        }
    }
}

impl<'a> Animation<'a> {
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn duration(&self) -> f64 {
        self.duration
    }

    pub fn ticks_per_second(&self) -> f64 {
        self.ticks_per_second
    }

    pub fn num_channels(&self) -> u32 {
        self.num_channels
    }

    pub fn channel_iter(&self) -> NodeAnimIter {
        NodeAnimIter::new(self.channels as *const *const AiNodeAnim,
                          self.num_channels as usize)
    }

    pub fn get_node_anim(&self, id: usize) -> Option<NodeAnim> {
        if id < self.num_channels as usize {
            unsafe { Some(NodeAnim::from_raw(*(self.channels.offset(id as isize)))) }
        } else {
            None
        }
    }
}
