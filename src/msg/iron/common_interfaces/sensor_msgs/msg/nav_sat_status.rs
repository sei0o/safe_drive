// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;
pub const STATUS_NO_FIX: i8 = -1; // unable to fix position
pub const STATUS_FIX: i8 = 0; // unaugmented fix
pub const STATUS_SBAS_FIX: i8 = 1; // with satellite-based augmentation
pub const STATUS_GBAS_FIX: i8 = 2; // with ground-based augmentation
pub const SERVICE_GPS: u16 = 1;
pub const SERVICE_GLONASS: u16 = 2;
pub const SERVICE_COMPASS: u16 = 4; // includes BeiDou.
pub const SERVICE_GALILEO: u16 = 8;

extern "C" {
    fn sensor_msgs__msg__NavSatStatus__init(msg: *mut NavSatStatus) -> bool;
    fn sensor_msgs__msg__NavSatStatus__fini(msg: *mut NavSatStatus);
    fn sensor_msgs__msg__NavSatStatus__are_equal(lhs: *const NavSatStatus, rhs: *const NavSatStatus) -> bool;
    fn sensor_msgs__msg__NavSatStatus__Sequence__init(msg: *mut NavSatStatusSeqRaw, size: usize) -> bool;
    fn sensor_msgs__msg__NavSatStatus__Sequence__fini(msg: *mut NavSatStatusSeqRaw);
    fn sensor_msgs__msg__NavSatStatus__Sequence__are_equal(lhs: *const NavSatStatusSeqRaw, rhs: *const NavSatStatusSeqRaw) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__NavSatStatus() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct NavSatStatus {
    pub status: i8,
    pub service: u16,
}

impl NavSatStatus {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__NavSatStatus__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for NavSatStatus {
    fn drop(&mut self) {
        unsafe { sensor_msgs__msg__NavSatStatus__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct NavSatStatusSeqRaw {
    data: *mut NavSatStatus,
    size: usize,
    capacity: usize,
}

/// Sequence of NavSatStatus.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct NavSatStatusSeq<const N: usize> {
    data: *mut NavSatStatus,
    size: usize,
    capacity: usize,
}

impl<const N: usize> NavSatStatusSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: NavSatStatusSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__NavSatStatus__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn null() -> Self {
        let msg: NavSatStatusSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {data: msg.data, size: msg.size, capacity: msg.capacity }
    }

    pub fn as_slice(&self) -> &[NavSatStatus] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [NavSatStatus] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, NavSatStatus> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, NavSatStatus> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for NavSatStatusSeq<N> {
    fn drop(&mut self) {
        let mut msg = NavSatStatusSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { sensor_msgs__msg__NavSatStatus__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for NavSatStatusSeq<N> {}
unsafe impl<const N: usize> Sync for NavSatStatusSeq<N> {}


impl TypeSupport for NavSatStatus {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__NavSatStatus()
        }
    }
}

impl PartialEq for NavSatStatus {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            sensor_msgs__msg__NavSatStatus__are_equal(self, other)
        }
    }
}

impl<const N: usize> PartialEq for NavSatStatusSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = NavSatStatusSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
            let msg2 = NavSatStatusSeqRaw{data: other.data, size: other.size, capacity: other.capacity};
            sensor_msgs__msg__NavSatStatus__Sequence__are_equal(&msg1, &msg2)
        }
    }
}
