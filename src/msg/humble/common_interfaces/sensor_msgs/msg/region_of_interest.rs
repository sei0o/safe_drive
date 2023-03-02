// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn sensor_msgs__msg__RegionOfInterest__init(msg: *mut RegionOfInterest) -> bool;
    fn sensor_msgs__msg__RegionOfInterest__fini(msg: *mut RegionOfInterest);
    fn sensor_msgs__msg__RegionOfInterest__are_equal(lhs: *const RegionOfInterest, rhs: *const RegionOfInterest) -> bool;
    fn sensor_msgs__msg__RegionOfInterest__Sequence__init(msg: *mut RegionOfInterestSeqRaw, size: usize) -> bool;
    fn sensor_msgs__msg__RegionOfInterest__Sequence__fini(msg: *mut RegionOfInterestSeqRaw);
    fn sensor_msgs__msg__RegionOfInterest__Sequence__are_equal(lhs: *const RegionOfInterestSeqRaw, rhs: *const RegionOfInterestSeqRaw) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__RegionOfInterest() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct RegionOfInterest {
    pub x_offset: u32,
    pub y_offset: u32,
    pub height: u32,
    pub width: u32,
    pub do_rectify: bool,
}

impl RegionOfInterest {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__RegionOfInterest__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for RegionOfInterest {
    fn drop(&mut self) {
        unsafe { sensor_msgs__msg__RegionOfInterest__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct RegionOfInterestSeqRaw {
    data: *mut RegionOfInterest,
    size: usize,
    capacity: usize,
}

/// Sequence of RegionOfInterest.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct RegionOfInterestSeq<const N: usize> {
    data: *mut RegionOfInterest,
    size: usize,
    capacity: usize,
}

impl<const N: usize> RegionOfInterestSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: RegionOfInterestSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__RegionOfInterest__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn null() -> Self {
        let msg: RegionOfInterestSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {data: msg.data, size: msg.size, capacity: msg.capacity }
    }

    pub fn as_slice(&self) -> &[RegionOfInterest] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [RegionOfInterest] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, RegionOfInterest> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, RegionOfInterest> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for RegionOfInterestSeq<N> {
    fn drop(&mut self) {
        let mut msg = RegionOfInterestSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { sensor_msgs__msg__RegionOfInterest__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for RegionOfInterestSeq<N> {}
unsafe impl<const N: usize> Sync for RegionOfInterestSeq<N> {}


impl TypeSupport for RegionOfInterest {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__RegionOfInterest()
        }
    }
}

impl PartialEq for RegionOfInterest {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            sensor_msgs__msg__RegionOfInterest__are_equal(self, other)
        }
    }
}

impl<const N: usize> PartialEq for RegionOfInterestSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = RegionOfInterestSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
            let msg2 = RegionOfInterestSeqRaw{data: other.data, size: other.size, capacity: other.capacity};
            sensor_msgs__msg__RegionOfInterest__Sequence__are_equal(&msg1, &msg2)
        }
    }
}

