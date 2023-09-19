// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::super::*;
use super::*;
use crate::msg::*;
use crate::rcl;
pub const FIELD_TYPE_NOT_SET: u8 = 0;
pub const FIELD_TYPE_NESTED_TYPE: u8 = 1;
pub const FIELD_TYPE_FLOAT: u8 = 10;
pub const FIELD_TYPE_DOUBLE: u8 = 11;
pub const FIELD_TYPE_LONG_DOUBLE: u8 = 12;
pub const FIELD_TYPE_CHAR: u8 = 13;
pub const FIELD_TYPE_WCHAR: u8 = 14;
pub const FIELD_TYPE_BOOLEAN: u8 = 15;
pub const FIELD_TYPE_BYTE: u8 = 16;
pub const FIELD_TYPE_STRING: u8 = 17;
pub const FIELD_TYPE_WSTRING: u8 = 18;
pub const FIELD_TYPE_FIXED_STRING: u8 = 19;
pub const FIELD_TYPE_FIXED_WSTRING: u8 = 20;
pub const FIELD_TYPE_BOUNDED_STRING: u8 = 21;
pub const FIELD_TYPE_BOUNDED_WSTRING: u8 = 22;
pub const FIELD_TYPE_NESTED_TYPE_ARRAY: u8 = 49;
pub const FIELD_TYPE_FLOAT_ARRAY: u8 = 58;
pub const FIELD_TYPE_DOUBLE_ARRAY: u8 = 59;
pub const FIELD_TYPE_LONG_DOUBLE_ARRAY: u8 = 60;
pub const FIELD_TYPE_CHAR_ARRAY: u8 = 61;
pub const FIELD_TYPE_WCHAR_ARRAY: u8 = 62;
pub const FIELD_TYPE_BOOLEAN_ARRAY: u8 = 63;
pub const FIELD_TYPE_BYTE_ARRAY: u8 = 64;
pub const FIELD_TYPE_STRING_ARRAY: u8 = 65;
pub const FIELD_TYPE_WSTRING_ARRAY: u8 = 66;
pub const FIELD_TYPE_FIXED_STRING_ARRAY: u8 = 67;
pub const FIELD_TYPE_FIXED_WSTRING_ARRAY: u8 = 68;
pub const FIELD_TYPE_BOUNDED_STRING_ARRAY: u8 = 69;
pub const FIELD_TYPE_BOUNDED_WSTRING_ARRAY: u8 = 70;
pub const FIELD_TYPE_NESTED_TYPE_BOUNDED_SEQUENCE: u8 = 97;
pub const FIELD_TYPE_FLOAT_BOUNDED_SEQUENCE: u8 = 106;
pub const FIELD_TYPE_DOUBLE_BOUNDED_SEQUENCE: u8 = 107;
pub const FIELD_TYPE_LONG_DOUBLE_BOUNDED_SEQUENCE: u8 = 108;
pub const FIELD_TYPE_CHAR_BOUNDED_SEQUENCE: u8 = 109;
pub const FIELD_TYPE_WCHAR_BOUNDED_SEQUENCE: u8 = 110;
pub const FIELD_TYPE_BOOLEAN_BOUNDED_SEQUENCE: u8 = 111;
pub const FIELD_TYPE_BYTE_BOUNDED_SEQUENCE: u8 = 112;
pub const FIELD_TYPE_STRING_BOUNDED_SEQUENCE: u8 = 113;
pub const FIELD_TYPE_WSTRING_BOUNDED_SEQUENCE: u8 = 114;
pub const FIELD_TYPE_FIXED_STRING_BOUNDED_SEQUENCE: u8 = 115;
pub const FIELD_TYPE_FIXED_WSTRING_BOUNDED_SEQUENCE: u8 = 116;
pub const FIELD_TYPE_BOUNDED_STRING_BOUNDED_SEQUENCE: u8 = 117;
pub const FIELD_TYPE_BOUNDED_WSTRING_BOUNDED_SEQUENCE: u8 = 118;
pub const FIELD_TYPE_NESTED_TYPE_UNBOUNDED_SEQUENCE: u8 = 145;
pub const FIELD_TYPE_FLOAT_UNBOUNDED_SEQUENCE: u8 = 154;
pub const FIELD_TYPE_DOUBLE_UNBOUNDED_SEQUENCE: u8 = 155;
pub const FIELD_TYPE_LONG_DOUBLE_UNBOUNDED_SEQUENCE: u8 = 156;
pub const FIELD_TYPE_CHAR_UNBOUNDED_SEQUENCE: u8 = 157;
pub const FIELD_TYPE_WCHAR_UNBOUNDED_SEQUENCE: u8 = 158;
pub const FIELD_TYPE_BOOLEAN_UNBOUNDED_SEQUENCE: u8 = 159;
pub const FIELD_TYPE_BYTE_UNBOUNDED_SEQUENCE: u8 = 160;
pub const FIELD_TYPE_STRING_UNBOUNDED_SEQUENCE: u8 = 161;
pub const FIELD_TYPE_WSTRING_UNBOUNDED_SEQUENCE: u8 = 162;
pub const FIELD_TYPE_FIXED_STRING_UNBOUNDED_SEQUENCE: u8 = 163;
pub const FIELD_TYPE_FIXED_WSTRING_UNBOUNDED_SEQUENCE: u8 = 164;
pub const FIELD_TYPE_BOUNDED_STRING_UNBOUNDED_SEQUENCE: u8 = 165;
pub const FIELD_TYPE_BOUNDED_WSTRING_UNBOUNDED_SEQUENCE: u8 = 166;

