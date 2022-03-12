#![allow(non_upper_case_globals)]

pub(crate) const VENDOR_ID__2D42: u16 = 0x0483;
pub(crate) const PRODUCT_ID__2D42: u16 = 0x2d42;

pub(crate) const FUNC_SCOPE_SETTING: u16 = 0x0000;
pub(crate) const FUNC_SCOPE_CAPTURE: u16 = 0x0100;
pub(crate) const FUNC_AWG_SETTING: u16 = 0x0002;
pub(crate) const FUNC_SCREEN_SETTING: u16 = 0x0003;

pub(crate) const SCOPE_ENABLE_CH1: u8 = 0x00;
pub(crate) const SCOPE_COUPLING_CH1: u8 = 0x01;
pub(crate) const SCOPE_PROBE_X_CH1: u8 = 0x02;
pub(crate) const SCOPE_BW_LIMIT_CH1: u8 = 0x03;
pub(crate) const SCOPE_SCALE_CH1: u8 = 0x04;
pub(crate) const SCOPE_OFFSET_CH1: u8 = 0x05;

pub(crate) const SCOPE_ENABLE_CH2: u8 = 0x06;
pub(crate) const SCOPE_COUPLING_CH2: u8 = 0x07;
pub(crate) const SCOPE_PROBE_X_CH2: u8 = 0x08;
pub(crate) const SCOPE_BW_LIMIT_CH2: u8 = 0x09;
pub(crate) const SCOPE_SCALE_CH2: u8 = 0x0A;
pub(crate) const SCOPE_OFFSET_CH2: u8 = 0x0B;

pub(crate) const SCOPE_START_STOP: u8 = 0x0C;

pub(crate) const SCOPE_SCALE_TIME: u8 = 0x0E;
pub(crate) const SCOPE_OFFSET_TIME: u8 = 0x0F;

pub(crate) const SCOPE_TRIGGER_SOURCE: u8 = 0x10;
pub(crate) const SCOPE_TRIGGER_SLOPE: u8 = 0x11;
pub(crate) const SCOPE_TRIGGER_MODE: u8 = 0x12;
pub(crate) const SCOPE_TRIGGER_LEVEL: u8 = 0x14;

// TODO how to send this to device?
#[allow(dead_code)]
pub(crate) const SCOPE_AUTO_SETTING: u8 = 0x13;

pub(crate) const SCOPE_START_RECV: u8 = 0x16;

pub(crate) const SCOPE_VAL_COUPLING_AC: u8 = 0x00;
pub(crate) const SCOPE_VAL_COUPLING_DC: u8 = 0x01;
pub(crate) const SCOPE_VAL_COUPLING_GND: u8 = 0x02;

pub(crate) const SCOPE_VAL_PROBE_X1: u8 = 0x00;
pub(crate) const SCOPE_VAL_PROBE_X10: u8 = 0x01;
pub(crate) const SCOPE_VAL_PROBE_X100: u8 = 0x02;
pub(crate) const SCOPE_VAL_PROBE_X1000: u8 = 0x03;

pub(crate) const SCOPE_VAL_SCALE_10mV: u8 = 0x00;
pub(crate) const SCOPE_VAL_SCALE_20mV: u8 = 0x01;
pub(crate) const SCOPE_VAL_SCALE_50mV: u8 = 0x02;
pub(crate) const SCOPE_VAL_SCALE_100mV: u8 = 0x03;
pub(crate) const SCOPE_VAL_SCALE_200mV: u8 = 0x04;
pub(crate) const SCOPE_VAL_SCALE_500mV: u8 = 0x05;
pub(crate) const SCOPE_VAL_SCALE_1V: u8 = 0x06;
pub(crate) const SCOPE_VAL_SCALE_2V: u8 = 0x07;
pub(crate) const SCOPE_VAL_SCALE_5V: u8 = 0x08;
pub(crate) const SCOPE_VAL_SCALE_10V: u8 = 0x09;

