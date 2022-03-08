/// TODO not all types need to be float, some should actually be u32, e.g. AWG Amplitude.
use std::time::Duration;

use clap::ArgEnum;
use strum_macros::{Display, EnumString};

// From GTK Adjustment.
#[derive(Debug)]
pub struct Adjustment {
    pub upper: f32,
    pub lower: f32,
    pub step: f32,
    pub page: f32,
}

impl Adjustment {
    pub fn new(upper: f32, lower: f32, step: f32, page: f32) -> Self {
        if upper <= lower {
            panic!(
                "upper is less than or equal to lower, upper={} lower={}",
                upper, lower
            );
        }
        if step == 0.0 {
            panic!("step is zero");
        }
        if page == 0.0 {
            panic!("page is zero");
        }
        Self {
            upper,
            lower,
            step,
            page,
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
}

#[derive(Display, Debug, Clone, EnumString, ArgEnum)]
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

#[derive(Display, Debug, Clone, EnumString, ArgEnum)]
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

#[derive(Display, Debug, Clone, EnumString, ArgEnum)]
pub enum Coupling {
    AC,
    DC,
    Ground,
}

impl Coupling {
    // Because CLion doesn't like the Display implemented by strum.
    pub fn my_to_string(&self) -> impl std::fmt::Display + '_ {
        self
    }

    pub fn raw_value(&self) -> u8 {
        match self {
            Self::AC => 0,
            Self::DC => 1,
            Self::Ground => 2,
        }
    }
}

#[derive(Display, Debug, Clone, EnumString, ArgEnum)]
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

    pub fn raw_value(&self) -> u8 {
        match self {
            Self::X1 => 0,
            Self::X10 => 1,
            Self::X100 => 2,
            Self::X1000 => 3,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Display, Debug, Clone, EnumString, ArgEnum)]
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

    pub fn raw_order(&self) -> u8 {
        match self {
            Self::mv10 => 0,
            Self::mv20 => 1,
            Self::mv50 => 2,
            Self::mv100 => 3,
            Self::mv200 => 4,
            Self::mv500 => 5,
            Self::v1 => 6,
            Self::v2 => 7,
            Self::v5 => 8,
            Self::v10 => 9,
            // Self::V20 => ?,
            // Self::V50 => ?,
            // Self::V100 => ?,
        }
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
#[derive(Display, Debug, Clone, EnumString, ArgEnum)]
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

    pub fn raw_value(&self) -> u8 {
        match self {
            Self::ns5 => 0,
            Self::ns10 => 1,
            Self::ns20 => 2,
            Self::ns50 => 3,
            Self::ns100 => 4,
            Self::ns200 => 5,
            Self::ns500 => 6,
            Self::us1 => 7,
            Self::us2 => 8,
            Self::us5 => 9,
            Self::us10 => 10,
            Self::us20 => 11,
            Self::us50 => 12,
            Self::us100 => 13,
            Self::us200 => 14,
            Self::us500 => 15,
            Self::ms1 => 16,
            Self::ms2 => 17,
            Self::ms5 => 18,
            Self::ms10 => 19,
            Self::ms20 => 20,
            Self::ms50 => 21,
            Self::ms100 => 22,
            Self::ms200 => 23,
            Self::ms500 => 24,
            Self::s1 => 25,
            Self::s2 => 26,
            Self::s5 => 27,
            Self::s10 => 28,
            Self::s20 => 29,
            Self::s50 => 30,
            Self::s100 => 31,
            Self::s200 => 32,
            Self::s500 => 33,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Display, Debug, Clone, EnumString, ArgEnum)]
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

    pub fn raw_value(&self) -> u8 {
        match self {
            Self::Rising => 0,
            Self::Falling => 1,
            Self::Both => 2,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Display, Debug, Clone, EnumString, ArgEnum)]
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

    pub fn raw_value(&self) -> u8 {
        match self {
            Self::Auto => 0,
            Self::Normal => 1,
            Self::Single => 2,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Display, Debug, Clone, EnumString, ArgEnum)]
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

    pub fn raw_value(&self) -> u8 {
        match self {
            AwgType::Square => 0,
            AwgType::Ramp => 1,
            AwgType::Sin => 2,
            AwgType::Trap => 3,
            AwgType::Arb1 => 4,
            AwgType::Arb2 => 5,
            AwgType::Arb3 => 6,
            AwgType::Arb4 => 7,
        }
    }
}

pub struct TrapDuty {
    pub high: f32,
    pub low: f32,
    pub rise: f32,
}

pub struct HantekConfig {
    timeout: Duration,

