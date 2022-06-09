// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn actionlib_msgs__msg__GoalStatusArray__init(msg: *mut GoalStatusArray) -> bool;
    fn actionlib_msgs__msg__GoalStatusArray__fini(msg: *mut GoalStatusArray);
    fn actionlib_msgs__msg__GoalStatusArray__Sequence__init(msg: *mut GoalStatusArraySequence, size: usize) -> bool;
    fn actionlib_msgs__msg__GoalStatusArray__Sequence__fini(msg: *mut GoalStatusArraySequence);
    fn rosidl_typesupport_c__get_message_type_support_handle__actionlib_msgs__msg__GoalStatusArray() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct GoalStatusArray {
    pub header: std_msgs::msg::Header,
    pub status_list: GoalStatusSequence,
}

impl GoalStatusArray {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { actionlib_msgs__msg__GoalStatusArray__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for GoalStatusArray {
    fn drop(&mut self) {
        unsafe { actionlib_msgs__msg__GoalStatusArray__fini(self) };
    }
}

impl TopicMsg for GoalStatusArray {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__actionlib_msgs__msg__GoalStatusArray()
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct GoalStatusArraySequence {
    data: *mut GoalStatusArray,
    size: usize,
    capacity: usize,
}

impl GoalStatusArraySequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { actionlib_msgs__msg__GoalStatusArray__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[GoalStatusArray]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [GoalStatusArray]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for GoalStatusArraySequence {
    fn drop(&mut self) {
        unsafe { actionlib_msgs__msg__GoalStatusArray__Sequence__fini(self) };
    }
}

