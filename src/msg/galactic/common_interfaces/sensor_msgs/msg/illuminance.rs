// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn sensor_msgs__msg__Illuminance__init(msg: *mut Illuminance) -> bool;
    fn sensor_msgs__msg__Illuminance__fini(msg: *mut Illuminance);
    fn sensor_msgs__msg__Illuminance__Sequence__init(msg: *mut IlluminanceSequence, size: usize) -> bool;
    fn sensor_msgs__msg__Illuminance__Sequence__fini(msg: *mut IlluminanceSequence);
    fn rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__Illuminance() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct Illuminance {
    pub header: std_msgs::msg::Header,
    pub illuminance: f64,
    pub variance: f64,
}

impl Illuminance {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__Illuminance__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for Illuminance {
    fn drop(&mut self) {
        unsafe { sensor_msgs__msg__Illuminance__fini(self) };
    }
}

impl TopicMsg for Illuminance {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__Illuminance()
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct IlluminanceSequence {
    data: *mut Illuminance,
    size: usize,
    capacity: usize,
}

impl IlluminanceSequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__Illuminance__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[Illuminance]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [Illuminance]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for IlluminanceSequence {
    fn drop(&mut self) {
        unsafe { sensor_msgs__msg__Illuminance__Sequence__fini(self) };
    }
}

