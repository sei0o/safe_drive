// Copied from tests/common/action_msg/action/my_action.rs
#![allow(dead_code)]

use safe_drive::{
    msg::{
        builtin_interfaces::UnsafeTime, unique_identifier_msgs, ActionGoal, ActionMsg,
        ActionResult, GetUUID, GoalResponse, ResultResponse, TypeSupport,
    },
    rcl,
};

extern "C" {
    fn rosidl_typesupport_c__get_action_type_support_handle__example_msg__action__MyAction(
    ) -> *const rcl::rosidl_action_type_support_t;
}

#[derive(Debug)]
pub struct MyAction;

impl ActionMsg for MyAction {
    type Goal = MyAction_SendGoal;
    type Result = MyAction_GetResult;
    type Feedback = MyAction_FeedbackMessage;
    fn type_support() -> *const rcl::rosidl_action_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_action_type_support_handle__example_msg__action__MyAction()
        }
    }

    type GoalContent = MyAction_Goal;

    fn new_goal_request(
        goal: Self::GoalContent,
        uuid: [u8; 16],
    ) -> <Self::Goal as ActionGoal>::Request {
        MyAction_SendGoal_Request {
            goal,
            goal_id: unique_identifier_msgs::msg::UUID { uuid },
        }
    }

    type ResultContent = MyAction_Result;

    fn new_result_response(
        status: u8,
        result: Self::ResultContent,
    ) -> <Self::Result as ActionResult>::Response {
        MyAction_GetResult_Response { status, result }
    }

    type FeedbackContent = MyAction_Feedback;

    fn new_feedback_message(feedback: Self::FeedbackContent, uuid: [u8; 16]) -> Self::Feedback {
        MyAction_FeedbackMessage {
            feedback,
            goal_id: unique_identifier_msgs::msg::UUID { uuid },
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct MyAction_SendGoal_Request {
    pub goal_id: unique_identifier_msgs::msg::UUID,
    pub goal: MyAction_Goal,
}

#[repr(C)]
#[derive(Debug)]
pub struct MyAction_SendGoal_Response {
    pub accepted: bool,
    pub stamp: UnsafeTime,
}

#[repr(C)]
#[derive(Debug)]
pub struct MyAction_GetResult_Request {
    pub goal_id: unique_identifier_msgs::msg::UUID,
}

#[repr(C)]
#[derive(Debug)]
pub struct MyAction_GetResult_Response {
    pub status: u8,
    pub result: MyAction_Result,
}

#[repr(C)]
#[derive(Debug)]
pub struct MyAction_FeedbackMessage {
    pub goal_id: unique_identifier_msgs::msg::UUID,
    pub feedback: MyAction_Feedback,
}

#[repr(C)]
#[derive(Debug)]
pub struct MyAction_Goal {
    pub a: i64,
}

extern "C" {
    fn example_msg__action__MyAction_Goal__init(msg: *mut MyAction_Goal) -> bool;
    fn example_msg__action__MyAction_Goal__fini(msg: *mut MyAction_Goal);
    fn example_msg__action__MyAction_Goal__are_equal(
        lhs: *const MyAction_Goal,
        rhs: *const MyAction_Goal,
    ) -> bool;
    fn example_msg__action__MyAction_Goal__Sequence__init(
        msg: *mut MyAction_GoalSeqRaw,
        size: usize,
    ) -> bool;
    fn example_msg__action__MyAction_Goal__Sequence__fini(msg: *mut MyAction_GoalSeqRaw);
    fn example_msg__action__MyAction_Goal__Sequence__are_equal(
        lhs: *const MyAction_GoalSeqRaw,
        rhs: *const MyAction_GoalSeqRaw,
    ) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__example_msg__action__MyAction_Goal(
    ) -> *const rcl::rosidl_message_type_support_t;
}

impl TypeSupport for MyAction_Goal {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__example_msg__action__MyAction_Goal()
        }
    }
}

impl PartialEq for MyAction_Goal {
    fn eq(&self, other: &Self) -> bool {
        unsafe { example_msg__action__MyAction_Goal__are_equal(self, other) }
    }
}

