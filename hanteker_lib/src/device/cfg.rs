use std::collections::HashMap;
/// TODO not all types need to be float, some should actually be u32, e.g. AWG Amplitude.
use std::time::Duration;

use clap::ArgEnum;
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, PartialEq)]
pub struct Adjustment {
    pub upper: f32,
    pub lower: f32,
}

impl Adjustment {
    pub fn new(upper: f32, lower: f32) -> Self {
        if upper <= lower {
            panic!(
                "upper is less than or equal to lower, upper={} lower={}",
                upper, lower
            );
        }
        Self { upper, lower }
    }

    pub fn are_limits_sane(&self) -> bool {
        self.upper.is_finite()
            && !self.upper.is_nan()
            && self.lower.is_finite()
            && !self.lower.is_nan()
    }

    pub fn limits_are_zero(&self) -> bool {
        self.upper == 0.0 && self.lower == 0.0
    }
}

#[derive(Display, Debug, Clone, EnumString, ArgEnum, PartialEq, Eq)]
pub enum DeviceFunction {
    Scope,
    AWG,
    DMM,
}

impl DeviceFunction {
    // Because CLion doesn't like the Display implemented by strum.
    pub fn my_to_string(&self) -> impl std::fmt::Display + '_ {
        self
    }
}

#[derive(Display, Debug, Clone, EnumString, ArgEnum, PartialEq, Eq)]
pub enum RunningStatus {
    Start,
    Stop,
}

impl RunningStatus {
    // Because CLion doesn't like the Display implemented by strum.
    pub fn my_to_string(&self) -> impl std::fmt::Display + '_ {
        self
    }
}

#[derive(Display, Debug, Clone, EnumString, ArgEnum, PartialEq, Eq)]
pub enum Coupling {
    AC,
    DC,
    GND,
}

impl Coupling {
    // Because CLion doesn't like the Display implemented by strum.
    pub fn my_to_string(&self) -> impl std::fmt::Display + '_ {
        self
    }
}

#[derive(Display, Debug, Clone, EnumString, ArgEnum, PartialEq, Eq)]
pub enum Probe {
    X1,
    X10,
    X100,
    X1000,
}

impl Probe {
    // Because CLion doesn't like the Display implemented by strum.
    pub fn my_to_string(&self) -> impl std::fmt::Display + '_ {
        self
    }
}

#[allow(non_camel_case_types)]
#[derive(Display, Debug, Clone, EnumString, ArgEnum, PartialEq, Eq)]
pub enum Scale {
    mv10,
    mv20,
    mv50,
    mv100,
    mv200,
    mv500,
    v1,
    v2,
    v5,
    v10,
    // v20,
    // v50,
    // v100,
}

impl Scale {
    // Because CLion doesn't like the Display implemented by strum.
    pub fn my_to_string(&self) -> impl std::fmt::Display + '_ {
        self
    }

    pub fn raw_value(&self) -> f32 {
        match self {
            Self::mv10 => 0.01,
            Self::mv20 => 0.02,
            Self::mv50 => 0.05,
            Self::mv100 => 0.1,
            Self::mv200 => 0.2,
            Self::mv500 => 0.5,
            Self::v1 => 1.0,
            Self::v2 => 2.0,
            Self::v5 => 5.0,
            Self::v10 => 10.0,
            // Self::V20 => ?,
            // Self::V50 => ?,
            // Self::V100 => ?,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Display, Debug, Clone, EnumString, ArgEnum, PartialEq, Eq)]
pub enum TimeScale {
    ns5,
    ns10,
    ns20,
    ns50,
    ns100,
    ns200,
    ns500,
    us1,
    us2,
    us5,
    us10,
    us20,
    us50,
    us100,
    us200,
    us500,
    ms1,
    ms2,
    ms5,
    ms10,
    ms20,
    ms50,
    ms100,
    ms200,
    ms500,
    s1,
    s2,
    s5,
    s10,
    s20,
    s50,
    s100,
    s200,
    s500,
}

impl TimeScale {
    // Because CLion doesn't like the Display implemented by strum.
    pub fn my_to_string(&self) -> impl std::fmt::Display + '_ {
        self
    }
}

#[allow(non_camel_case_types)]
#[derive(Display, Debug, Clone, EnumString, ArgEnum, PartialEq, Eq)]
pub enum TriggerSlope {
    Rising,
    Falling,
    Both,
}

impl TriggerSlope {
    // Because CLion doesn't like the Display implemented by strum.
    pub fn my_to_string(&self) -> impl std::fmt::Display + '_ {
        self
    }
}

