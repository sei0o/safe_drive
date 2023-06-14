use crossbeam_channel::{Receiver, Sender, TryRecvError};
use parking_lot::Mutex;
use std::{collections::BTreeMap, ffi::CString, mem::MaybeUninit, sync::Arc, time::Duration};

use crate::{
    clock::Clock,
    error::{DynError, RCLActionError, RCLActionResult},
    get_allocator,
    msg::{
        builtin_interfaces,
        interfaces::action_msgs::{
            msg::{GoalInfo, GoalInfoSeq},
            srv::{ERROR_NONE, ERROR_REJECTED},
        },
        unique_identifier_msgs::msg::UUID,
        ActionGoal, ActionMsg, GetUUID, GoalResponse,
    },
    node::Node,
    qos::Profile,
    rcl::{
        self, action_msgs__msg__GoalInfo, action_msgs__msg__GoalInfo__Sequence,
        rcl_action_cancel_request_t, rcutils_get_error_string, unique_identifier_msgs__msg__UUID,
    },
    RecvResult,
};

#[cfg(feature = "galactic")]
use crate::qos::galactic::*;

#[cfg(feature = "humble")]
use crate::qos::humble::*;

use super::{
    handle::{GoalHandle, GoalHandleMessage, GoalHandleMessageContent},
    CancelRequest, GetResultServiceRequest, SendGoalServiceRequest,
};

pub struct ServerQosOption {
    pub goal_service: Profile,
    pub result_service: Profile,
    pub cancel_service: Profile,
    pub feedback_topic: Profile,
    pub status_topic: Profile,
    pub result_timeout: Duration,
}

impl Default for ServerQosOption {
    fn default() -> Self {
        let status_topic_profile = Profile {
            history: HistoryPolicy::KeepLast,
            depth: 1,
            reliability: ReliabilityPolicy::Reliable,
            durability: DurabilityPolicy::TransientLocal,
            liveliness: LivelinessPolicy::SystemDefault,
            avoid_ros_namespace_conventions: false,
            ..Default::default()
        };

        Self {
            goal_service: Profile::services_default(),
            result_service: Profile::services_default(),
            cancel_service: Profile::services_default(),
            feedback_topic: Profile::default(),
            status_topic: status_topic_profile,
            result_timeout: Duration::from_secs(15 * 60),
        }
    }
}

impl From<ServerQosOption> for rcl::rcl_action_server_options_t {
    fn from(opts: ServerQosOption) -> Self {
        rcl::rcl_action_server_options_t {
            goal_service_qos: (&opts.goal_service).into(),
            cancel_service_qos: (&opts.cancel_service).into(),
            result_service_qos: (&opts.result_service).into(),
            feedback_topic_qos: (&opts.feedback_topic).into(),
            status_topic_qos: (&opts.status_topic).into(),
            allocator: get_allocator(),
            result_timeout: rcl::rcl_duration_t {
                nanoseconds: opts.result_timeout.as_nanos() as i64,
            },
        }
    }
}

// These constants are defined in action_msgs.
pub enum GoalStatus {
    Unknown = 0,
    Accepted = 1,
    Executing = 2,
    Canceling = 3,
    Succeeded = 4,
    Canceled = 5,
    Aborted = 6,
}

pub(crate) struct ActionServerData<T: ActionMsg> {
    pub server: rcl::rcl_action_server_t,
    pub node: Arc<Node>,

    /// Once the server has completed the result for a goal, it is kept here and the result requests are responsed with the result value in this map.
    pub results: Mutex<BTreeMap<[u8; 16], T::ResultContent>>,
}

impl<T: ActionMsg> ActionServerData<T> {
    pub(crate) unsafe fn as_ptr_mut(&self) -> *mut rcl::rcl_action_server_t {
        &self.server as *const _ as *mut _
    }
}

unsafe impl<T: ActionMsg> Sync for ActionServerData<T> {}
unsafe impl<T: ActionMsg> Send for ActionServerData<T> {}

/// An action server.
pub struct Server<T: ActionMsg> {
    pub(crate) data: Arc<ActionServerData<T>>,
    clock: Clock,

    /// Handler for goal requests from clients.
    goal_request_callback: Box<dyn Fn(GoalHandle<T>, SendGoalServiceRequest<T>) -> bool>,
    cancel_request_callback: Box<dyn Fn(CancelRequest) -> bool>,