impl<const N: usize> PartialEq for MyAction_GoalSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = MyAction_GoalSeqRaw {
                data: self.data,
                size: self.size,
                capacity: self.capacity,
            };
            let msg2 = MyAction_GoalSeqRaw {
                data: other.data,
                size: other.size,
                capacity: other.capacity,
            };
            example_msg__action__MyAction_Goal__Sequence__are_equal(&msg1, &msg2)
        }
    }
}

impl MyAction_Goal {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { example_msg__action__MyAction_Goal__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for MyAction_Goal {
    fn drop(&mut self) {
        unsafe { example_msg__action__MyAction_Goal__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct MyAction_GoalSeqRaw {
    data: *mut MyAction_Goal,
    size: usize,
    capacity: usize,
}

/// Sequence of MyAction_Goal.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct MyAction_GoalSeq<const N: usize> {
    data: *mut MyAction_Goal,
    size: usize,
    capacity: usize,
}

impl<const N: usize> MyAction_GoalSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }
        let mut msg: MyAction_GoalSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { example_msg__action__MyAction_Goal__Sequence__init(&mut msg, size) } {
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
        let msg: MyAction_GoalSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {
            data: msg.data,
            size: msg.size,
            capacity: msg.capacity,
        }
    }

    pub fn as_slice(&self) -> &[MyAction_Goal] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [MyAction_Goal] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, MyAction_Goal> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, MyAction_Goal> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for MyAction_GoalSeq<N> {
    fn drop(&mut self) {
        let mut msg = MyAction_GoalSeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { example_msg__action__MyAction_Goal__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for MyAction_GoalSeq<N> {}
unsafe impl<const N: usize> Sync for MyAction_GoalSeq<N> {}

extern "C" {
    fn example_msg__action__MyAction_SendGoal_Request__init(
        msg: *mut MyAction_SendGoal_Request,
    ) -> bool;
    fn example_msg__action__MyAction_SendGoal_Request__fini(msg: *mut MyAction_SendGoal_Request);
    fn example_msg__action__MyAction_SendGoal_Request__are_equal(
        lhs: *const MyAction_SendGoal_Request,
        rhs: *const MyAction_SendGoal_Request,
    ) -> bool;
    fn example_msg__action__MyAction_SendGoal_Request__Sequence__init(
        msg: *mut MyAction_SendGoal_RequestSeqRaw,
        size: usize,
    ) -> bool;
    fn example_msg__action__MyAction_SendGoal_Request__Sequence__fini(
        msg: *mut MyAction_SendGoal_RequestSeqRaw,
    );
    fn example_msg__action__MyAction_SendGoal_Request__Sequence__are_equal(
        lhs: *const MyAction_SendGoal_RequestSeqRaw,
        rhs: *const MyAction_SendGoal_RequestSeqRaw,
    ) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__example_msg__action__MyAction_SendGoal_Request(
    ) -> *const rcl::rosidl_message_type_support_t;
}

impl TypeSupport for MyAction_SendGoal_Request {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__example_msg__action__MyAction_SendGoal_Request()
        }
    }
}

impl PartialEq for MyAction_SendGoal_Request {
    fn eq(&self, other: &Self) -> bool {
        unsafe { example_msg__action__MyAction_SendGoal_Request__are_equal(self, other) }
    }
}

impl<const N: usize> PartialEq for MyAction_SendGoal_RequestSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = MyAction_SendGoal_RequestSeqRaw {
                data: self.data,
                size: self.size,
                capacity: self.capacity,
            };
            let msg2 = MyAction_SendGoal_RequestSeqRaw {
                data: other.data,
                size: other.size,
                capacity: other.capacity,
            };
            example_msg__action__MyAction_SendGoal_Request__Sequence__are_equal(&msg1, &msg2)
        }
    }
}

impl MyAction_SendGoal_Request {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { example_msg__action__MyAction_SendGoal_Request__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for MyAction_SendGoal_Request {
    fn drop(&mut self) {
        unsafe { example_msg__action__MyAction_SendGoal_Request__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct MyAction_SendGoal_RequestSeqRaw {
    data: *mut MyAction_SendGoal_Request,
    size: usize,
    capacity: usize,
}

/// Sequence of MyAction_SendGoal_Request.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct MyAction_SendGoal_RequestSeq<const N: usize> {
    data: *mut MyAction_SendGoal_Request,
    size: usize,
    capacity: usize,
}

impl<const N: usize> MyAction_SendGoal_RequestSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }
        let mut msg: MyAction_SendGoal_RequestSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { example_msg__action__MyAction_SendGoal_Request__Sequence__init(&mut msg, size) }
        {
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
        let msg: MyAction_SendGoal_RequestSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {
            data: msg.data,
            size: msg.size,
            capacity: msg.capacity,
        }
    }

