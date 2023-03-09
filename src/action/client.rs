use std::{collections::BTreeMap, ffi::CString, mem::MaybeUninit, sync::Arc};

use crate::{
    error::{DynError, RCLActionError, RCLActionResult, RCLError},
    get_allocator,
    msg::{interfaces::action_msgs::msg::GoalStatusArray, ActionMsg},
    node::Node,
    qos::Profile,
    rcl, RecvResult,
};

use super::{
    GetResultServiceRequest, GetResultServiceResponse, SendGoalServiceRequest,
    SendGoalServiceResponse,
};

pub struct ClientQosOption {
    goal_service: Profile,
    result_service: Profile,
    cancel_service: Profile,
    feedback_topic: Profile,
    status_topic: Profile,
}

impl From<ClientQosOption> for rcl::rcl_action_client_options_t {
    fn from(opts: ClientQosOption) -> Self {
        rcl::rcl_action_client_options_t {
            goal_service_qos: (&opts.goal_service).into(),
            result_service_qos: (&opts.result_service).into(),
            cancel_service_qos: (&opts.cancel_service).into(),
            feedback_topic_qos: (&opts.feedback_topic).into(),
            status_topic_qos: (&opts.status_topic).into(),
            allocator: get_allocator(),
        }
    }
}

/// An action client.
pub struct Client<T: ActionMsg> {
    client: rcl::rcl_action_client_t,
    node: Arc<Node>,

    goal_response_callbacks: BTreeMap<i64, Box<dyn FnOnce(SendGoalServiceResponse<T>)>>,
    result_response_callbacks: BTreeMap<i64, Box<dyn FnOnce(GetResultServiceResponse<T>)>>,
}