    device_function: Option<DeviceFunction>,

    enabled_channels: Vec<Option<bool>>,
    channel_coupling: Vec<Option<Coupling>>,
    channel_probe: Vec<Option<Probe>>,
    channel_scale: Vec<Option<Scale>>,
    channel_offset: Vec<Option<f32>>,
    channel_bandwidth_limit: Vec<Option<bool>>,
    channel_offset_adjustment: Vec<Option<Adjustment>>,

    time_scale: Option<TimeScale>,
    time_offset: Option<f32>,
    time_offset_adjustment: Option<Adjustment>,

    running_status: Option<RunningStatus>,
    trigger_source_channel: Option<usize>,
    trigger_slope: Option<TriggerSlope>,
    trigger_mode: Option<TriggerMode>,
    trigger_level_adjustment: Option<Adjustment>,
    trigger_level: Option<f32>,

    awg_type: Option<AwgType>,
    awg_frequency: Option<f32>,
    awg_amplitude: Option<f32>,
    awg_offset: Option<f32>,
    awg_duty_square: Option<f32>,
    awg_duty_ramp: Option<f32>,
    awg_duty_trap: Option<TrapDuty>,
    awg_running_status: Option<RunningStatus>,
}

impl HantekConfig {
    pub fn new(timeout: Duration, num_channels: usize) -> Self {
        Self {
            timeout,

            device_function: None,

            enabled_channels: vec![None; num_channels],
            channel_coupling: (0..num_channels).map(|_| None).collect(),
            channel_probe: (0..num_channels).map(|_| None).collect(),
            channel_scale: (0..num_channels).map(|_| None).collect(),
            channel_offset: (0..num_channels).map(|_| None).collect(),
            channel_bandwidth_limit: (0..num_channels).map(|_| None).collect(),
            channel_offset_adjustment: (0..num_channels).map(|_| None).collect(),

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

    pub fn get_timeout(&self) -> Duration {
        self.timeout
    }

    pub fn set_device_function(&mut self, function: DeviceFunction) {
        self.device_function = Some(function);
    }

    pub fn get_device_function(&self) -> Option<&DeviceFunction> {
        self.device_function.as_ref()
    }

    /// ============================================================ CHANNEL

    pub fn enable_channel(&mut self, channel_no: usize) {
        let my_channel_no = self.get_internal_channel_no(channel_no);
        self.enabled_channels[my_channel_no] = Some(true);
    }

    pub fn disable_channel(&mut self, channel_no: usize) {
        let my_channel_no = self.get_internal_channel_no(channel_no);
        self.enabled_channels[my_channel_no] = Some(false);
    }

    pub fn get_channel_enable_status(&self, channel_no: usize) -> Option<&bool> {
        let my_channel_no = self.get_internal_channel_no(channel_no);
        self.enabled_channels[my_channel_no].as_ref()
    }

    pub fn channel_disable_bandwidth_limit(&mut self, channel_no: usize) {
        let my_channel_no = self.get_internal_channel_no(channel_no);
        self.channel_bandwidth_limit[my_channel_no] = Some(true);
    }

    pub fn channel_enable_bandwidth_limit(&mut self, channel_no: usize) {
        let my_channel_no = self.get_internal_channel_no(channel_no);
        self.channel_bandwidth_limit[my_channel_no] = Some(false);
    }

    pub fn get_channel_bandwidth_limit_status(&self, channel_no: usize) -> Option<&bool> {
        let my_channel_no = self.get_internal_channel_no(channel_no);
        self.channel_bandwidth_limit[my_channel_no].as_ref()
    }

    pub fn set_channel_coupling(&mut self, channel_no: usize, coupling: Coupling) {
        let my_channel_no = self.get_internal_channel_no(channel_no);
        self.channel_coupling[my_channel_no] = Some(coupling);
    }

    pub fn get_channel_coupling(&self, channel_no: usize) -> Option<&Coupling> {
        let my_channel_no = self.get_internal_channel_no(channel_no);
        self.channel_coupling[my_channel_no].as_ref()
    }

    pub fn set_channel_probe(&mut self, channel_no: usize, probe: Probe) {
        let my_channel_no = self.get_internal_channel_no(channel_no);
        self.channel_probe[my_channel_no] = Some(probe);
    }

    pub fn get_channel_probe(&self, channel_no: usize) -> Option<&Probe> {
        let my_channel_no = self.get_internal_channel_no(channel_no);
        self.channel_probe[my_channel_no].as_ref()
    }

    pub fn set_channel_scale(&mut self, channel_no: usize, scale: Scale) {
        let my_channel_no = self.get_internal_channel_no(channel_no);
        self.channel_scale[my_channel_no] = Some(scale);
    }

    pub fn get_channel_scale(&self, channel_no: usize) -> Option<&Scale> {
        let my_channel_no = self.get_internal_channel_no(channel_no);
        self.channel_scale[my_channel_no].as_ref()
    }

    pub fn set_channel_offset(&mut self, channel_no: usize, offset: f32) {
        let my_channel_no = self.get_internal_channel_no(channel_no);
        self.channel_offset[my_channel_no] = Some(offset);
    }

    pub fn get_channel_offset(&self, channel_no: usize) -> Option<f32> {
        let my_channel_no = self.get_internal_channel_no(channel_no);
        self.channel_offset[my_channel_no]
    }

    pub fn set_channel_adjustment(
        &mut self,
        channel_no: usize,
        upper: f32,
        lower: f32,
        step: f32,
        page: f32,
    ) {
        let my_channel_no = self.get_internal_channel_no(channel_no);
        self.channel_offset_adjustment[my_channel_no] =
            Some(Adjustment::new(upper, lower, step, page));
    }

    pub fn get_channel_adjustment(&mut self, channel_no: usize) -> Option<&Adjustment> {
        let my_channel_no = self.get_internal_channel_no(channel_no);
        self.channel_offset_adjustment[my_channel_no].as_ref()
    }

    /// ================================================================== SCOPE

    pub fn set_time_scale(&mut self, time_scale: TimeScale) {
        self.time_scale = Some(time_scale);
    }

    pub fn get_timescale(&self) -> Option<&TimeScale> {
        self.time_scale.as_ref()
    }

    pub fn set_time_offset(&mut self, time_offset: f32) {
        self.time_offset = Some(time_offset);
    }

    pub fn get_time_offset(&self) -> Option<&f32> {
        self.time_offset.as_ref()
    }

    pub fn get_time_offset_adjustment(&self) -> Option<&Adjustment> {
        self.time_offset_adjustment.as_ref()
    }

    pub fn set_time_offset_adjustment(&mut self, upper: f32, lower: f32, step: f32, page: f32) {
        self.time_offset_adjustment = Some(Adjustment::new(upper, lower, step, page));
    }

    pub fn set_trigger_source_channel_no(&mut self, channel_no: usize) {
        self.get_internal_channel_no(channel_no);
        self.trigger_source_channel = Some(channel_no);
    }

    pub fn get_trigger_source_channel_no(&self) -> Option<&usize> {
        self.trigger_source_channel.as_ref()
    }

    pub fn set_trigger_slope(&mut self, trigger_slope: TriggerSlope) {
        self.trigger_slope = Some(trigger_slope);
    }

    pub fn get_trigger_slope(&self) -> Option<&TriggerSlope> {
        self.trigger_slope.as_ref()
    }

    pub fn set_trigger_mode(&mut self, trigger_mode: TriggerMode) {
        self.trigger_mode = Some(trigger_mode);
    }

    pub fn get_trigger_mode(&self) -> Option<&TriggerMode> {
        self.trigger_mode.as_ref()
    }

    pub fn set_trigger_level(&mut self, trigger_level: f32) {
        self.trigger_level = Some(trigger_level);
    }

    pub fn get_trigger_level(&self) -> Option<&f32> {
        self.trigger_level.as_ref()
    }

    pub fn set_trigger_level_adjustment(&mut self, upper: f32, lower: f32, step: f32, page: f32) {
        self.trigger_level_adjustment = Some(Adjustment::new(upper, lower, step, page));
    }

    pub fn get_trigger_level_adjustment(&self) -> Option<&Adjustment> {
        self.trigger_level_adjustment.as_ref()
    }

    pub fn start(&mut self) {
        self.running_status = Some(RunningStatus::Start);
    }

    pub fn stop(&mut self) {
        self.running_status = Some(RunningStatus::Stop);
    }

    pub fn get_running_status(&self) -> Option<&RunningStatus> {
        self.running_status.as_ref()
    }

    /// ==================================================================== AWG

    pub fn get_awg_type(&self) -> Option<&AwgType> {
        self.awg_type.as_ref()
    }

    pub fn set_awg_type(&mut self, awg_type: AwgType) {
        self.awg_type = Some(awg_type);
    }

    pub fn get_awg_frequency(&self) -> Option<&f32> {
        self.awg_frequency.as_ref()
    }

    pub fn set_awg_frequency(&mut self, frequency: f32) {
        self.awg_frequency = Some(frequency);
    }

    pub fn get_awg_amplitude(&self) -> Option<&f32> {
        self.awg_amplitude.as_ref()
    }

    pub fn set_awg_amplitude(&mut self, amplitude: f32) {
        self.awg_amplitude = Some(amplitude);
    }

    pub fn get_awg_offset(&self) -> Option<&f32> {
        self.awg_offset.as_ref()
    }

    pub fn set_awg_offset(&mut self, offset: f32) {
        self.awg_offset = Some(offset);
    }

    pub fn set_awg_duty_square(&mut self, duty: f32) {
        self.awg_duty_square = Some(duty);
    }

    pub fn get_awg_duty_square(&self) -> Option<&f32> {
        self.awg_duty_square.as_ref()
    }

    pub fn set_awg_duty_ramp(&mut self, duty: f32) {
        self.awg_duty_ramp = Some(duty);
    }

    pub fn get_awg_duty_ramp(&self) -> Option<&f32> {
        self.awg_duty_ramp.as_ref()
    }

    pub fn set_awg_duty_trap(&mut self, high: f32, low: f32, rise: f32) {
        self.awg_duty_trap = Some(TrapDuty { high, low, rise });
    }

    pub fn get_awg_duty_trap(&self) -> Option<&TrapDuty> {
        self.awg_duty_trap.as_ref()
    }

    pub fn awg_start(&mut self) {
        self.awg_running_status = Some(RunningStatus::Start);
    }

    pub fn awg_stop(&mut self) {
        self.awg_running_status = Some(RunningStatus::Stop);
    }

    pub fn get_awg_running_status(&self) -> Option<&RunningStatus> {
        self.awg_running_status.as_ref()
    }

    /// =============================================================== INTERNAL

    fn get_internal_channel_no(&self, channel_no: usize) -> usize {
        let my_channel_no = channel_no - 1;
        if my_channel_no >= self.enabled_channels.len() {
            panic!(
                "channel no out of range, available_channels=1..{} given_channel={}",
                self.enabled_channels.len(),
                channel_no
            );
        }
        my_channel_no
    }
}
