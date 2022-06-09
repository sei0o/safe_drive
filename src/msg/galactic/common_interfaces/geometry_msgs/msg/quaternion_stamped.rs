// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn geometry_msgs__msg__QuaternionStamped__init(msg: *mut QuaternionStamped) -> bool;
    fn geometry_msgs__msg__QuaternionStamped__fini(msg: *mut QuaternionStamped);
    fn geometry_msgs__msg__QuaternionStamped__Sequence__init(msg: *mut QuaternionStampedSequence, size: usize) -> bool;
    fn geometry_msgs__msg__QuaternionStamped__Sequence__fini(msg: *mut QuaternionStampedSequence);
    fn rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__QuaternionStamped() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct QuaternionStamped {
    pub header: std_msgs::msg::Header,
    pub quaternion: Quaternion,
}

impl QuaternionStamped {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { geometry_msgs__msg__QuaternionStamped__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for QuaternionStamped {
    fn drop(&mut self) {
        unsafe { geometry_msgs__msg__QuaternionStamped__fini(self) };
    }
}

impl TopicMsg for QuaternionStamped {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__QuaternionStamped()
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct QuaternionStampedSequence {
    data: *mut QuaternionStamped,
    size: usize,
    capacity: usize,
}

impl QuaternionStampedSequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { geometry_msgs__msg__QuaternionStamped__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[QuaternionStamped]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [QuaternionStamped]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for QuaternionStampedSequence {
    fn drop(&mut self) {
        unsafe { geometry_msgs__msg__QuaternionStamped__Sequence__fini(self) };
    }
}

