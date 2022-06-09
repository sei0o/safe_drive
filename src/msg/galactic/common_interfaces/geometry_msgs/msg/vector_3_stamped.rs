// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn geometry_msgs__msg__Vector3Stamped__init(msg: *mut Vector3Stamped) -> bool;
    fn geometry_msgs__msg__Vector3Stamped__fini(msg: *mut Vector3Stamped);
    fn geometry_msgs__msg__Vector3Stamped__Sequence__init(msg: *mut Vector3StampedSequence, size: usize) -> bool;
    fn geometry_msgs__msg__Vector3Stamped__Sequence__fini(msg: *mut Vector3StampedSequence);
    fn rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__Vector3Stamped() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct Vector3Stamped {
    pub header: std_msgs::msg::Header,
    pub vector: Vector3,
}

impl Vector3Stamped {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { geometry_msgs__msg__Vector3Stamped__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for Vector3Stamped {
    fn drop(&mut self) {
        unsafe { geometry_msgs__msg__Vector3Stamped__fini(self) };
    }
}

impl TopicMsg for Vector3Stamped {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__Vector3Stamped()
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Vector3StampedSequence {
    data: *mut Vector3Stamped,
    size: usize,
    capacity: usize,
}

impl Vector3StampedSequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { geometry_msgs__msg__Vector3Stamped__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[Vector3Stamped]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [Vector3Stamped]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for Vector3StampedSequence {
    fn drop(&mut self) {
        unsafe { geometry_msgs__msg__Vector3Stamped__Sequence__fini(self) };
    }
}

