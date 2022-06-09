// This file was automatically generated by ros2msg_to_rs (https://github.com/tier4/ros2msg_to_rs).
use super::*;
use super::super::super::*;
use crate::msg::*;
use crate::rcl;
pub const POWER_SUPPLY_STATUS_UNKNOWN: u8 = 0;
pub const POWER_SUPPLY_STATUS_CHARGING: u8 = 1;
pub const POWER_SUPPLY_STATUS_DISCHARGING: u8 = 2;
pub const POWER_SUPPLY_STATUS_NOT_CHARGING: u8 = 3;
pub const POWER_SUPPLY_STATUS_FULL: u8 = 4;
pub const POWER_SUPPLY_HEALTH_UNKNOWN: u8 = 0;
pub const POWER_SUPPLY_HEALTH_GOOD: u8 = 1;
pub const POWER_SUPPLY_HEALTH_OVERHEAT: u8 = 2;
pub const POWER_SUPPLY_HEALTH_DEAD: u8 = 3;
pub const POWER_SUPPLY_HEALTH_OVERVOLTAGE: u8 = 4;
pub const POWER_SUPPLY_HEALTH_UNSPEC_FAILURE: u8 = 5;
pub const POWER_SUPPLY_HEALTH_COLD: u8 = 6;
pub const POWER_SUPPLY_HEALTH_WATCHDOG_TIMER_EXPIRE: u8 = 7;
pub const POWER_SUPPLY_HEALTH_SAFETY_TIMER_EXPIRE: u8 = 8;
pub const POWER_SUPPLY_TECHNOLOGY_UNKNOWN: u8 = 0;
pub const POWER_SUPPLY_TECHNOLOGY_NIMH: u8 = 1;
pub const POWER_SUPPLY_TECHNOLOGY_LION: u8 = 2;
pub const POWER_SUPPLY_TECHNOLOGY_LIPO: u8 = 3;
pub const POWER_SUPPLY_TECHNOLOGY_LIFE: u8 = 4;
pub const POWER_SUPPLY_TECHNOLOGY_NICD: u8 = 5;
pub const POWER_SUPPLY_TECHNOLOGY_LIMN: u8 = 6;

extern "C" {
    fn sensor_msgs__msg__BatteryState__init(msg: *mut BatteryState) -> bool;
    fn sensor_msgs__msg__BatteryState__fini(msg: *mut BatteryState);
    fn sensor_msgs__msg__BatteryState__Sequence__init(msg: *mut BatteryStateSequence, size: usize) -> bool;
    fn sensor_msgs__msg__BatteryState__Sequence__fini(msg: *mut BatteryStateSequence);
    fn rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__BatteryState() -> *const rcl::rosidl_message_type_support_t;
}


#[repr(C)]
#[derive(Debug)]
pub struct BatteryState {
    pub header: std_msgs::msg::Header,
    pub voltage: f32,
    pub temperature: f32,
    pub current: f32,
    pub charge: f32,
    pub capacity: f32,
    pub design_capacity: f32,
    pub percentage: f32,
    pub power_supply_status: u8,
    pub power_supply_health: u8,
    pub power_supply_technology: u8,
    pub present: bool,
    pub cell_voltage: crate::msg::F32Seq<0>,
    pub cell_temperature: crate::msg::F32Seq<0>,
    pub location: crate::msg::RosString<0>,
    pub serial_number: crate::msg::RosString<0>,
}

impl BatteryState {
    pub fn new() -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__BatteryState__init(&mut msg) } {
            Some(msg)
        } else {
            None
        }
    }
}

impl Drop for BatteryState {
    fn drop(&mut self) {
        unsafe { sensor_msgs__msg__BatteryState__fini(self) };
    }
}

impl TopicMsg for BatteryState {
    fn type_support() -> *const rcl::rosidl_message_type_support_t {
        unsafe {
            rosidl_typesupport_c__get_message_type_support_handle__sensor_msgs__msg__BatteryState()
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct BatteryStateSequence {
    data: *mut BatteryState,
    size: usize,
    capacity: usize,
}

impl BatteryStateSequence {
    pub fn new(size: usize) -> Option<Self> {
        let mut msg: Self = unsafe { std::mem::MaybeUninit::zeroed().assume_init() };
        if unsafe { sensor_msgs__msg__BatteryState__Sequence__init(&mut msg, size) } {
            Some(msg)
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> Option<&[BatteryState]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts(self.data, self.size) };
            Some(s)
        }
    }

    pub fn as_slice_mut(&mut self) -> Option<&mut [BatteryState]> {
        if self.data.is_null() {
            None
        } else {
            let s = unsafe { std::slice::from_raw_parts_mut(self.data, self.size) };
            Some(s)
        }
    }
}

impl Drop for BatteryStateSequence {
    fn drop(&mut self) {
        unsafe { sensor_msgs__msg__BatteryState__Sequence__fini(self) };
    }
}

