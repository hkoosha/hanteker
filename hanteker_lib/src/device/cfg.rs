//! TODO not all types need to be float, some should actually be u32, e.g. AWG Amplitude.

use std::collections::HashMap;
use std::fmt::{Display, Formatter};
#[cfg(feature = "gui")]
use std::hash::Hash;
use std::time::Duration;

#[cfg(feature = "cli")]
use clap::ArgEnum;
#[cfg(feature = "gui")]
use druid::Data;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString, EnumVariantNames};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "gui", derive(Data))]
pub struct Adjustment {
    pub upper: f32,
    pub lower: f32,
}

impl Adjustment {
    pub const ZERO: Adjustment = Adjustment { lower: 0.0, upper: 0.0 };

    pub fn new(mut upper: f32, mut lower: f32) -> Self {
        // -0.0 to 0.0.
        if upper == 0.0 {
            upper = 0.0;
        }
        if lower == 0.0 {
            lower = 0.0;
        }

        if upper < lower {
            panic!(
                "upper is less than or equal to lower, upper={} lower={}, upper_repr={}, lower_repr={}",
                upper, lower, upper.to_bits(), upper.to_bits(),
            );
        }

        Self {
            lower: if lower < upper { lower } else { upper },
            upper: if lower < upper { upper } else { lower },
        }
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

    pub fn same(&self, other: &Self) -> bool {
        (self.upper == other.upper && self.lower == other.lower)
            || (self.upper.to_bits() == other.upper.to_bits()
            && self.lower.to_bits() == other.lower.to_bits())
    }
}

#[derive(Display, Debug, Clone, EnumString, EnumIter, EnumVariantNames, PartialEq, Eq)]
#[cfg_attr(feature = "cli", derive(ArgEnum))]
#[cfg_attr(feature = "gui", derive(Data))]
pub enum DeviceFunction {
    Scope,
    AWG,
    DMM,
}

impl DeviceFunction {
    pub fn my_iter() -> impl Iterator<Item=DeviceFunction> {
        Self::iter()
    }

    pub fn my_options() -> Vec<(String, Self)> {
        Self::my_iter()
            .map(|it| {
                let as_string = it.my_to_string().to_string();
                (as_string, it)
            })
            .collect()
    }

    // Because CLion doesn't like the Display implemented by strum.
    pub fn my_to_string(&self) -> impl std::fmt::Display + '_ {
        self
    }
}

#[derive(Display, Debug, Clone, EnumString, EnumIter, EnumVariantNames, PartialEq, Eq)]
#[cfg_attr(feature = "cli", derive(ArgEnum))]
#[cfg_attr(feature = "gui", derive(Data))]
pub enum RunningStatus {
    Start,
    Stop,
}

impl RunningStatus {
    pub fn my_iter() -> impl Iterator<Item=RunningStatus> {
        Self::iter()
    }

    pub fn my_options() -> Vec<(String, Self)> {
        Self::my_iter()
            .map(|it| {
                let as_string = it.my_to_string().to_string();
                (as_string, it)
            })
            .collect()
    }

    // Because CLion doesn't like the Display implemented by strum.
    pub fn my_to_string(&self) -> impl std::fmt::Display + '_ {
        self
    }

    pub fn is_start(&self) -> bool {
        *self == Self::Start
    }

    pub fn is_stop(&self) -> bool {
        *self == Self::Stop
    }
}

#[derive(Display, Debug, Clone, EnumString, EnumIter, EnumVariantNames, PartialEq, Eq)]
#[cfg_attr(feature = "cli", derive(ArgEnum))]
#[cfg_attr(feature = "gui", derive(Data))]
pub enum Coupling {
    AC,
    DC,
    GND,
}

impl Coupling {
    pub fn my_iter() -> impl Iterator<Item=Coupling> {
        Self::iter()
    }

    pub fn my_options() -> Vec<(String, Self)> {
        Self::my_iter()
            .map(|it| {
                let as_string = it.my_to_string().to_string();
                (as_string, it)
            })
            .collect()
    }

    // Because CLion doesn't like the Display implemented by strum.
    pub fn my_to_string(&self) -> impl std::fmt::Display + '_ {
        self
    }
}

#[derive(Display, Debug, Clone, EnumString, EnumIter, EnumVariantNames, PartialEq, Eq)]
#[cfg_attr(feature = "cli", derive(ArgEnum))]
#[cfg_attr(feature = "gui", derive(Data))]
pub enum Probe {
    X1,
    X10,
    X100,
    X1000,
}

impl Probe {
    pub fn my_iter() -> impl Iterator<Item=Probe> {
        Self::iter()
    }