    pub fn as_slice(&self) -> &[MyAction_SendGoal_Request] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [MyAction_SendGoal_Request] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, MyAction_SendGoal_Request> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, MyAction_SendGoal_Request> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for MyAction_SendGoal_RequestSeq<N> {
    fn drop(&mut self) {
        let mut msg = MyAction_SendGoal_RequestSeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { example_msg__action__MyAction_SendGoal_Request__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for MyAction_SendGoal_RequestSeq<N> {}
unsafe impl<const N: usize> Sync for MyAction_SendGoal_RequestSeq<N> {}

extern "C" {
    fn example_msg__action__MyAction_SendGoal_Response__init(
        msg: *mut MyAction_SendGoal_Response,
    ) -> bool;
    fn example_msg__action__MyAction_SendGoal_Response__fini(msg: *mut MyAction_SendGoal_Response);
    fn example_msg__action__MyAction_SendGoal_Response__are_equal(
        lhs: *const MyAction_SendGoal_Response,
        rhs: *const MyAction_SendGoal_Response,
    ) -> bool;
    fn example_msg__action__MyAction_SendGoal_Response__Sequence__init(
        msg: *mut MyAction_SendGoal_ResponseSeqRaw,
        size: usize,
    ) -> bool;
    fn example_msg__action__MyAction_SendGoal_Response__Sequence__fini(
        msg: *mut MyAction_SendGoal_ResponseSeqRaw,
    );
    fn example_msg__action__MyAction_SendGoal_Response__Sequence__are_equal(
        lhs: *const MyAction_SendGoal_ResponseSeqRaw,
        rhs: *const MyAction_SendGoal_ResponseSeqRaw,
    ) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__example_msg__action__MyAction_SendGoal_Response(
    ) -> *const rcl::rosidl_message_type_support_t;
}

impl TypeSupport for MyAction_SendGoal_Response {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__example_msg__action__MyAction_SendGoal_Response()
        }
    }
}

impl PartialEq for MyAction_SendGoal_Response {
    fn eq(&self, other: &Self) -> bool {
        unsafe { example_msg__action__MyAction_SendGoal_Response__are_equal(self, other) }
    }
}

impl<const N: usize> PartialEq for MyAction_SendGoal_ResponseSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = MyAction_SendGoal_ResponseSeqRaw {
                data: self.data,
                size: self.size,
                capacity: self.capacity,
            };
            let msg2 = MyAction_SendGoal_ResponseSeqRaw {
                data: other.data,
                size: other.size,
                capacity: other.capacity,
            };
            example_msg__action__MyAction_SendGoal_Response__Sequence__are_equal(&msg1, &msg2)
        }
    }
}

impl MyAction_SendGoal_Response {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { example_msg__action__MyAction_SendGoal_Response__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for MyAction_SendGoal_Response {
    fn drop(&mut self) {
        unsafe { example_msg__action__MyAction_SendGoal_Response__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct MyAction_SendGoal_ResponseSeqRaw {
    data: *mut MyAction_SendGoal_Response,
    size: usize,
    capacity: usize,
}

/// Sequence of MyAction_SendGoal_Response.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct MyAction_SendGoal_ResponseSeq<const N: usize> {
    data: *mut MyAction_SendGoal_Response,
    size: usize,
    capacity: usize,
}

impl<const N: usize> MyAction_SendGoal_ResponseSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }
        let mut msg: MyAction_SendGoal_ResponseSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe {
            example_msg__action__MyAction_SendGoal_Response__Sequence__init(&mut msg, size)
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
        let msg: MyAction_SendGoal_ResponseSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {
            data: msg.data,
            size: msg.size,
            capacity: msg.capacity,
        }
    }

