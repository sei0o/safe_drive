// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::super::*;
use super::*;
use crate::msg::*;
use crate::rcl;
pub const STATUS_UNKNOWN: i8 = 0;
pub const STATUS_ACCEPTED: i8 = 1;
pub const STATUS_EXECUTING: i8 = 2;
pub const STATUS_CANCELING: i8 = 3;
pub const STATUS_SUCCEEDED: i8 = 4;
pub const STATUS_CANCELED: i8 = 5;
pub const STATUS_ABORTED: i8 = 6;

extern "C" {
    fn action_msgs__msg__GoalStatus__init(msg: *mut GoalStatus) -> bool;
    fn action_msgs__msg__GoalStatus__fini(msg: *mut GoalStatus);
    fn action_msgs__msg__GoalStatus__are_equal(
        lhs: *const GoalStatus,
        rhs: *const GoalStatus,
    ) -> bool;
    fn action_msgs__msg__GoalStatus__Sequence__init(
        msg: *mut GoalStatusSeqRaw,
        size: usize,
    ) -> bool;
    fn action_msgs__msg__GoalStatus__Sequence__fini(msg: *mut GoalStatusSeqRaw);
    fn action_msgs__msg__GoalStatus__Sequence__are_equal(
        lhs: *const GoalStatusSeqRaw,
        rhs: *const GoalStatusSeqRaw,
    ) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__action_msgs__msg__GoalStatus(
    ) -> *const rcl::rosidl_message_type_support_t;
}

#[repr(C)]
#[derive(Debug)]
pub struct GoalStatus {
    pub goal_info: GoalInfo,
    pub status: i8,
}

impl GoalStatus {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { action_msgs__msg__GoalStatus__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for GoalStatus {
    fn drop(&mut self) {
        unsafe { action_msgs__msg__GoalStatus__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct GoalStatusSeqRaw {
    data: *mut GoalStatus,
    size: usize,
    capacity: usize,
}

/// Sequence of GoalStatus.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct GoalStatusSeq<const N: usize> {
    data: *mut GoalStatus,
    size: usize,
    capacity: usize,
}

impl<const N: usize> GoalStatusSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: GoalStatusSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { action_msgs__msg__GoalStatus__Sequence__init(&mut msg, size) } {
            Some(Self {
                data: msg.data,
                size: msg.size,
                capacity: msg.capacity,
            })
        } else {
            None
        }
    }

    pub fn null() -> Self {
        let msg: GoalStatusSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {
            data: msg.data,
            size: msg.size,
            capacity: msg.capacity,
        }
    }

    pub fn as_slice(&self) -> &[GoalStatus] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [GoalStatus] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, GoalStatus> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, GoalStatus> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for GoalStatusSeq<N> {
    fn drop(&mut self) {
        let mut msg = GoalStatusSeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { action_msgs__msg__GoalStatus__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for GoalStatusSeq<N> {}
unsafe impl<const N: usize> Sync for GoalStatusSeq<N> {}

impl TypeSupport for GoalStatus {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__action_msgs__msg__GoalStatus()
        }
    }
}

impl PartialEq for GoalStatus {
    fn eq(&self, other: &Self) -> bool {
        unsafe { action_msgs__msg__GoalStatus__are_equal(self, other) }
    }
}

impl<const N: usize> PartialEq for GoalStatusSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = GoalStatusSeqRaw {
                data: self.data,
                size: self.size,
                capacity: self.capacity,
            };
            let msg2 = GoalStatusSeqRaw {
                data: other.data,
                size: other.size,
                capacity: other.capacity,
            };
            action_msgs__msg__GoalStatus__Sequence__are_equal(&msg1, &msg2)
        }
    }
}