    pub fn my_options() -> Vec<(String, Self)> {
        Self::my_iter()
            .map(|it| {
                let as_string = it.my_to_string().to_string();
                (as_string, it)
            })
            .collect()
    }

    // Because CLion doesn't like the Display implemented by strum.
    pub fn my_to_string(&self) -> impl std::fmt::Display + '_ {
        self
    }
}

#[allow(non_camel_case_types)]
#[derive(Display, Debug, Clone, EnumString, EnumIter, EnumVariantNames, PartialEq, Eq)]
#[cfg_attr(feature = "cli", derive(ArgEnum))]
#[cfg_attr(feature = "gui", derive(Data))]
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
    pub fn my_iter() -> impl Iterator<Item=Scale> {
        Self::iter()
    }

    pub fn my_options() -> Vec<(String, Self)> {
        Self::my_iter()
            .map(|it| {
                let as_string = it.my_to_string().to_string();
                (as_string, it)
            })
            .collect()
    }

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
#[derive(Display, Debug, Clone, EnumString, EnumIter, EnumVariantNames, PartialEq, Eq)]
#[cfg_attr(feature = "cli", derive(ArgEnum))]
#[cfg_attr(feature = "gui", derive(Data))]
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
    pub fn my_iter() -> impl Iterator<Item=TimeScale> {
        Self::iter()
    }

    pub fn my_options() -> Vec<(String, Self)> {
        Self::my_iter()
            .map(|it| {
                let as_string = it.my_to_string().to_string();
                (as_string, it)
            })
            .collect()
    }

    // Because CLion doesn't like the Display implemented by strum.
    pub fn my_to_string(&self) -> impl std::fmt::Display + '_ {
        self
    }
}

#[allow(non_camel_case_types)]
#[derive(Display, Debug, Clone, EnumString, EnumIter, EnumVariantNames, PartialEq, Eq)]
#[cfg_attr(feature = "cli", derive(ArgEnum))]
#[cfg_attr(feature = "gui", derive(Data))]
pub enum TriggerSlope {
    Rising,
    Falling,
    Both,
}

impl TriggerSlope {
    pub fn my_iter() -> impl Iterator<Item=TriggerSlope> {
        Self::iter()
    }

    pub fn my_options() -> Vec<(String, Self)> {
        Self::my_iter()
            .map(|it| {
                let as_string = it.my_to_string().to_string();
                (as_string, it)
            })
            .collect()
    }

    // Because CLion doesn't like the Display implemented by strum.
    pub fn my_to_string(&self) -> impl std::fmt::Display + '_ {
        self
    }
}

#[allow(non_camel_case_types)]
#[derive(Display, Debug, Clone, EnumString, EnumIter, EnumVariantNames, PartialEq, Eq)]
#[cfg_attr(feature = "cli", derive(ArgEnum))]
#[cfg_attr(feature = "gui", derive(Data))]
pub enum TriggerMode {
    Auto,
    Normal,
    Single,
}

impl TriggerMode {
    pub fn my_iter() -> impl Iterator<Item=TriggerMode> {
        Self::iter()
    }

    pub fn my_options() -> Vec<(String, Self)> {
        Self::my_iter()
            .map(|it| {
                let as_string = it.my_to_string().to_string();
                (as_string, it)
            })
            .collect()
    }

    // Because CLion doesn't like the Display implemented by strum.
    pub fn my_to_string(&self) -> impl std::fmt::Display + '_ {
        self
    }
}

#[allow(non_camel_case_types)]
#[derive(Display, Debug, Clone, EnumString, EnumIter, EnumVariantNames, PartialEq, Eq)]
#[cfg_attr(feature = "cli", derive(ArgEnum))]
#[cfg_attr(feature = "gui", derive(Data))]
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
    pub fn my_iter() -> impl Iterator<Item=AwgType> {
        Self::iter()
    }

    pub fn my_options() -> Vec<(String, Self)> {
        Self::my_iter()
            .map(|it| {
                let as_string = it.my_to_string().to_string();
                (as_string, it)
            })
            .collect()
    }

    // Because CLion doesn't like the Display implemented by strum.
    pub fn my_to_string(&self) -> impl std::fmt::Display + '_ {
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "gui", derive(Data))]
pub struct TrapDuty {
    pub high: f32,
    pub low: f32,
    pub rise: f32,
}

impl TrapDuty {
    pub const ZERO: TrapDuty = TrapDuty { high: 0.0, low: 0.0, rise: 0.0 };

