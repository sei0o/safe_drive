// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn sensor_msgs__msg__LaserScan__init(msg: *mut LaserScan) -> bool;
    fn sensor_msgs__msg__LaserScan__fini(msg: *mut LaserScan);
    fn sensor_msgs__msg__LaserScan__Sequence__init(msg: *mut LaserScanSeqRaw, size: usize) -> bool;
    fn sensor_msgs__msg__LaserScan__Sequence__fini(msg: *mut LaserScanSeqRaw);
    fn rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__LaserScan() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct LaserScan {
    pub header: std_msgs::msg::Header,
    pub angle_min: f32,
    pub angle_max: f32,
    pub angle_increment: f32,
    pub time_increment: f32,
    pub scan_time: f32,
    pub range_min: f32,
    pub range_max: f32,
    pub ranges: crate::msg::F32Seq<0>,
    pub intensities: crate::msg::F32Seq<0>,
}

impl LaserScan {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__LaserScan__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for LaserScan {
    fn drop(&mut self) {
        unsafe { sensor_msgs__msg__LaserScan__fini(self) };
    }
}


struct LaserScanSeqRaw {
    data: *mut LaserScan,
    size: usize,
    capacity: usize,
}

/// Sequence of LaserScan.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct LaserScanSeq<const N: usize> {
    data: *mut LaserScan,
    size: usize,
    capacity: usize,
}

impl<const N: usize> LaserScanSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: LaserScanSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__LaserScan__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[LaserScan]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [LaserScan]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl<const N: usize> Drop for LaserScanSeq<N> {
    fn drop(&mut self) {
        let mut msg = LaserScanSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { sensor_msgs__msg__LaserScan__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for LaserScanSeq<N> {}
unsafe impl<const N: usize> Sync for LaserScanSeq<N> {}


impl TopicMsg for LaserScan {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__LaserScan()
        }
    }
}