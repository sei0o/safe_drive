// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn statistics_msgs__msg__MetricsMessage__init(msg: *mut MetricsMessage) -> bool;
    fn statistics_msgs__msg__MetricsMessage__fini(msg: *mut MetricsMessage);
    fn statistics_msgs__msg__MetricsMessage__are_equal(lhs: *const MetricsMessage, rhs: *const MetricsMessage) -> bool;
    fn statistics_msgs__msg__MetricsMessage__Sequence__init(msg: *mut MetricsMessageSeqRaw, size: usize) -> bool;
    fn statistics_msgs__msg__MetricsMessage__Sequence__fini(msg: *mut MetricsMessageSeqRaw);
    fn statistics_msgs__msg__MetricsMessage__Sequence__are_equal(lhs: *const MetricsMessageSeqRaw, rhs: *const MetricsMessageSeqRaw) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__statistics_msgs__msg__MetricsMessage() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct MetricsMessage {
    pub measurement_source_name: crate::msg::RosString<0>,
    pub metrics_source: crate::msg::RosString<0>,
    pub unit: crate::msg::RosString<0>,
    pub window_start: builtin_interfaces::UnsafeTime,
    pub window_stop: builtin_interfaces::UnsafeTime,
    pub statistics: StatisticDataPointSeq<0>,
}

impl MetricsMessage {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { statistics_msgs__msg__MetricsMessage__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for MetricsMessage {
    fn drop(&mut self) {
        unsafe { statistics_msgs__msg__MetricsMessage__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct MetricsMessageSeqRaw {
    data: *mut MetricsMessage,
    size: usize,
    capacity: usize,
}

/// Sequence of MetricsMessage.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct MetricsMessageSeq<const N: usize> {
    data: *mut MetricsMessage,
    size: usize,
    capacity: usize,
}

impl<const N: usize> MetricsMessageSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: MetricsMessageSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { statistics_msgs__msg__MetricsMessage__Sequence__init(&mut msg, size) } {
            Some(Self {data: msg.data, size: msg.size, capacity: msg.capacity })
        } else {
            None
        }
    }

    pub fn null() -> Self {
        let msg: MetricsMessageSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {data: msg.data, size: msg.size, capacity: msg.capacity }
    }

    pub fn as_slice(&self) -> &[MetricsMessage] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [MetricsMessage] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, MetricsMessage> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, MetricsMessage> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for MetricsMessageSeq<N> {
    fn drop(&mut self) {
        let mut msg = MetricsMessageSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
        unsafe { statistics_msgs__msg__MetricsMessage__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for MetricsMessageSeq<N> {}
unsafe impl<const N: usize> Sync for MetricsMessageSeq<N> {}


impl TypeSupport for MetricsMessage {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__statistics_msgs__msg__MetricsMessage()
        }
    }
}

impl PartialEq for MetricsMessage {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            statistics_msgs__msg__MetricsMessage__are_equal(self, other)
        }
    }
}

impl<const N: usize> PartialEq for MetricsMessageSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = MetricsMessageSeqRaw{data: self.data, size: self.size, capacity: self.capacity};
            let msg2 = MetricsMessageSeqRaw{data: other.data, size: other.size, capacity: other.capacity};
            statistics_msgs__msg__MetricsMessage__Sequence__are_equal(&msg1, &msg2)
        }
    }
}