extern "C" {
    fn type_description_interfaces__msg__FieldType__init(msg: *mut FieldType) -> bool;
    fn type_description_interfaces__msg__FieldType__fini(msg: *mut FieldType);
    fn type_description_interfaces__msg__FieldType__are_equal(
        lhs: *const FieldType,
        rhs: *const FieldType,
    ) -> bool;
    fn type_description_interfaces__msg__FieldType__Sequence__init(
        msg: *mut FieldTypeSeqRaw,
        size: usize,
    ) -> bool;
    fn type_description_interfaces__msg__FieldType__Sequence__fini(msg: *mut FieldTypeSeqRaw);
    fn type_description_interfaces__msg__FieldType__Sequence__are_equal(
        lhs: *const FieldTypeSeqRaw,
        rhs: *const FieldTypeSeqRaw,
    ) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__type_description_interfaces__msg__FieldType(
    ) -> *const rcl::rosidl_message_type_support_t;
}

#[repr(C)]
#[derive(Debug)]
pub struct FieldType {
    pub FIELD_TYPE_INT8: u8,
    pub FIELD_TYPE_UINT8: u8,
    pub FIELD_TYPE_INT16: u8,
    pub FIELD_TYPE_UINT16: u8,
    pub FIELD_TYPE_INT32: u8,
    pub FIELD_TYPE_UINT32: u8,
    pub FIELD_TYPE_INT64: u8,
    pub FIELD_TYPE_UINT64: u8,
    pub FIELD_TYPE_INT8_ARRAY: u8,
    pub FIELD_TYPE_UINT8_ARRAY: u8,
    pub FIELD_TYPE_INT16_ARRAY: u8,
    pub FIELD_TYPE_UINT16_ARRAY: u8,
    pub FIELD_TYPE_INT32_ARRAY: u8,
    pub FIELD_TYPE_UINT32_ARRAY: u8,
    pub FIELD_TYPE_INT64_ARRAY: u8,
    pub FIELD_TYPE_UINT64_ARRAY: u8,
    pub FIELD_TYPE_INT8_BOUNDED_SEQUENCE: u8,
    pub FIELD_TYPE_UINT8_BOUNDED_SEQUENCE: u8,
    pub FIELD_TYPE_INT16_BOUNDED_SEQUENCE: u8,
    pub FIELD_TYPE_UINT16_BOUNDED_SEQUENCE: u8,
    pub FIELD_TYPE_INT32_BOUNDED_SEQUENCE: u8,
    pub FIELD_TYPE_UINT32_BOUNDED_SEQUENCE: u8,
    pub FIELD_TYPE_INT64_BOUNDED_SEQUENCE: u8,
    pub FIELD_TYPE_UINT64_BOUNDED_SEQUENCE: u8,
    pub FIELD_TYPE_INT8_UNBOUNDED_SEQUENCE: u8,
    pub FIELD_TYPE_UINT8_UNBOUNDED_SEQUENCE: u8,
    pub FIELD_TYPE_INT16_UNBOUNDED_SEQUENCE: u8,
    pub FIELD_TYPE_UINT16_UNBOUNDED_SEQUENCE: u8,
    pub FIELD_TYPE_INT32_UNBOUNDED_SEQUENCE: u8,
    pub FIELD_TYPE_UINT32_UNBOUNDED_SEQUENCE: u8,
    pub FIELD_TYPE_INT64_UNBOUNDED_SEQUENCE: u8,
    pub FIELD_TYPE_UINT64_UNBOUNDED_SEQUENCE: u8,
    pub type_id: u8,
    pub capacity: u64,
    pub string_capacity: u64,
    pub nested_type_name: crate::msg::RosString<255>,
}

impl FieldType {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { type_description_interfaces__msg__FieldType__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for FieldType {
    fn drop(&mut self) {
        unsafe { type_description_interfaces__msg__FieldType__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct FieldTypeSeqRaw {
    data: *mut FieldType,
    size: usize,
    capacity: usize,
}

/// Sequence of FieldType.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct FieldTypeSeq<const N: usize> {
    data: *mut FieldType,
    size: usize,
    capacity: usize,
}

impl<const N: usize> FieldTypeSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: FieldTypeSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { type_description_interfaces__msg__FieldType__Sequence__init(&mut msg, size) } {
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
        let msg: FieldTypeSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {
            data: msg.data,
            size: msg.size,
            capacity: msg.capacity,
        }
    }

    pub fn as_slice(&self) -> &[FieldType] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [FieldType] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, FieldType> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, FieldType> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for FieldTypeSeq<N> {
    fn drop(&mut self) {
        let mut msg = FieldTypeSeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { type_description_interfaces__msg__FieldType__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for FieldTypeSeq<N> {}
unsafe impl<const N: usize> Sync for FieldTypeSeq<N> {}

impl TypeSupport for FieldType {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__type_description_interfaces__msg__FieldType()
        }
    }
}

impl PartialEq for FieldType {
    fn eq(&self, other: &Self) -> bool {
        unsafe { type_description_interfaces__msg__FieldType__are_equal(self, other) }
    }
}

impl<const N: usize> PartialEq for FieldTypeSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = FieldTypeSeqRaw {
                data: self.data,
                size: self.size,
                capacity: self.capacity,
            };
            let msg2 = FieldTypeSeqRaw {
                data: other.data,
                size: other.size,
                capacity: other.capacity,
            };
            type_description_interfaces__msg__FieldType__Sequence__are_equal(&msg1, &msg2)
        }
    }
}