    pub fn as_slice(&self) -> &[MyAction_SendGoal_Response] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [MyAction_SendGoal_Response] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, MyAction_SendGoal_Response> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, MyAction_SendGoal_Response> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for MyAction_SendGoal_ResponseSeq<N> {
    fn drop(&mut self) {
        let mut msg = MyAction_SendGoal_ResponseSeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { example_msg__action__MyAction_SendGoal_Response__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for MyAction_SendGoal_ResponseSeq<N> {}
unsafe impl<const N: usize> Sync for MyAction_SendGoal_ResponseSeq<N> {}

extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__example_msg__action__MyAction_SendGoal(
    ) -> *const rcl::rosidl_service_type_support_t;
}

#[derive(Debug)]
pub struct MyAction_SendGoal;

impl ActionGoal for MyAction_SendGoal {
    type Request = MyAction_SendGoal_Request;
    type Response = MyAction_SendGoal_Response;
    fn type_support() -> *const rcl::rosidl_service_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_service_type_support_handle__example_msg__action__MyAction_SendGoal()
        }
    }
}

impl GetUUID for MyAction_SendGoal_Request {
    fn get_uuid(&self) -> &[u8; 16] {
        &self.goal_id.uuid
    }
}

impl GoalResponse for MyAction_SendGoal_Response {
    fn is_accepted(&self) -> bool {
        self.accepted
    }

    fn get_time_stamp(&self) -> UnsafeTime {
        UnsafeTime {
            sec: self.stamp.sec,
            nanosec: self.stamp.nanosec,
        }
    }

    fn new(accepted: bool, stamp: UnsafeTime) -> Self {
        Self { accepted, stamp }
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct MyAction_Result {
    pub b: i64,
}

extern "C" {
    fn example_msg__action__MyAction_Result__init(msg: *mut MyAction_Result) -> bool;
    fn example_msg__action__MyAction_Result__fini(msg: *mut MyAction_Result);
    fn example_msg__action__MyAction_Result__are_equal(
        lhs: *const MyAction_Result,
        rhs: *const MyAction_Result,
    ) -> bool;
    fn example_msg__action__MyAction_Result__Sequence__init(
        msg: *mut MyAction_ResultSeqRaw,
        size: usize,
    ) -> bool;
    fn example_msg__action__MyAction_Result__Sequence__fini(msg: *mut MyAction_ResultSeqRaw);
    fn example_msg__action__MyAction_Result__Sequence__are_equal(
        lhs: *const MyAction_ResultSeqRaw,
        rhs: *const MyAction_ResultSeqRaw,
    ) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__example_msg__action__MyAction_Result(
    ) -> *const rcl::rosidl_message_type_support_t;
}

impl TypeSupport for MyAction_Result {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__example_msg__action__MyAction_Result()
        }
    }
}

impl PartialEq for MyAction_Result {
    fn eq(&self, other: &Self) -> bool {
        unsafe { example_msg__action__MyAction_Result__are_equal(self, other) }
    }
}

impl<const N: usize> PartialEq for MyAction_ResultSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = MyAction_ResultSeqRaw {
                data: self.data,
                size: self.size,
                capacity: self.capacity,
            };
            let msg2 = MyAction_ResultSeqRaw {
                data: other.data,
                size: other.size,
                capacity: other.capacity,
            };
            example_msg__action__MyAction_Result__Sequence__are_equal(&msg1, &msg2)
        }
    }
}

impl MyAction_Result {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { example_msg__action__MyAction_Result__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for MyAction_Result {
    fn drop(&mut self) {
        unsafe { example_msg__action__MyAction_Result__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct MyAction_ResultSeqRaw {
    data: *mut MyAction_Result,
    size: usize,
    capacity: usize,
}

/// Sequence of MyAction_Result.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct MyAction_ResultSeq<const N: usize> {
    data: *mut MyAction_Result,
    size: usize,
    capacity: usize,
}

impl<const N: usize> MyAction_ResultSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }
        let mut msg: MyAction_ResultSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { example_msg__action__MyAction_Result__Sequence__init(&mut msg, size) } {
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
        let msg: MyAction_ResultSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {
            data: msg.data,
            size: msg.size,
            capacity: msg.capacity,
        }
    }

