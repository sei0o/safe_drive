// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;
pub const BOX: u8 = 1;
pub const SPHERE: u8 = 2;
pub const CYLINDER: u8 = 3;
pub const CONE: u8 = 4;
pub const BOX_X: u8 = 0;
pub const BOX_Y: u8 = 1;
pub const BOX_Z: u8 = 2;
pub const SPHERE_RADIUS: u8 = 0;
pub const CYLINDER_HEIGHT: u8 = 0;
pub const CYLINDER_RADIUS: u8 = 1;
pub const CONE_HEIGHT: u8 = 0;
pub const CONE_RADIUS: u8 = 1;

extern "C" {
    fn shape_msgs__msg__SolidPrimitive__init(msg: *mut SolidPrimitive) -> bool;
    fn shape_msgs__msg__SolidPrimitive__fini(msg: *mut SolidPrimitive);
    fn shape_msgs__msg__SolidPrimitive__Sequence__init(msg: *mut SolidPrimitiveSequence, size: usize) -> bool;
    fn shape_msgs__msg__SolidPrimitive__Sequence__fini(msg: *mut SolidPrimitiveSequence);
    fn rosidl_typesupport_c__get_message_type_support_handle__shape_msgs__msg__SolidPrimitive() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct SolidPrimitive {
    pub type_: u8,
    pub dimensions: crate::msg::F64Seq<3>,
}

impl SolidPrimitive {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { shape_msgs__msg__SolidPrimitive__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for SolidPrimitive {
    fn drop(&mut self) {
        unsafe { shape_msgs__msg__SolidPrimitive__fini(self) };
    }
}

impl TopicMsg for SolidPrimitive {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__shape_msgs__msg__SolidPrimitive()
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct SolidPrimitiveSequence {
    data: *mut SolidPrimitive,
    size: usize,
    capacity: usize,
}

impl SolidPrimitiveSequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { shape_msgs__msg__SolidPrimitive__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[SolidPrimitive]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [SolidPrimitive]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for SolidPrimitiveSequence {
    fn drop(&mut self) {
        unsafe { shape_msgs__msg__SolidPrimitive__Sequence__fini(self) };
    }
}

