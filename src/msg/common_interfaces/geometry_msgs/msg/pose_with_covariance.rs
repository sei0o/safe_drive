// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;

extern "C" {
    fn geometry_msgs__msg__PoseWithCovariance__init(msg: *mut PoseWithCovariance) -> bool;
    fn geometry_msgs__msg__PoseWithCovariance__fini(msg: *mut PoseWithCovariance);
    fn geometry_msgs__msg__PoseWithCovariance__Sequence__init(msg: *mut PoseWithCovarianceSequence, size: usize) -> bool;
    fn geometry_msgs__msg__PoseWithCovariance__Sequence__fini(msg: *mut PoseWithCovarianceSequence);
}


#[repr(C)]
#[derive(Debug)]
pub struct PoseWithCovariance {
    pub pose: super::Pose,
    pub covariance: [f64; 36],
}

impl PoseWithCovariance {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { geometry_msgs__msg__PoseWithCovariance__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for PoseWithCovariance {
    fn drop(&mut self) {
        unsafe { geometry_msgs__msg__PoseWithCovariance__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct PoseWithCovarianceSequence {
    data: *mut PoseWithCovariance,
    size: usize,
    capacity: usize,
}

impl PoseWithCovarianceSequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { geometry_msgs__msg__PoseWithCovariance__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[PoseWithCovariance]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [PoseWithCovariance]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for PoseWithCovarianceSequence {
    fn drop(&mut self) {
        unsafe { geometry_msgs__msg__PoseWithCovariance__Sequence__fini(self) };
    }
}