    pub fn as_slice(&self) -> &[MyAction_Result] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [MyAction_Result] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, MyAction_Result> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, MyAction_Result> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for MyAction_ResultSeq<N> {
    fn drop(&mut self) {
        let mut msg = MyAction_ResultSeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { example_msg__action__MyAction_Result__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for MyAction_ResultSeq<N> {}
unsafe impl<const N: usize> Sync for MyAction_ResultSeq<N> {}

extern "C" {
    fn example_msg__action__MyAction_GetResult_Request__init(
        msg: *mut MyAction_GetResult_Request,
    ) -> bool;
    fn example_msg__action__MyAction_GetResult_Request__fini(msg: *mut MyAction_GetResult_Request);
    fn example_msg__action__MyAction_GetResult_Request__are_equal(
        lhs: *const MyAction_GetResult_Request,
        rhs: *const MyAction_GetResult_Request,
    ) -> bool;
    fn example_msg__action__MyAction_GetResult_Request__Sequence__init(
        msg: *mut MyAction_GetResult_RequestSeqRaw,
        size: usize,
    ) -> bool;
    fn example_msg__action__MyAction_GetResult_Request__Sequence__fini(
        msg: *mut MyAction_GetResult_RequestSeqRaw,
    );
    fn example_msg__action__MyAction_GetResult_Request__Sequence__are_equal(
        lhs: *const MyAction_GetResult_RequestSeqRaw,
        rhs: *const MyAction_GetResult_RequestSeqRaw,
    ) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__example_msg__action__MyAction_GetResult_Request(
    ) -> *const rcl::rosidl_message_type_support_t;
}

impl TypeSupport for MyAction_GetResult_Request {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__example_msg__action__MyAction_GetResult_Request()
        }
    }
}

impl PartialEq for MyAction_GetResult_Request {
    fn eq(&self, other: &Self) -> bool {
        unsafe { example_msg__action__MyAction_GetResult_Request__are_equal(self, other) }
    }
}

impl<const N: usize> PartialEq for MyAction_GetResult_RequestSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = MyAction_GetResult_RequestSeqRaw {
                data: self.data,
                size: self.size,
                capacity: self.capacity,
            };
            let msg2 = MyAction_GetResult_RequestSeqRaw {
                data: other.data,
                size: other.size,
                capacity: other.capacity,
            };
            example_msg__action__MyAction_GetResult_Request__Sequence__are_equal(&msg1, &msg2)
        }
    }
}

impl MyAction_GetResult_Request {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { example_msg__action__MyAction_GetResult_Request__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for MyAction_GetResult_Request {
    fn drop(&mut self) {
        unsafe { example_msg__action__MyAction_GetResult_Request__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct MyAction_GetResult_RequestSeqRaw {
    data: *mut MyAction_GetResult_Request,
    size: usize,
    capacity: usize,
}

/// Sequence of MyAction_GetResult_Request.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct MyAction_GetResult_RequestSeq<const N: usize> {
    data: *mut MyAction_GetResult_Request,
    size: usize,
    capacity: usize,
}

impl<const N: usize> MyAction_GetResult_RequestSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }
        let mut msg: MyAction_GetResult_RequestSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe {
            example_msg__action__MyAction_GetResult_Request__Sequence__init(&mut msg, size)
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
        let msg: MyAction_GetResult_RequestSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {
            data: msg.data,
            size: msg.size,
            capacity: msg.capacity,
        }
    }