    pub fn same(&self, other: &Self) -> bool {
        (self.high == other.high && self.low == other.low && self.rise == other.rise)
            || (self.high.to_bits() == other.high.to_bits()
            && self.low.to_bits() == other.low.to_bits()
            && self.rise.to_bits() == other.rise.to_bits())
    }
}

impl Display for TrapDuty {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "TrapDuty{{high: {}, low: {}, rise: {}}}", self.high, self.low, self.rise)
    }
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

#[cfg(feature = "gui")]
impl Data for HantekConfig {
    fn same(&self, other: &Self) -> bool {
        if self.timeout != other.timeout {
            return false;
        }

        if self.device_function != other.device_function {
            return false;
        }

        if self.enabled_channels != other.enabled_channels {
            return false;
        }
        if self.channel_coupling != other.channel_coupling {
            return false;
        }
        if self.channel_probe != other.channel_probe {
            return false;
        }
        if self.channel_scale != other.channel_scale {
            return false;
        }
        if self.channel_bandwidth_limit != other.channel_bandwidth_limit {
            return false;
        }

        if !compare_map(
            &self.channel_offset,
            &other.channel_offset,
            compare_some_f32,
        ) {
            return false;
        }

        if !compare_map(
            &self.channel_offset_adjustment,
            &other.channel_offset_adjustment,
            compare_some_adjustment,
        ) {
            return false;
        }

        if self.time_scale != other.time_scale {
            return false;
        }
        if !compare_some_f32(&self.time_offset, &other.time_offset) {
            return false;
        }
        if !compare_some_adjustment(&self.time_offset_adjustment, &other.time_offset_adjustment) {
            return false;
        }

        if self.running_status != other.running_status {
            return false;
        }
        if self.trigger_source_channel != other.trigger_source_channel {
            return false;
        }
        if self.trigger_slope != other.trigger_slope {
            return false;
        }
        if self.trigger_mode != other.trigger_mode {
            return false;
        }

        if !compare_some_adjustment(
            &self.trigger_level_adjustment,
            &other.trigger_level_adjustment,
        ) {
            return false;
        }
        if !compare_some_f32(&self.trigger_level, &other.trigger_level) {
            return false;
        }

        if self.awg_type != other.awg_type {
            return false;
        }

        if !compare_some_f32(&self.awg_frequency, &other.awg_frequency) {
            return false;
        }
        if !compare_some_f32(&self.awg_amplitude, &other.awg_amplitude) {
            return false;
        }
        if !compare_some_f32(&self.awg_offset, &other.awg_offset) {
            return false;
        }
        if !compare_some_f32(&self.awg_duty_square, &other.awg_duty_square) {
            return false;
        }
        if !compare_some_f32(&self.awg_duty_ramp, &other.awg_duty_ramp) {
            return false;
        }
        if !compare_some_trap_duty(&self.awg_duty_trap, &other.awg_duty_trap) {
            return false;
        }
        if self.awg_running_status != other.awg_running_status {
            return false;
        }

        true
    }
}

#[cfg(feature = "gui")]
fn compare_some_trap_duty(t0: &Option<TrapDuty>, t1: &Option<TrapDuty>) -> bool {
    if t0.is_some() != t1.is_some() {
        false
    } else if t0.is_some() {
        let t0 = t0.as_ref().unwrap();
        let t1 = t1.as_ref().unwrap();
        t0.same(t1)
    } else {
        true
    }
}

#[cfg(feature = "gui")]
fn compare_some_f32(f0: &Option<f32>, f1: &Option<f32>) -> bool {
    if f0.is_some() != f1.is_some() {
        false
    } else if f0.is_some() {
        let f0 = f0.unwrap().to_bits();
        let f1 = f1.unwrap().to_bits();
        f0 == f1
    } else {
        true
    }
}

#[cfg(feature = "gui")]
fn compare_some_adjustment(a0: &Option<Adjustment>, a1: &Option<Adjustment>) -> bool {
    if a0.is_some() != a1.is_some() {
        false
    } else if a0.is_some() {
        let a0 = a0.as_ref().unwrap();
        let a1 = a1.as_ref().unwrap();
        a0.same(a1)
    } else {
        true
    }
}

#[cfg(feature = "gui")]
fn compare_map<K: std::cmp::Eq + Hash, V>(
    m0: &HashMap<K, V>,
    m1: &HashMap<K, V>,
    comparator: impl Fn(&V, &V) -> bool,
) -> bool {
    m0.len() == m1.len()
        && m0.keys().all(|k| m1.contains_key(k))
        && m0.iter().all(|(k0, v0)| comparator(v0, &m1[k0]))
}
