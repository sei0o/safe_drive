// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
pub const KEEP_ALIVE: u8 = 0;
pub const POSE_UPDATE: u8 = 1;
pub const MENU_SELECT: u8 = 2;
pub const BUTTON_CLICK: u8 = 3;
pub const MOUSE_DOWN: u8 = 4;
pub const MOUSE_UP: u8 = 5;

extern "C" {
    fn visualization_msgs__msg__InteractiveMarkerFeedback__init(msg: *mut InteractiveMarkerFeedback) -> bool;
    fn visualization_msgs__msg__InteractiveMarkerFeedback__fini(msg: *mut InteractiveMarkerFeedback);
    fn visualization_msgs__msg__InteractiveMarkerFeedback__Sequence__init(msg: *mut InteractiveMarkerFeedbackSequence, size: usize) -> bool;
    fn visualization_msgs__msg__InteractiveMarkerFeedback__Sequence__fini(msg: *mut InteractiveMarkerFeedbackSequence);
}


#[repr(C)]
#[derive(Debug)]
pub struct InteractiveMarkerFeedback {
    pub header: std_msgs::msg::Header,
    pub client_id: crate::msg::RosString<0>,
    pub marker_name: crate::msg::RosString<0>,
    pub control_name: crate::msg::RosString<0>,
    pub event_type: u8,
    pub pose: geometry_msgs::msg::Pose,
    pub menu_entry_id: u32,
    pub mouse_point: geometry_msgs::msg::Point,
    pub mouse_point_valid: bool,
}

impl InteractiveMarkerFeedback {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { visualization_msgs__msg__InteractiveMarkerFeedback__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for InteractiveMarkerFeedback {
    fn drop(&mut self) {
        unsafe { visualization_msgs__msg__InteractiveMarkerFeedback__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct InteractiveMarkerFeedbackSequence {
    data: *mut InteractiveMarkerFeedback,
    size: usize,
    capacity: usize,
}

impl InteractiveMarkerFeedbackSequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { visualization_msgs__msg__InteractiveMarkerFeedback__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[InteractiveMarkerFeedback]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [InteractiveMarkerFeedback]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for InteractiveMarkerFeedbackSequence {
    fn drop(&mut self) {
        unsafe { visualization_msgs__msg__InteractiveMarkerFeedback__Sequence__fini(self) };
    }
}
