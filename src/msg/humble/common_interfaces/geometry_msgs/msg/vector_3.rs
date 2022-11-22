// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::super::*;
use super::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn geometry_msgs__msg__Vector3__init(msg: *mut Vector3) -> bool;
    fn geometry_msgs__msg__Vector3__fini(msg: *mut Vector3);
    fn geometry_msgs__msg__Vector3__are_equal(lhs: *const Vector3, rhs: *const Vector3) -> bool;
    fn geometry_msgs__msg__Vector3__Sequence__init(msg: *mut Vector3SeqRaw, size: usize) -> bool;
    fn geometry_msgs__msg__Vector3__Sequence__fini(msg: *mut Vector3SeqRaw);
    fn geometry_msgs__msg__Vector3__Sequence__are_equal(
        lhs: *const Vector3SeqRaw,
        rhs: *const Vector3SeqRaw,
    ) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__Vector3(
    ) -> *const rcl::rosidl_message_type_support_t;
}

#[repr(C)]
#[derive(Debug)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { geometry_msgs__msg__Vector3__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for Vector3 {
    fn drop(&mut self) {
        unsafe { geometry_msgs__msg__Vector3__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct Vector3SeqRaw {
    data: *mut Vector3,
    size: usize,
    capacity: usize,
}

/// Sequence of Vector3.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct Vector3Seq<const N: usize> {
    data: *mut Vector3,
    size: usize,
    capacity: usize,
}

impl<const N: usize> Vector3Seq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: Vector3SeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { geometry_msgs__msg__Vector3__Sequence__init(&mut msg, size) } {
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
        let msg: Vector3SeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {
            data: msg.data,
            size: msg.size,
            capacity: msg.capacity,
        }
    }

    pub fn as_slice(&self) -> &[Vector3] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [Vector3] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Vector3> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Vector3> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for Vector3Seq<N> {
    fn drop(&mut self) {
        let mut msg = Vector3SeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { geometry_msgs__msg__Vector3__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for Vector3Seq<N> {}
unsafe impl<const N: usize> Sync for Vector3Seq<N> {}

impl TypeSupport for Vector3 {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__geometry_msgs__msg__Vector3()
        }
    }
}

impl PartialEq for Vector3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { geometry_msgs__msg__Vector3__are_equal(self, other) }
    }
}

impl<const N: usize> PartialEq for Vector3Seq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = Vector3SeqRaw {
                data: self.data,
                size: self.size,
                capacity: self.capacity,
            };
            let msg2 = Vector3SeqRaw {
                data: other.data,
                size: other.size,
                capacity: other.capacity,
            };
            geometry_msgs__msg__Vector3__Sequence__are_equal(&msg1, &msg2)
        }
    }
}