#[allow(non_camel_case_types)]
#[derive(Display, Debug, Clone, EnumString, ArgEnum, PartialEq, Eq)]
pub enum TriggerMode {
    Auto,
    Normal,
    Single,
}

impl TriggerMode {
    // Because CLion doesn't like the Display implemented by strum.
    pub fn my_to_string(&self) -> impl std::fmt::Display + '_ {
        self
    }
}

#[allow(non_camel_case_types)]
#[derive(Display, Debug, Clone, EnumString, ArgEnum, PartialEq, Eq)]
pub enum AwgType {
    Square,
    Ramp,
    Sin,
    Trap,
    Arb1,
    Arb2,
    Arb3,
    Arb4,
}

impl AwgType {
    // Because CLion doesn't like the Display implemented by strum.
    pub fn my_to_string(&self) -> impl std::fmt::Display + '_ {
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TrapDuty {
    pub high: f32,
    pub low: f32,
    pub rise: f32,
}

#[derive(Debug, Clone)]
pub struct HantekConfig {
    pub timeout: Option<Duration>,

    pub device_function: Option<DeviceFunction>,

    pub enabled_channels: HashMap<usize, Option<bool>>,
    pub channel_coupling: HashMap<usize, Option<Coupling>>,
    pub channel_probe: HashMap<usize, Option<Probe>>,
    pub channel_scale: HashMap<usize, Option<Scale>>,
    pub channel_offset: HashMap<usize, Option<f32>>,
    pub channel_bandwidth_limit: HashMap<usize, Option<bool>>,
    pub channel_offset_adjustment: HashMap<usize, Option<Adjustment>>,

    pub time_scale: Option<TimeScale>,
    pub time_offset: Option<f32>,
    pub time_offset_adjustment: Option<Adjustment>,

    pub running_status: Option<RunningStatus>,
    pub trigger_source_channel: Option<usize>,
    pub trigger_slope: Option<TriggerSlope>,
    pub trigger_mode: Option<TriggerMode>,
    pub trigger_level_adjustment: Option<Adjustment>,
    pub trigger_level: Option<f32>,

    pub awg_type: Option<AwgType>,
    pub awg_frequency: Option<f32>,
    pub awg_amplitude: Option<f32>,
    pub awg_offset: Option<f32>,
    pub awg_duty_square: Option<f32>,
    pub awg_duty_ramp: Option<f32>,
    pub awg_duty_trap: Option<TrapDuty>,
    pub awg_running_status: Option<RunningStatus>,
}

impl HantekConfig {
    pub fn new(num_channels: usize) -> Self {
        Self {
            timeout: None,

            device_function: None,

            enabled_channels: (1..=num_channels).map(|idx| (idx, None)).collect(),
            channel_coupling: (1..=num_channels).map(|idx| (idx, None)).collect(),
            channel_probe: (1..=num_channels).map(|idx| (idx, None)).collect(),
            channel_scale: (1..=num_channels).map(|idx| (idx, None)).collect(),
            channel_offset: (1..=num_channels).map(|idx| (idx, None)).collect(),
            channel_bandwidth_limit: (1..=num_channels).map(|idx| (idx, None)).collect(),
            channel_offset_adjustment: (1..=num_channels).map(|idx| (idx, None)).collect(),

            time_scale: None,
            time_offset: None,
            time_offset_adjustment: None,

            running_status: None,
            trigger_source_channel: None,
            trigger_slope: None,
            trigger_mode: None,
            trigger_level_adjustment: None,
            trigger_level: None,

            awg_type: None,
            awg_frequency: None,
            awg_amplitude: None,
            awg_offset: None,
            awg_duty_square: None,
            awg_duty_ramp: None,
            awg_duty_trap: None,
            awg_running_status: None,
        }
    }
}