    pub fn as_slice(&self) -> &[MyAction_GetResult_Request] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [MyAction_GetResult_Request] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, MyAction_GetResult_Request> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, MyAction_GetResult_Request> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for MyAction_GetResult_RequestSeq<N> {
    fn drop(&mut self) {
        let mut msg = MyAction_GetResult_RequestSeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { example_msg__action__MyAction_GetResult_Request__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for MyAction_GetResult_RequestSeq<N> {}
unsafe impl<const N: usize> Sync for MyAction_GetResult_RequestSeq<N> {}

extern "C" {
    fn example_msg__action__MyAction_GetResult_Response__init(
        msg: *mut MyAction_GetResult_Response,
    ) -> bool;
    fn example_msg__action__MyAction_GetResult_Response__fini(
        msg: *mut MyAction_GetResult_Response,
    );
    fn example_msg__action__MyAction_GetResult_Response__are_equal(
        lhs: *const MyAction_GetResult_Response,
        rhs: *const MyAction_GetResult_Response,
    ) -> bool;
    fn example_msg__action__MyAction_GetResult_Response__Sequence__init(
        msg: *mut MyAction_GetResult_ResponseSeqRaw,
        size: usize,
    ) -> bool;
    fn example_msg__action__MyAction_GetResult_Response__Sequence__fini(
        msg: *mut MyAction_GetResult_ResponseSeqRaw,
    );
    fn example_msg__action__MyAction_GetResult_Response__Sequence__are_equal(
        lhs: *const MyAction_GetResult_ResponseSeqRaw,
        rhs: *const MyAction_GetResult_ResponseSeqRaw,
    ) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__example_msg__action__MyAction_GetResult_Response(
    ) -> *const rcl::rosidl_message_type_support_t;
}

impl TypeSupport for MyAction_GetResult_Response {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__example_msg__action__MyAction_GetResult_Response()
        }
    }
}

impl PartialEq for MyAction_GetResult_Response {
    fn eq(&self, other: &Self) -> bool {
        unsafe { example_msg__action__MyAction_GetResult_Response__are_equal(self, other) }
    }
}

impl<const N: usize> PartialEq for MyAction_GetResult_ResponseSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = MyAction_GetResult_ResponseSeqRaw {
                data: self.data,
                size: self.size,
                capacity: self.capacity,
            };
            let msg2 = MyAction_GetResult_ResponseSeqRaw {
                data: other.data,
                size: other.size,
                capacity: other.capacity,
            };
            example_msg__action__MyAction_GetResult_Response__Sequence__are_equal(&msg1, &msg2)
        }
    }
}

impl MyAction_GetResult_Response {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { example_msg__action__MyAction_GetResult_Response__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for MyAction_GetResult_Response {
    fn drop(&mut self) {
        unsafe { example_msg__action__MyAction_GetResult_Response__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct MyAction_GetResult_ResponseSeqRaw {
    data: *mut MyAction_GetResult_Response,
    size: usize,
    capacity: usize,
}

/// Sequence of MyAction_GetResult_Response.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct MyAction_GetResult_ResponseSeq<const N: usize> {
    data: *mut MyAction_GetResult_Response,
    size: usize,
    capacity: usize,
}

impl<const N: usize> MyAction_GetResult_ResponseSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }
        let mut msg: MyAction_GetResult_ResponseSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe {
            example_msg__action__MyAction_GetResult_Response__Sequence__init(&mut msg, size)
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
        let msg: MyAction_GetResult_ResponseSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {
            data: msg.data,
            size: msg.size,
            capacity: msg.capacity,
        }
    }

