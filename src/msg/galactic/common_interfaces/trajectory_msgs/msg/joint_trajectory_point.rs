// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn trajectory_msgs__msg__JointTrajectoryPoint__init(msg: *mut JointTrajectoryPoint) -> bool;
    fn trajectory_msgs__msg__JointTrajectoryPoint__fini(msg: *mut JointTrajectoryPoint);
    fn trajectory_msgs__msg__JointTrajectoryPoint__Sequence__init(msg: *mut JointTrajectoryPointSequence, size: usize) -> bool;
    fn trajectory_msgs__msg__JointTrajectoryPoint__Sequence__fini(msg: *mut JointTrajectoryPointSequence);
    fn rosidl_typesupport_c__get_message_type_support_handle__trajectory_msgs__msg__JointTrajectoryPoint() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct JointTrajectoryPoint {
    pub positions: crate::msg::F64Seq<0>,
    pub velocities: crate::msg::F64Seq<0>,
    pub accelerations: crate::msg::F64Seq<0>,
    pub effort: crate::msg::F64Seq<0>,
    pub time_from_start: builtin_interfaces__msg__Duration,
}

impl JointTrajectoryPoint {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { trajectory_msgs__msg__JointTrajectoryPoint__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for JointTrajectoryPoint {
    fn drop(&mut self) {
        unsafe { trajectory_msgs__msg__JointTrajectoryPoint__fini(self) };
    }
}

impl TopicMsg for JointTrajectoryPoint {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__trajectory_msgs__msg__JointTrajectoryPoint()
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct JointTrajectoryPointSequence {
    data: *mut JointTrajectoryPoint,
    size: usize,
    capacity: usize,
}

impl JointTrajectoryPointSequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { trajectory_msgs__msg__JointTrajectoryPoint__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[JointTrajectoryPoint]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [JointTrajectoryPoint]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for JointTrajectoryPointSequence {
    fn drop(&mut self) {
        unsafe { trajectory_msgs__msg__JointTrajectoryPoint__Sequence__fini(self) };
    }
}

