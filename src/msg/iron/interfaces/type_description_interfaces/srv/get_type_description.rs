// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::super::super::*;
use super::super::*;
use crate::msg::common_interfaces::*;
use crate::msg::*;
use crate::rcl;

extern "C" {
    fn type_description_interfaces__srv__GetTypeDescription_Request__init(
        msg: *mut GetTypeDescriptionRequest,
    ) -> bool;
    fn type_description_interfaces__srv__GetTypeDescription_Request__fini(
        msg: *mut GetTypeDescriptionRequest,
    );
    fn type_description_interfaces__srv__GetTypeDescription_Request__Sequence__init(
        msg: *mut GetTypeDescriptionRequestSeqRaw,
        size: usize,
    ) -> bool;
    fn type_description_interfaces__srv__GetTypeDescription_Request__Sequence__fini(
        msg: *mut GetTypeDescriptionRequestSeqRaw,
    );
    fn type_description_interfaces__srv__GetTypeDescription_Response__init(
        msg: *mut GetTypeDescriptionResponse,
    ) -> bool;
    fn type_description_interfaces__srv__GetTypeDescription_Response__fini(
        msg: *mut GetTypeDescriptionResponse,
    );
    fn type_description_interfaces__srv__GetTypeDescription_Response__Sequence__init(
        msg: *mut GetTypeDescriptionResponseSeqRaw,
        size: usize,
    ) -> bool;
    fn type_description_interfaces__srv__GetTypeDescription_Response__Sequence__fini(
        msg: *mut GetTypeDescriptionResponseSeqRaw,
    );
    fn rosidl_typesupport_c__get_service_type_support_handle__type_description_interfaces__srv__GetTypeDescription(
    ) -> *const rcl::rosidl_service_type_support_t;
    fn rosidl_typesupport_c__get_message_type_support_handle__type_description_interfaces__srv__GetTypeDescription_Request(
    ) -> *const rcl::rosidl_message_type_support_t;
    fn rosidl_typesupport_c__get_message_type_support_handle__type_description_interfaces__srv__GetTypeDescription_Response(
    ) -> *const rcl::rosidl_message_type_support_t;
}

#[repr(C)]
#[derive(Debug)]
pub struct GetTypeDescriptionRequest {
    pub type_name: crate::msg::RosString<0>,
    pub type_hash: crate::msg::RosString<0>,
    pub include_type_sources: bool,
}

#[repr(C)]
#[derive(Debug)]
pub struct GetTypeDescriptionResponse {
    pub successful: bool,
    pub failure_reason: crate::msg::RosString<0>,
    pub type_description: TypeDescription,
    pub type_sources: TypeSourceSeq<0>,
    pub extra_information: KeyValueSeq<0>,
}

impl GetTypeDescriptionRequest {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { type_description_interfaces__srv__GetTypeDescription_Request__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for GetTypeDescriptionRequest {
    fn drop(&mut self) {
        unsafe { type_description_interfaces__srv__GetTypeDescription_Request__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct GetTypeDescriptionRequestSeqRaw {
    data: *mut GetTypeDescriptionRequest,
    size: usize,
    capacity: usize,
}

/// Sequence of GetTypeDescriptionRequest.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct GetTypeDescriptionRequestSeq<const N: usize> {
    data: *mut GetTypeDescriptionRequest,
    size: usize,
    capacity: usize,
}

impl<const N: usize> GetTypeDescriptionRequestSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: GetTypeDescriptionRequestSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe {
            type_description_interfaces__srv__GetTypeDescription_Request__Sequence__init(
                &mut msg, size,
            )
        } {
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
        let msg: GetTypeDescriptionRequestSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {
            data: msg.data,
            size: msg.size,
            capacity: msg.capacity,
        }
    }

    pub fn as_slice(&self) -> &[GetTypeDescriptionRequest] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [GetTypeDescriptionRequest] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, GetTypeDescriptionRequest> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, GetTypeDescriptionRequest> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for GetTypeDescriptionRequestSeq<N> {
    fn drop(&mut self) {
        let mut msg = GetTypeDescriptionRequestSeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe {
            type_description_interfaces__srv__GetTypeDescription_Request__Sequence__fini(&mut msg)
        };
    }
}

unsafe impl<const N: usize> Send for GetTypeDescriptionRequestSeq<N> {}
unsafe impl<const N: usize> Sync for GetTypeDescriptionRequestSeq<N> {}

impl GetTypeDescriptionResponse {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { type_description_interfaces__srv__GetTypeDescription_Response__init(&mut msg) }
        {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for GetTypeDescriptionResponse {
    fn drop(&mut self) {
        unsafe { type_description_interfaces__srv__GetTypeDescription_Response__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct GetTypeDescriptionResponseSeqRaw {
    data: *mut GetTypeDescriptionResponse,
    size: usize,
    capacity: usize,
}

/// Sequence of GetTypeDescriptionResponse.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct GetTypeDescriptionResponseSeq<const N: usize> {
    data: *mut GetTypeDescriptionResponse,
    size: usize,
    capacity: usize,
}

impl<const N: usize> GetTypeDescriptionResponseSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }

        let mut msg: GetTypeDescriptionResponseSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe {
            type_description_interfaces__srv__GetTypeDescription_Response__Sequence__init(
                &mut msg, size,
            )
        } {
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
        let msg: GetTypeDescriptionResponseSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {
            data: msg.data,
            size: msg.size,
            capacity: msg.capacity,
        }
    }

    pub fn as_slice(&self) -> &[GetTypeDescriptionResponse] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [GetTypeDescriptionResponse] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, GetTypeDescriptionResponse> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, GetTypeDescriptionResponse> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for GetTypeDescriptionResponseSeq<N> {
    fn drop(&mut self) {
        let mut msg = GetTypeDescriptionResponseSeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe {
            type_description_interfaces__srv__GetTypeDescription_Response__Sequence__fini(&mut msg)
        };
    }
}

unsafe impl<const N: usize> Send for GetTypeDescriptionResponseSeq<N> {}
unsafe impl<const N: usize> Sync for GetTypeDescriptionResponseSeq<N> {}

pub struct GetTypeDescription;

impl ServiceMsg for GetTypeDescription {
    type Request = GetTypeDescriptionRequest;
    type Response = GetTypeDescriptionResponse;
    fn type_support() -> *const rcl::rosidl_service_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_service_type_support_handle__type_description_interfaces__srv__GetTypeDescription()
        }
    }
}

impl TypeSupport for GetTypeDescriptionRequest {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__type_description_interfaces__srv__GetTypeDescription_Request()
        }
    }
}

impl TypeSupport for GetTypeDescriptionResponse {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__type_description_interfaces__srv__GetTypeDescription_Response()
        }
    }
}