    pub fn as_slice(&self) -> &[MyAction_GetResult_Response] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [MyAction_GetResult_Response] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, MyAction_GetResult_Response> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, MyAction_GetResult_Response> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for MyAction_GetResult_ResponseSeq<N> {
    fn drop(&mut self) {
        let mut msg = MyAction_GetResult_ResponseSeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { example_msg__action__MyAction_GetResult_Response__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for MyAction_GetResult_ResponseSeq<N> {}
unsafe impl<const N: usize> Sync for MyAction_GetResult_ResponseSeq<N> {}

extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__example_msg__action__MyAction_GetResult(
    ) -> *const rcl::rosidl_service_type_support_t;
}

#[derive(Debug)]
pub struct MyAction_GetResult;

impl ActionResult for MyAction_GetResult {
    type Request = MyAction_GetResult_Request;
    type Response = MyAction_GetResult_Response;
    fn type_support() -> *const rcl::rosidl_service_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_service_type_support_handle__example_msg__action__MyAction_GetResult()
        }
    }
}

impl GetUUID for MyAction_GetResult_Request {
    fn get_uuid(&self) -> &[u8; 16] {
        &self.goal_id.uuid
    }
}

impl ResultResponse for MyAction_GetResult_Response {
    fn get_status(&self) -> u8 {
        self.status
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct MyAction_Feedback {
    pub c: i64,
}

extern "C" {
    fn example_msg__action__MyAction_Feedback__init(msg: *mut MyAction_Feedback) -> bool;
    fn example_msg__action__MyAction_Feedback__fini(msg: *mut MyAction_Feedback);
    fn example_msg__action__MyAction_Feedback__are_equal(
        lhs: *const MyAction_Feedback,
        rhs: *const MyAction_Feedback,
    ) -> bool;
    fn example_msg__action__MyAction_Feedback__Sequence__init(
        msg: *mut MyAction_FeedbackSeqRaw,
        size: usize,
    ) -> bool;
    fn example_msg__action__MyAction_Feedback__Sequence__fini(msg: *mut MyAction_FeedbackSeqRaw);
    fn example_msg__action__MyAction_Feedback__Sequence__are_equal(
        lhs: *const MyAction_FeedbackSeqRaw,
        rhs: *const MyAction_FeedbackSeqRaw,
    ) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__example_msg__action__MyAction_Feedback(
    ) -> *const rcl::rosidl_message_type_support_t;
}

impl TypeSupport for MyAction_Feedback {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__example_msg__action__MyAction_Feedback()
        }
    }
}

impl PartialEq for MyAction_Feedback {
    fn eq(&self, other: &Self) -> bool {
        unsafe { example_msg__action__MyAction_Feedback__are_equal(self, other) }
    }
}

impl<const N: usize> PartialEq for MyAction_FeedbackSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = MyAction_FeedbackSeqRaw {
                data: self.data,
                size: self.size,
                capacity: self.capacity,
            };
            let msg2 = MyAction_FeedbackSeqRaw {
                data: other.data,
                size: other.size,
                capacity: other.capacity,
            };
            example_msg__action__MyAction_Feedback__Sequence__are_equal(&msg1, &msg2)
        }
    }
}

impl MyAction_Feedback {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { example_msg__action__MyAction_Feedback__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for MyAction_Feedback {
    fn drop(&mut self) {
        unsafe { example_msg__action__MyAction_Feedback__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct MyAction_FeedbackSeqRaw {
    data: *mut MyAction_Feedback,
    size: usize,
    capacity: usize,
}

/// Sequence of MyAction_Feedback.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct MyAction_FeedbackSeq<const N: usize> {
    data: *mut MyAction_Feedback,
    size: usize,
    capacity: usize,
}

impl<const N: usize> MyAction_FeedbackSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }
        let mut msg: MyAction_FeedbackSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { example_msg__action__MyAction_Feedback__Sequence__init(&mut msg, size) } {
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
        let msg: MyAction_FeedbackSeqRaw = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {
            data: msg.data,
            size: msg.size,
            capacity: msg.capacity,
        }
    }