impl<T> Client<T>
where
    T: ActionMsg,
{
    // Create a client.
    pub fn new(
        node: Arc<Node>,
        action_name: &str,
        qos: Option<ClientQosOption>,
    ) -> RCLActionResult<Self> {
        let mut client = rcl::MTSafeFn::rcl_action_get_zero_initialized_client();
        let options = qos
            .map(rcl::rcl_action_client_options_t::from)
            .unwrap_or(rcl::MTSafeFn::rcl_action_client_get_default_options());
        let action_name = CString::new(action_name).unwrap_or_default();

        {
            let guard = rcl::MT_UNSAFE_FN.lock();
            guard.rcl_action_client_init(
                &mut client,
                unsafe { node.as_ptr_mut() },
                T::type_support(),
                action_name.as_ptr(),
                &options,
            )?;
        }

        Ok(Self {
            client,
            node,
            goal_response_callbacks: Default::default(),
            result_response_callbacks: Default::default(),
        })
    }

    pub fn is_server_available(&self) -> RCLActionResult<bool> {
        let guard = rcl::MT_UNSAFE_FN.lock();

        let mut is_available = false;
        match guard.rcl_action_server_is_available(
            self.node.as_ptr(),
            &self.client,
            &mut is_available as *mut _,
        ) {
            Ok(()) => Ok(is_available),
            Err(RCLActionError::RCLError(RCLError::NodeInvalid)) => {
                // TODO: soft failure in case of shutdown context
                eprintln!("Invalid node (the shutdown has started?)");
                Ok(false)
            }
            Err(e) => Err(e.into()),
        }
    }

    // Send a goal request to the server. the UUID are automatically attached.
    // pub fn send_goal(&mut self, goal: &<<T as ActionMsg>::Goal as ServiceMsg>

    // )

    // pub fn send_goal_with_uuid()

    // Send a goal request.
    // TODO: this shouldn't be pub?
    pub fn send_goal_request(
        &mut self,
        data: &SendGoalServiceRequest<T>,
        callback: Box<dyn FnOnce(SendGoalServiceResponse<T>)>,
    ) -> Result<(), DynError> {
        // TODO: use mutex?
        // TODO: callbacks are FnMut? Fn? FnOnce?

        let mut seq: i64 = 0;
        rcl::MTSafeFn::rcl_action_send_goal_request(
            &self.client,
            data as *const SendGoalServiceRequest<T> as _,
            &mut seq,
        )?;

        if self.goal_response_callbacks.contains_key(&seq) {
            Err(format!(
                "A goal callback with sequence number {} already exists",
                seq
            )
            .into())
        } else {
            self.goal_response_callbacks.insert(seq, callback);
            Ok(())
        }
    }

    // TODO: what should be the second type argument for RecvResult
    // TODO: return Result<()>?
    pub fn try_recv_goal_response(&mut self) -> RecvResult<(), ()> {
        let guard = rcl::MT_UNSAFE_FN.lock();

        let mut header: rcl::rmw_request_id_t = unsafe { MaybeUninit::zeroed().assume_init() };
        let mut response: SendGoalServiceResponse<T> =
            unsafe { MaybeUninit::zeroed().assume_init() };
        match guard.rcl_action_take_goal_response(
            &self.client,
            &mut header,
            &mut response as *const _ as *mut _,
        ) {
            Ok(()) => {
                let seq = header.sequence_number;
                match self.goal_response_callbacks.remove(&seq) {
                    Some(callback) => callback(response),
                    None => {
                        return RecvResult::Err(
                            format!(
                                "The goal callback with sequence number {} was not found",
                                seq
                            )
                            .into(),
                        );
                    }
                }

                RecvResult::Ok(())
            }
            Err(RCLActionError::ClientTakeFailed) => RecvResult::RetryLater(()),
            Err(e) => RecvResult::Err(e.into()),
        }
    }

    // TODO: ergonomic api, maybe return [GoalStatus], GoalStatusArray is a rosidl message type
    pub fn take_status(&self) -> RCLActionResult<GoalStatusArray> {
        let guard = rcl::MT_UNSAFE_FN.lock();

        let mut status_array: GoalStatusArray = unsafe { MaybeUninit::zeroed().assume_init() };
        guard.rcl_action_take_status(&self.client, &mut status_array as *const _ as *mut _)?;

        Ok(status_array)
    }

    pub fn send_result_request(
        &mut self,
        data: &GetResultServiceRequest<T>,
        callback: Box<dyn FnOnce(GetResultServiceResponse<T>)>,
    ) -> Result<(), DynError> {
        // TODO: use mutex?

        let mut seq: i64 = 0;
        rcl::MTSafeFn::rcl_action_send_result_request(
            &self.client,
            data as *const GetResultServiceRequest<T> as _,
            &mut seq,
        )?;

        if self.result_response_callbacks.contains_key(&seq) {
            Err(format!(
                "A goal callback with sequence number {} already exists",
                seq
            )
            .into())
        } else {
            self.result_response_callbacks.insert(seq, callback);
            Ok(())
        }
    }

    // Takes a result for the goal.
    // TODO: or take_result_response
    pub fn try_recv_result_response(&self) -> RCLActionResult<GetResultServiceResponse<T>> {
        let guard = rcl::MT_UNSAFE_FN.lock();

        let mut header: rcl::rmw_request_id_t = unsafe { MaybeUninit::zeroed().assume_init() };
        let mut response: GetResultServiceResponse<T> =
            unsafe { MaybeUninit::zeroed().assume_init() };
        guard.rcl_action_take_result_response(
            &self.client,
            &mut header,
            &mut response as *const _ as *mut _,
        )?;

        Ok(response)
    }

    // Takes a feedback for the goal.
    pub fn try_recv_feedback(&self) -> RCLActionResult<<T as ActionMsg>::Feedback> {
        let guard = rcl::MT_UNSAFE_FN.lock();

        let mut feedback: <T as ActionMsg>::Feedback =
            unsafe { MaybeUninit::zeroed().assume_init() };
        guard.rcl_action_take_feedback(&self.client, &mut feedback as *const _ as *mut _)?;

        Ok(feedback)
    }
}

impl<T: ActionMsg> Drop for Client<T> {
    fn drop(&mut self) {
        let guard = rcl::MT_UNSAFE_FN.lock();
        let _ = guard.rcl_action_client_fini(&mut self.client, unsafe { self.node.as_ptr_mut() });
    }
}