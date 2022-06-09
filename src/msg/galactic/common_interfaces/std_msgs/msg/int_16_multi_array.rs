// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn std_msgs__msg__Int16MultiArray__init(msg: *mut Int16MultiArray) -> bool;
    fn std_msgs__msg__Int16MultiArray__fini(msg: *mut Int16MultiArray);
    fn std_msgs__msg__Int16MultiArray__Sequence__init(msg: *mut Int16MultiArraySequence, size: usize) -> bool;
    fn std_msgs__msg__Int16MultiArray__Sequence__fini(msg: *mut Int16MultiArraySequence);
    fn rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__Int16MultiArray() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct Int16MultiArray {
    pub layout: MultiArrayLayout,
    pub data: crate::msg::I16Seq<0>,
}

impl Int16MultiArray {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { std_msgs__msg__Int16MultiArray__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for Int16MultiArray {
    fn drop(&mut self) {
        unsafe { std_msgs__msg__Int16MultiArray__fini(self) };
    }
}

impl TopicMsg for Int16MultiArray {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__std_msgs__msg__Int16MultiArray()
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Int16MultiArraySequence {
    data: *mut Int16MultiArray,
    size: usize,
    capacity: usize,
}

impl Int16MultiArraySequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { std_msgs__msg__Int16MultiArray__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[Int16MultiArray]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [Int16MultiArray]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for Int16MultiArraySequence {
    fn drop(&mut self) {
        unsafe { std_msgs__msg__Int16MultiArray__Sequence__fini(self) };
    }
}