    pub fn as_slice(&self) -> &[MyAction_Feedback] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [MyAction_Feedback] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, MyAction_Feedback> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, MyAction_Feedback> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for MyAction_FeedbackSeq<N> {
    fn drop(&mut self) {
        let mut msg = MyAction_FeedbackSeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { example_msg__action__MyAction_Feedback__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for MyAction_FeedbackSeq<N> {}
unsafe impl<const N: usize> Sync for MyAction_FeedbackSeq<N> {}

extern "C" {
    fn example_msg__action__MyAction_FeedbackMessage__init(
        msg: *mut MyAction_FeedbackMessage,
    ) -> bool;
    fn example_msg__action__MyAction_FeedbackMessage__fini(msg: *mut MyAction_FeedbackMessage);
    fn example_msg__action__MyAction_FeedbackMessage__are_equal(
        lhs: *const MyAction_FeedbackMessage,
        rhs: *const MyAction_FeedbackMessage,
    ) -> bool;
    fn example_msg__action__MyAction_FeedbackMessage__Sequence__init(
        msg: *mut MyAction_FeedbackMessageSeqRaw,
        size: usize,
    ) -> bool;
    fn example_msg__action__MyAction_FeedbackMessage__Sequence__fini(
        msg: *mut MyAction_FeedbackMessageSeqRaw,
    );
    fn example_msg__action__MyAction_FeedbackMessage__Sequence__are_equal(
        lhs: *const MyAction_FeedbackMessageSeqRaw,
        rhs: *const MyAction_FeedbackMessageSeqRaw,
    ) -> bool;
    fn rosidl_typesupport_c__get_message_type_support_handle__example_msg__action__MyAction_FeedbackMessage(
    ) -> *const rcl::rosidl_message_type_support_t;
}

impl TypeSupport for MyAction_FeedbackMessage {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__example_msg__action__MyAction_FeedbackMessage()
        }
    }
}

impl PartialEq for MyAction_FeedbackMessage {
    fn eq(&self, other: &Self) -> bool {
        unsafe { example_msg__action__MyAction_FeedbackMessage__are_equal(self, other) }
    }
}

impl<const N: usize> PartialEq for MyAction_FeedbackMessageSeq<N> {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            let msg1 = MyAction_FeedbackMessageSeqRaw {
                data: self.data,
                size: self.size,
                capacity: self.capacity,
            };
            let msg2 = MyAction_FeedbackMessageSeqRaw {
                data: other.data,
                size: other.size,
                capacity: other.capacity,
            };
            example_msg__action__MyAction_FeedbackMessage__Sequence__are_equal(&msg1, &msg2)
        }
    }
}

impl MyAction_FeedbackMessage {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { example_msg__action__MyAction_FeedbackMessage__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for MyAction_FeedbackMessage {
    fn drop(&mut self) {
        unsafe { example_msg__action__MyAction_FeedbackMessage__fini(self) };
    }
}

#[repr(C)]
#[derive(Debug)]
struct MyAction_FeedbackMessageSeqRaw {
    data: *mut MyAction_FeedbackMessage,
    size: usize,
    capacity: usize,
}

/// Sequence of MyAction_FeedbackMessage.
/// `N` is the maximum number of elements.
/// If `N` is `0`, the size is unlimited.
#[repr(C)]
#[derive(Debug)]
pub struct MyAction_FeedbackMessageSeq<const N: usize> {
    data: *mut MyAction_FeedbackMessage,
    size: usize,
    capacity: usize,
}

impl<const N: usize> MyAction_FeedbackMessageSeq<N> {
    /// Create a sequence of.
    /// `N` represents the maximum number of elements.
    /// If `N` is `0`, the sequence is unlimited.
    pub fn new(size: usize) -> Option<Self> {
        if N != 0 && size >= N {
            // the size exceeds in the maximum number
            return None;
        }
        let mut msg: MyAction_FeedbackMessageSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { example_msg__action__MyAction_FeedbackMessage__Sequence__init(&mut msg, size) }
        {
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
        let msg: MyAction_FeedbackMessageSeqRaw =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        Self {
            data: msg.data,
            size: msg.size,
            capacity: msg.capacity,
        }
    }

    pub fn as_slice(&self) -> &[MyAction_FeedbackMessage] {
        if self.data.is_null() {
            &[]
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            s
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [MyAction_FeedbackMessage] {
        if self.data.is_null() {
            &mut []
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            s
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, MyAction_FeedbackMessage> {
        self.as_slice().iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, MyAction_FeedbackMessage> {
        self.as_slice_mut().iter_mut()
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<const N: usize> Drop for MyAction_FeedbackMessageSeq<N> {
    fn drop(&mut self) {
        let mut msg = MyAction_FeedbackMessageSeqRaw {
            data: self.data,
            size: self.size,
            capacity: self.capacity,
        };
        unsafe { example_msg__action__MyAction_FeedbackMessage__Sequence__fini(&mut msg) };
    }
}

unsafe impl<const N: usize> Send for MyAction_FeedbackMessageSeq<N> {}
unsafe impl<const N: usize> Sync for MyAction_FeedbackMessageSeq<N> {}

impl GetUUID for MyAction_FeedbackMessage {
    fn get_uuid(&self) -> &[u8; 16] {
        &self.goal_id.uuid
    }
}
