// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;
pub const COVARIANCE_TYPE_UNKNOWN: u8 = 0;
pub const COVARIANCE_TYPE_APPROXIMATED: u8 = 1;
pub const COVARIANCE_TYPE_DIAGONAL_KNOWN: u8 = 2;
pub const COVARIANCE_TYPE_KNOWN: u8 = 3;

extern "C" {
    fn sensor_msgs__msg__NavSatFix__init(msg: *mut NavSatFix) -> bool;
    fn sensor_msgs__msg__NavSatFix__fini(msg: *mut NavSatFix);
    fn sensor_msgs__msg__NavSatFix__Sequence__init(msg: *mut NavSatFixSequence, size: usize) -> bool;
    fn sensor_msgs__msg__NavSatFix__Sequence__fini(msg: *mut NavSatFixSequence);
    fn rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__NavSatFix() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct NavSatFix {
    pub header: std_msgs::msg::Header,
    pub status: NavSatStatus,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub position_covariance: [f64; 9],
    pub position_covariance_type: u8,
}

impl NavSatFix {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__NavSatFix__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for NavSatFix {
    fn drop(&mut self) {
        unsafe { sensor_msgs__msg__NavSatFix__fini(self) };
    }
}

impl TopicMsg for NavSatFix {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__NavSatFix()
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct NavSatFixSequence {
    data: *mut NavSatFix,
    size: usize,
    capacity: usize,
}

impl NavSatFixSequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__NavSatFix__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[NavSatFix]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [NavSatFix]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for NavSatFixSequence {
    fn drop(&mut self) {
        unsafe { sensor_msgs__msg__NavSatFix__Sequence__fini(self) };
    }
}