    /// Channels used to communicate with worker threads.
    rx: Receiver<GoalHandleMessage<T>>,
    tx: Sender<GoalHandleMessage<T>>,
}

impl<T> Server<T>
where
    T: ActionMsg,
{
    /// Create a server.
    pub fn new<GR, CR>(
        node: Arc<Node>,
        action_name: &str,
        qos: Option<ServerQosOption>,
        goal_request_callback: GR,
        cancel_request_callback: CR,
    ) -> RCLActionResult<Self>
    where
        GR: Fn(GoalHandle<T>, SendGoalServiceRequest<T>) -> bool + 'static,
        CR: Fn(CancelRequest) -> bool + 'static,
    {
        let mut server = rcl::MTSafeFn::rcl_action_get_zero_initialized_server();
        let options = qos
            .map(rcl::rcl_action_server_options_t::from)
            .unwrap_or_else(rcl::MTSafeFn::rcl_action_server_get_default_options);
        // TODO: reconcile RCLResult and RCLActionResult to avoid unwrap
        let clock = Clock::new().unwrap();
        let action_name = CString::new(action_name).unwrap_or_default();

        {
            let guard = rcl::MT_UNSAFE_FN.lock();
            guard.rcl_action_server_init(
                &mut server,
                unsafe { node.as_ptr_mut() },
                clock.as_ptr_mut(),
                T::type_support(),
                action_name.as_ptr(),
                &options,
            )?;
        }

        let (tx, rx) = crossbeam_channel::unbounded();

        let server = Self {
            data: Arc::new(ActionServerData {
                server,
                node,
                results: Mutex::new(BTreeMap::new()),
            }),
            clock,
            goal_request_callback: Box::new(goal_request_callback),
            cancel_request_callback: Box::new(cancel_request_callback),
            rx,
            tx,
        };
        server.publish_status().unwrap();

        Ok(server)
    }

    pub fn try_recv_goal_request(&mut self) -> RecvResult<(), ()> {
        let mut header: rcl::rmw_request_id_t = unsafe { MaybeUninit::zeroed().assume_init() };
        let mut request: SendGoalServiceRequest<T> = unsafe { MaybeUninit::zeroed().assume_init() };
        let result = {
            let guard = rcl::MT_UNSAFE_FN.lock();
            guard.rcl_action_take_goal_request(
                &self.data.server,
                &mut header,
                &mut request as *const _ as *mut _,
            )
        };

        match result {
            Ok(()) => {
                // get current time
                let now_nanosec = self.clock.get_now().unwrap();
                let now_sec = now_nanosec / 10_i64.pow(9);
                let stamp = builtin_interfaces::UnsafeTime {
                    sec: now_sec as i32,
                    nanosec: (now_nanosec - now_sec * 10_i64.pow(9)) as u32,
                };

                // accept goal if appropriate
                let uuid = *request.get_uuid();
                let handle = self.create_goal_handle(uuid);
                let callback = &self.goal_request_callback;
                let accepted = callback(handle, request);

                if accepted {
                    // see rcl_interfaces/action_msgs/msg/GoalInfo.msg for definition
                    let mut goal_info = rcl::MTSafeFn::rcl_action_get_zero_initialized_goal_info();

                    goal_info.goal_id = unique_identifier_msgs__msg__UUID { uuid };

                    goal_info.stamp.sec = (now_nanosec / 10_i64.pow(9)) as i32;
                    goal_info.stamp.nanosec = (now_nanosec - now_sec * 10_i64.pow(9)) as u32;

                    let goal_handle = {
                        let guard = rcl::MT_UNSAFE_FN.lock();
                        guard.rcl_action_accept_new_goal(
                            unsafe { self.data.as_ptr_mut() },
                            &goal_info,
                        )
                    };
                    if goal_handle.is_null() {
                        let msg = unsafe { rcutils_get_error_string() };
                        return RecvResult::Err(format!("Failed to accept new goal: {msg}").into());
                    }

                    self.publish_status().unwrap();
                }

                // TODO: Make SendgoalServiceResponse independent of T (edit safe-drive-msg)
                // SendGoal
                type GoalResponse<T> = <<T as ActionMsg>::Goal as ActionGoal>::Response;

                let mut response = GoalResponse::<T>::new(accepted, stamp);
                // let mut response = SendGoalServiceResponse { accepted, stamp };

                // send response to client
                let guard = rcl::MT_UNSAFE_FN.lock();
                match guard.rcl_action_send_goal_response(
                    &self.data.server,
                    &mut header,
                    &mut response as *const _ as *mut _,
                ) {
                    Ok(()) => RecvResult::Ok(()),
                    Err(e) => RecvResult::Err(e.into()),
                }
            }
            Err(RCLActionError::ServerTakeFailed) => RecvResult::RetryLater(()),
            Err(e) => RecvResult::Err(e.into()),
        }
    }

    pub fn try_recv_cancel_request(&mut self) -> RecvResult<(), ()> {
        let guard = rcl::MT_UNSAFE_FN.lock();

        let mut header: rcl::rmw_request_id_t = unsafe { MaybeUninit::zeroed().assume_init() };
        let mut request: rcl_action_cancel_request_t =
            rcl::MTSafeFn::rcl_action_get_zero_initialized_cancel_request();

        match guard.rcl_action_take_cancel_request(
            &self.data.server,
            &mut header,
            &mut request as *const _ as *mut _,
        ) {
            Ok(()) => {
                let mut process_response =
                    rcl::MTSafeFn::rcl_action_get_zero_initialized_cancel_response();

                // compute which exact goals are requested to be cancelled
                match guard.rcl_action_process_cancel_request(
                    &self.data.server,
                    &request,
                    &mut process_response as *const _ as *mut _,
                ) {
                    Ok(()) => {}
                    // TODO: handle ERROR_UNKNOWN_GOAL_ID etc.
                    Err(e) => return RecvResult::Err(e.into()),
                }

                let goal_seq_ptr =
                    &process_response.msg.goals_canceling as *const _ as *const GoalInfoSeq<0>;
                let candidates = unsafe { &(*goal_seq_ptr) };

                let mut accepted_goals = vec![];
                for goal in candidates.iter() {
                    let callback = &self.cancel_request_callback;
                    let accepted = callback(request);

                    if accepted {
                        accepted_goals.push(goal);
                    }
                }

                let mut cancel_response =
                    rcl::MTSafeFn::rcl_action_get_zero_initialized_cancel_response();

                cancel_response.msg.return_code = if accepted_goals.is_empty() {
                    ERROR_REJECTED
                } else {
                    ERROR_NONE
                };
                cancel_response.msg.goals_canceling = action_msgs__msg__GoalInfo__Sequence {
                    data: accepted_goals.as_mut_ptr() as *mut _ as *mut action_msgs__msg__GoalInfo,
                    size: accepted_goals.len() as rcl::size_t,
                    capacity: accepted_goals.capacity() as rcl::size_t,
                };

                match guard.rcl_action_send_cancel_response(
                    &self.data.server,
                    &mut header,
                    &mut cancel_response.msg as *const _ as *mut _,
                ) {
                    Ok(()) => RecvResult::Ok(()),
                    Err(e) => RecvResult::Err(e.into()),
                }
            }
            Err(RCLActionError::ServerTakeFailed) => RecvResult::RetryLater(()),
            Err(e) => RecvResult::Err(e.into()),
        }
    }

    pub fn try_recv_result_request(&mut self) -> RecvResult<(), ()> {
        let mut header: rcl::rmw_request_id_t = unsafe { MaybeUninit::zeroed().assume_init() };
        let mut request: GetResultServiceRequest<T> =
            unsafe { MaybeUninit::zeroed().assume_init() };

        let take_result = {
            let guard = rcl::MT_UNSAFE_FN.lock();
            guard.rcl_action_take_result_request(
                &self.data.server,
                &mut header,
                &mut request as *const _ as *mut _,
            )
        };

        match take_result {
            Ok(()) => {
                let removed = {
                    let mut results = self.data.results.lock();
                    results.remove(request.get_uuid())
                };
                match removed {
                    Some(result) => {
                        let mut response =
                            T::new_result_response(GoalStatus::Succeeded as u8, result);
                        let guard = rcl::MT_UNSAFE_FN.lock();
                        match guard.rcl_action_send_result_response(
                            &self.data.server,
                            &mut header,
                            &mut response as *const _ as *mut _,
                        ) {
                            Ok(()) => RecvResult::Ok(()),
                            Err(e) => RecvResult::Err(e.into()),
                        }
                    }
                    None => RecvResult::Err(
                        format!(
                            "The result for the goal (uuid: {:?}) is not available yet",
                            request.get_uuid()
                        )
                        .into(),
                    ),
                }
            }
            Err(RCLActionError::ServerTakeFailed) => RecvResult::RetryLater(()),
            Err(e) => RecvResult::Err(e.into()),
        }
    }

    // TODO: when to publish?
    fn publish_status(&self) -> Result<(), DynError> {
        let guard = rcl::MT_UNSAFE_FN.lock();

        let mut status = rcl::MTSafeFn::rcl_action_get_zero_initialized_goal_status_array();
        guard.rcl_action_get_goal_status_array(&self.data.server, &mut status)?;
        guard.rcl_action_publish_status(&self.data.server, &status as *const _ as *const _)?;

        Ok(())
    }

    /// Wait for messages from goal handles.
    pub fn recv_data(&mut self) -> Result<(), DynError> {
        match self.rx.recv() {
            Ok(GoalHandleMessage { goal_id, content }) => {
                match content {
                    GoalHandleMessageContent::Feedback(mut feedback) => {
                        let guard = rcl::MT_UNSAFE_FN.lock();
                        guard.rcl_action_publish_feedback(
                            unsafe { self.data.as_ptr_mut() },
                            &mut feedback as *const _ as *mut _,
                        )?;
                    }
                    GoalHandleMessageContent::Result(result) => {
                        // cache result for later use
                        let mut results = self.data.results.lock();
                        if let Some(_) = results.insert(goal_id, result) {
                            return Err("the result for the goal already exists; it should be set only once".into());
                        }
                    }
                }
            }
            Err(e) => todo!(),
        }

        Ok(())
    }

    pub fn try_recv_data(&mut self) -> Result<(), DynError> {
        let _ = self.try_recv_result_request();

        match self.rx.try_recv() {
            Ok(GoalHandleMessage { goal_id, content }) => {
                match content {
                    GoalHandleMessageContent::Feedback(mut feedback) => {
                        let guard = rcl::MT_UNSAFE_FN.lock();
                        guard.rcl_action_publish_feedback(
                            unsafe { self.data.as_ptr_mut() },
                            &mut feedback as *const _ as *mut _,
                        )?;
                    }
                    GoalHandleMessageContent::Result(result) => {
                        // cache result for later use
                        let mut results = self.data.results.lock();
                        if let Some(_) = results.insert(goal_id, result) {
                            return Err("the result for the goal already exists; it should be set only once".into());
                        }
                    }
                }
            }
            Err(TryRecvError::Empty) => {}
            Err(e) => todo!(),
        }

        Ok(())
    }

    fn create_goal_handle(&self, goal_id: [u8; 16]) -> GoalHandle<T> {
        GoalHandle::new(goal_id, self.data.clone(), self.tx.clone())
    }
}

impl<T: ActionMsg> Drop for Server<T> {
    fn drop(&mut self) {
        let guard = rcl::MT_UNSAFE_FN.lock();
        let _ = guard.rcl_action_server_fini(unsafe { self.data.as_ptr_mut() }, unsafe {
            self.data.node.as_ptr_mut()
        });
    }
}

impl From<action_msgs__msg__GoalInfo> for GoalInfo {
    fn from(value: action_msgs__msg__GoalInfo) -> Self {
        Self {
            goal_id: value.goal_id.into(),
            stamp: value.stamp.into(),
        }
    }
}

impl From<unique_identifier_msgs__msg__UUID> for UUID {
    fn from(value: unique_identifier_msgs__msg__UUID) -> Self {
        Self { uuid: value.uuid }
    }
}

impl From<crate::rcl::builtin_interfaces__msg__Time> for crate::msg::builtin_interfaces__msg__Time {
    fn from(value: crate::rcl::builtin_interfaces__msg__Time) -> Self {
        Self {
            sec: value.sec,
            nanosec: value.nanosec,
        }
    }
}
