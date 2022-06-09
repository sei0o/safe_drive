// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn std_msgs__msg__Int8MultiArray__init(msg: *mut Int8MultiArray) -> bool;
    fn std_msgs__msg__Int8MultiArray__fini(msg: *mut Int8MultiArray);
    fn std_msgs__msg__Int8MultiArray__Sequence__init(msg: *mut Int8MultiArraySequence, size: usize) -> bool;
    fn std_msgs__msg__Int8MultiArray__Sequence__fini(msg: *mut Int8MultiArraySequence);
    fn rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__Int8MultiArray() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct Int8MultiArray {
    pub layout: MultiArrayLayout,
    pub data: crate::msg::I8Seq<0>,
}

impl Int8MultiArray {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { std_msgs__msg__Int8MultiArray__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for Int8MultiArray {
    fn drop(&mut self) {
        unsafe { std_msgs__msg__Int8MultiArray__fini(self) };
    }
}

impl TopicMsg for Int8MultiArray {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__Int8MultiArray()
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Int8MultiArraySequence {
    data: *mut Int8MultiArray,
    size: usize,
    capacity: usize,
}

impl Int8MultiArraySequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { std_msgs__msg__Int8MultiArray__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[Int8MultiArray]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [Int8MultiArray]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for Int8MultiArraySequence {
    fn drop(&mut self) {
        unsafe { std_msgs__msg__Int8MultiArray__Sequence__fini(self) };
    }
}