pub(crate) const SCOPE_VAL_SCALE_TIME_5ns: u8 = 0x00;
pub(crate) const SCOPE_VAL_SCALE_TIME_10ns: u8 = 0x01;
pub(crate) const SCOPE_VAL_SCALE_TIME_20ns: u8 = 0x02;
pub(crate) const SCOPE_VAL_SCALE_TIME_50ns: u8 = 0x03;
pub(crate) const SCOPE_VAL_SCALE_TIME_100ns: u8 = 0x04;
pub(crate) const SCOPE_VAL_SCALE_TIME_200ns: u8 = 0x05;
pub(crate) const SCOPE_VAL_SCALE_TIME_500ns: u8 = 0x06;
pub(crate) const SCOPE_VAL_SCALE_TIME_1us: u8 = 0x07;
pub(crate) const SCOPE_VAL_SCALE_TIME_2us: u8 = 0x08;
pub(crate) const SCOPE_VAL_SCALE_TIME_5us: u8 = 0x09;
pub(crate) const SCOPE_VAL_SCALE_TIME_10us: u8 = 0x0a;
pub(crate) const SCOPE_VAL_SCALE_TIME_20us: u8 = 0x0b;
pub(crate) const SCOPE_VAL_SCALE_TIME_50us: u8 = 0x0c;
pub(crate) const SCOPE_VAL_SCALE_TIME_100us: u8 = 0x0d;
pub(crate) const SCOPE_VAL_SCALE_TIME_200us: u8 = 0x0e;
pub(crate) const SCOPE_VAL_SCALE_TIME_500us: u8 = 0x0f;
pub(crate) const SCOPE_VAL_SCALE_TIME_1ms: u8 = 0x10;
pub(crate) const SCOPE_VAL_SCALE_TIME_2ms: u8 = 0x11;
pub(crate) const SCOPE_VAL_SCALE_TIME_5ms: u8 = 0x12;
pub(crate) const SCOPE_VAL_SCALE_TIME_10ms: u8 = 0x13;
pub(crate) const SCOPE_VAL_SCALE_TIME_20ms: u8 = 0x14;
pub(crate) const SCOPE_VAL_SCALE_TIME_50ms: u8 = 0x15;
pub(crate) const SCOPE_VAL_SCALE_TIME_100ms: u8 = 0x16;
pub(crate) const SCOPE_VAL_SCALE_TIME_200ms: u8 = 0x17;
pub(crate) const SCOPE_VAL_SCALE_TIME_500ms: u8 = 0x18;
pub(crate) const SCOPE_VAL_SCALE_TIME_1s: u8 = 0x19;
pub(crate) const SCOPE_VAL_SCALE_TIME_2s: u8 = 0x1a;
pub(crate) const SCOPE_VAL_SCALE_TIME_5s: u8 = 0x1b;
pub(crate) const SCOPE_VAL_SCALE_TIME_10s: u8 = 0x1c;
pub(crate) const SCOPE_VAL_SCALE_TIME_20s: u8 = 0x1d;
pub(crate) const SCOPE_VAL_SCALE_TIME_50s: u8 = 0x1e;
pub(crate) const SCOPE_VAL_SCALE_TIME_100s: u8 = 0x1f;
pub(crate) const SCOPE_VAL_SCALE_TIME_200s: u8 = 0x20;
pub(crate) const SCOPE_VAL_SCALE_TIME_500s: u8 = 0x21;

pub(crate) const SCOPE_VAL_TRIGGER_SLOPE_RISING: u8 = 0x00;
pub(crate) const SCOPE_VAL_TRIGGER_SLOPE_FALLING: u8 = 0x01;
pub(crate) const SCOPE_VAL_TRIGGER_SLOPE_BOTH: u8 = 0x02;

pub(crate) const SCOPE_VAL_TRIGGER_MODE_AUTO: u8 = 0x00;
pub(crate) const SCOPE_VAL_TRIGGER_MODE_NORMAL: u8 = 0x01;
pub(crate) const SCOPE_VAL_TRIGGER_MODE_SINGLE: u8 = 0x02;

pub(crate) const AWG_TYPE: u8 = 0x00;
pub(crate) const AWG_FREQ: u8 = 0x01;
pub(crate) const AWG_AMPLITUDE: u8 = 0x02;
pub(crate) const AWG_OFFSET: u8 = 0x03;
pub(crate) const AWG_SQUARE_DUTY: u8 = 0x04;
pub(crate) const AWG_RAMP_DUTY: u8 = 0x05;
pub(crate) const AWG_TRAP_DUTY: u8 = 0x06;
pub(crate) const AWG_START_STOP: u8 = 0x08;

pub(crate) const AWG_VAL_TYPE_SQUARE: u8 = 0x00;
pub(crate) const AWG_VAL_TYPE_RAMP: u8 = 0x01;
pub(crate) const AWG_VAL_TYPE_SIN: u8 = 0x02;
pub(crate) const AWG_VAL_TYPE_TRAP: u8 = 0x03;
pub(crate) const AWG_VAL_TYPE_ARB1: u8 = 0x04;
pub(crate) const AWG_VAL_TYPE_ARB2: u8 = 0x05;
pub(crate) const AWG_VAL_TYPE_ARB3: u8 = 0x06;
pub(crate) const AWG_VAL_TYPE_ARB4: u8 = 0x07;

pub(crate) const SCREEN_VAL_SCOPE: u8 = 0x00;
pub(crate) const SCREEN_VAL_DMM: u8 = 0x01;
pub(crate) const SCREEN_VAL_AWG: u8 = 0x02;
