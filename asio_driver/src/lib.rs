use bitflags::bitflags;
use windows::core::IntoParam;
pub type GUID = windows::core::GUID;

pub type AsioSamples = i64;
pub type AsioTimestamp = i64;
pub type AsioSampleRate = f64;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AsioName {
    pub inner: [std::ffi::c_char; 32],
}

impl AsioName {
    pub fn new() -> Self {
        AsioName {
            inner: [0 as std::ffi::c_char; 32],
        }
    }
    pub fn as_ptr(&self) -> *const std::ffi::c_char {
        self.inner.as_ptr()
    }
    pub fn as_mut_ptr(&mut self) -> *mut std::ffi::c_char {
        self.inner.as_mut_ptr()
    }
    pub unsafe fn to_str(&self) -> Result<&str, std::str::Utf8Error> {
        return std::ffi::CStr::from_ptr(self.inner.as_ptr()).to_str();
    }
}

impl From<[std::ffi::c_char; 32]> for AsioName {
    fn from(value: [std::ffi::c_char; 32]) -> Self {
        return AsioName { inner: value };
    }
}

impl Into<[std::ffi::c_char; 32]> for AsioName {
    fn into(self) -> [std::ffi::c_char; 32] {
        return self.inner;
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AsioErrorMsg {
    pub inner: [std::ffi::c_char; 124],
}

impl AsioErrorMsg {
    pub fn new() -> Self {
        AsioErrorMsg {
            inner: [0 as std::ffi::c_char; 124],
        }
    }
    pub fn as_ptr(&self) -> *const std::ffi::c_char {
        self.inner.as_ptr()
    }
    pub fn as_mut_ptr(&mut self) -> *mut std::ffi::c_char {
        self.inner.as_mut_ptr()
    }
    pub unsafe fn to_str(&self) -> Result<&str, std::str::Utf8Error> {
        return std::ffi::CStr::from_ptr(self.inner.as_ptr()).to_str();
    }
}

impl From<[std::ffi::c_char; 124]> for AsioErrorMsg {
    fn from(value: [std::ffi::c_char; 124]) -> Self {
        return AsioErrorMsg { inner: value };
    }
}

impl Into<[std::ffi::c_char; 124]> for AsioErrorMsg {
    fn into(self) -> [std::ffi::c_char; 124] {
        return self.inner;
    }
}

pub struct AsioClockSources {
    pub length: i32,
    pub array: [AsioClockSource; 16],
}

impl AsioClockSources {
    pub fn new() -> AsioClockSources {
        AsioClockSources {
            length: 0,
            array: [AsioClockSource::new(); 16],
        }
    }
    pub fn as_slice(&self) -> &[AsioClockSource] {
        &self.array[0..self.length as usize]
    }
    pub fn as_mut_slice(&mut self) -> &mut [AsioClockSource] {
        &mut self.array[0..self.length as usize]
    }
    pub fn iter(&self) -> std::slice::Iter<'_, AsioClockSource> {
        self.as_slice().iter()
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AsioBool {
    False = 0,
    True = 1,
}

impl AsioBool {
    pub fn to_bool(&self) -> bool {
        *self == AsioBool::True
    }
}

impl Into<bool> for AsioBool {
    fn into(self) -> bool {
        self == AsioBool::True
    }
}

impl From<bool> for AsioBool {
    fn from(value: bool) -> Self {
        if value {
            AsioBool::True
        } else {
            AsioBool::False
        }
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AsioSampleType {
    AsioSTInt16MSB = 0,
    AsioSTInt24MSB = 1, // used for 20 bits as well
    AsioSTInt32MSB = 2,
    AsioSTFloat32MSB = 3, // IEEE 754 32 bit float
    AsioSTFloat64MSB = 4, // IEEE 754 64 bit double float

    // these are used for 32 bit data buffer, with different alignment of the data inside
    // 32 bit PCI bus systems can be more easily used with these
    AsioSTInt32MSB16 = 8,  // 32 bit data with 16 bit alignment
    AsioSTInt32MSB18 = 9,  // 32 bit data with 18 bit alignment
    AsioSTInt32MSB20 = 10, // 32 bit data with 20 bit alignment
    AsioSTInt32MSB24 = 11, // 32 bit data with 24 bit alignment

    AsioSTInt16LSB = 16,
    AsioSTInt24LSB = 17, // used for 20 bits as well
    AsioSTInt32LSB = 18,
    AsioSTFloat32LSB = 19, // IEEE 754 32 bit float, as found on Intel x86 architecture
    AsioSTFloat64LSB = 20, // IEEE 754 64 bit double float, as found on Intel x86 architecture

    // these are used for 32 bit data buffer, with different alignment of the data inside
    // 32 bit PCI bus systems can more easily used with these
    AsioSTInt32LSB16 = 24, // 32 bit data with 18 bit alignment
    AsioSTInt32LSB18 = 25, // 32 bit data with 18 bit alignment
    AsioSTInt32LSB20 = 26, // 32 bit data with 20 bit alignment
    AsioSTInt32LSB24 = 27, // 32 bit data with 24 bit alignment

    //	Asio DSD format.
    AsioSTDSDInt8LSB1 = 32, // DSD 1 bit data, 8 samples per byte. First sample in Least significant bit.
    AsioSTDSDInt8MSB1 = 33, // DSD 1 bit data, 8 samples per byte. First sample in Most significant bit.
    AsioSTDSDInt8NER8 = 40, // DSD 8 bit data, 1 sample per byte. No Endianness required.
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AsioError {
    Ok = 0,
    Success = 0x3f4847a0,
    NotPresent = -1000,
    HwMalfunction,
    InvalidParameter,
    InvalidMode,
    SpNotAdvancing,
    NoClock,
    NoMemory,
}

#[repr(C)]
pub struct AsioTimeCode {
    pub speed: f64,
    pub time_code_samples: AsioSamples,
    pub flags: AsioTimeCodeFlags,
    future: [u8; 64],
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct AsioTimeCodeFlags :u32 {
        const valid      = 1 << 0;
        const running    = 1 << 1;
        const reverse    = 1 << 2;
        const onspeed    = 1 << 3;
        const still      = 1 << 4;
        const speedValid = 1 << 8;
    }
}

#[repr(C)]
pub struct AsioTimeInfo {
    pub speed: f64,
    pub system_time: AsioTimestamp,
    pub sample_position: AsioSamples,
    pub sample_rate: AsioSampleRate,
    pub flags: u32,
    reserved: [u8; 12],
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub  struct AsioTimeInfoFlags :u32 {
        const systemTimeValid     = 1 << 0; // must always be valid
        const samplePositionValid = 1 << 1; // must always be valid
        const sampleRateValid     = 1 << 2;
        const speedValid          = 1 << 3;
        const sampleRateChanged   = 1 << 4;
        const clockSourceChanged  = 1 << 5;
    }
}

#[repr(C)]
pub struct AsioTime {
    reserved: [i32; 4],
    pub time_info: AsioTimeInfo,
    pub time_code: AsioTimeCode,
}

#[rustfmt::skip]
#[repr(C)]
pub struct AsioCallbacks {
    pub buffer_switch: unsafe extern "C" fn(doubleBufferIdx: i32, directProcess: AsioBool),
    pub sample_rate_did_change: unsafe extern "C" fn(sampleRate: AsioSampleRate),
    pub asio_message: unsafe extern "C" fn(selector: AsioMessageSelector, value: i32, message: *mut std::ffi::c_void, opt: *mut f64) -> i32,
    pub buffer_switch_time_info: unsafe extern "C" fn(params: *mut AsioTime, doubleBufferIndex: i32, directProcess: AsioBool) -> *mut AsioTime,
}

#[repr(i32)]
#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AsioMessageSelector {
    SelectorSupported = 1,    // selector in <value>, returns 1L if supported,
                              // 0 otherwise
    EngineVersion,            // returns engine (host) asio implementation version,
                              // 2 or higher
    ResetRequest,             // request driver reset. if accepted, this
                              // will close the driver (ASIO_Exit() ) and
                              // re-open it again (ASIO_Init() etc). some
                              // drivers need to reconfigure for instance
                              // when the sample rate changes, or some basic
                              // changes have been made in ASIO_ControlPanel().
                              // returns 1L; note the request is merely passed
                              // to the application, there is no way to determine
                              // if it gets accepted at this time (but it usually
                              // will be).
    BufferSizeChange,         // not yet supported, will currently always return 0L.
                              // for now, use kAsioResetRequest instead.
                              // once implemented, the new buffer size is expected
                              // in <value>, and on success returns 1L
    ResyncRequest,            // the driver went out of sync, such that
                              // the timestamp is no longer valid. this
                              // is a request to re-start the engine and
                              // slave devices (sequencer). returns 1 for ok,
                              // 0 if not supported.
    LatenciesatenciesChanged, // the drivers latencies have changed. The engine
                              // will refetch the latencies.
    SupportsTimeInfo,         // if host returns true here, it will expect the
                              // callback bufferSwitchTimeInfo to be called instead
                              // of bufferSwitch
    SupportsTimeCode,         //
    MMCCommand,               // unused - value: number of commands, message points to mmc commands
    SupportsInputMonitor,     // kAsioSupportsXXX return 1 if host supports this
    SupportsInputGain,        // unused and undefined
    SupportsInputMeter,       // unused and undefined
    SupportsOutputGain,       // unused and undefined
    SupportsOutputMeter,      // unused and undefined
    Overload,                 // driver detected an overload
}

#[repr(i32)]
#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AsioFutureSelector {
    EnableTimeCodeRead = 1, // no arguments
    DisableTimeCodeRead,    // no arguments
    SetInputMonitor,        // ASIOInputMonitor* in params
    Transport,              // ASIOTransportParameters* in params
    SetInputGain,           // ASIOChannelControls* in params, apply gain
    GetInputMeter,          // ASIOChannelControls* in params, fill meter
    SetOutputGain,          // ASIOChannelControls* in params, apply gain
    GetOutputMeter,         // ASIOChannelControls* in params, fill meter
    CanInputMonitor,        // no arguments for CanXXX selectors
    CanTimeInfo,
    CanTimeCode,
    CanTransport,
    CanInputGain,
    CanInputMeter,
    CanOutputGain,
    CanOutputMeter,
    OptionalOne,
    
    //    DSD support
    //    The following extensions are required to allow switching
    //    and control of the DSD subsystem.
    SetIoFormat   = 0x23111961, // ASIOIoFormat * in params
    GetIoFormat   = 0x23111983, // ASIOIoFormat * in params
    CanDoIoFormat = 0x23112004, // ASIOIoFormat * in params
    
    // Extension for drop out detection
    CanReportOverload        = 0x24042012, // return ASE_SUCCESS if driver can
                                           // detect and report overloads
    GetInternalBufferSamples = 0x25042012, // ASIOInternalBufferInfo * in params.
                                           // Deliver size of driver internal buffering,
                                           // return ASE_SUCCESS if supported
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct AsioClockSource {
    pub index: i32,
    pub associated_channel: i32,
    pub associated_group: i32,
    pub is_current_source: AsioBool,
    pub name: AsioName,
}

impl AsioClockSource {
    pub fn new() -> Self {
        AsioClockSource {
            index: 0,
            associated_channel: 0,
            associated_group: 0,
            is_current_source: AsioBool::False,
            name: AsioName::new(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct AsioChannelInfo {
    pub channel: i32,
    pub is_input: AsioBool,
    pub is_active: AsioBool,
    pub channel_group: i32,
    pub sample_type: AsioSampleType,
    pub name: AsioName,
}

impl AsioChannelInfo {
    pub fn new(channel: i32, is_input: bool) -> AsioChannelInfo {
        AsioChannelInfo {
            channel,
            is_input: AsioBool::from(is_input),
            is_active: AsioBool::False,
            channel_group: 0,
            sample_type: AsioSampleType::AsioSTInt16MSB,
            name: AsioName::new(),
        }
    }
    pub fn new_input(channel: i32) -> AsioChannelInfo {
        AsioChannelInfo::new(channel, true)
    }
    pub fn new_output(channel: i32) -> AsioChannelInfo {
        AsioChannelInfo::new(channel, false)
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct AsioBufferInfo {
    pub is_input: AsioBool,
    pub channel_num: i32,
    pub buffers: [*mut std::ffi::c_void; 2],
}

impl AsioBufferInfo {
    pub fn new(channel: i32, is_input: bool) -> AsioBufferInfo {
        AsioBufferInfo {
            is_input: AsioBool::from(is_input),
            channel_num: channel,
            buffers: [std::ptr::null_mut(); 2],
        }
    }
    pub fn new_input(channel: i32) -> AsioBufferInfo {
        AsioBufferInfo::new(channel, true)
    }
    pub fn new_output(channel: i32) -> AsioBufferInfo {
        AsioBufferInfo::new(channel, false)
    }
}

#[repr(C)]
pub struct AsioInputMonitor {
    pub input: i32,
    pub output: i32,
    pub gain: i32,
    pub state: AsioBool,
    pub pan: i32,
}

#[repr(C)]
pub struct AsioChannelControls {
    pub channel: i32,
    pub is_input: AsioBool,
    pub gain: i32,
    pub meter: i32,
    future: [u8; 32],
}

#[repr(C)]
pub struct AsioTransportParameters {
    pub command: AsioTransportCommand,
    pub sample_position: AsioSamples,
    pub track: i32,
    pub track_switches: [i32; 16],
    future: [u8; 64],
}

#[repr(i32)]
#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AsioTransportCommand {
    Start = 1,
    Stop,
    Locate,     // to samplePosition
    PunchIn,
    PunchOut,
    ArmOn,      // track
    ArmOff,     // track
    MonitorOn,  // track
    MonitorOff, // track
    Arm,        // trackSwitches
    Monitor     // trackSwitches
}

#[repr(i32)]
#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AsioIoFormatType
{
    FormatInvalid = -1,
    PCM = 0,
    DSD = 1,
}

#[repr(C)]
pub struct AsioIoFormat {
    pub format_type: AsioIoFormatType,
    future: [u8; 480],
}

#[repr(C)]
pub struct AsioInternalBufferInfo {
    input_samples: i32,
    output_samples: i32,
}

#[repr(transparent)]
pub struct AsioDriver(windows::core::IUnknown);
impl AsioDriver {
    pub unsafe fn new(driver_guid: windows::core::GUID) -> Result<AsioDriver, String> {
        let win_result = windows::Win32::System::Com::CoInitialize(None);
        if let Some(err) = win_result.err() {
            return Err(err.to_string());
        }
        let win_result = co_create_instance_non_static_iid(
            &driver_guid,
            None,
            windows::Win32::System::Com::CLSCTX_INPROC_SERVER,
            &driver_guid,
        );
        return match win_result {
            Err(err) => Err(err.to_string()),
            Ok(driver) => Ok(driver),
        };
    }
    #[rustfmt::skip]
    pub unsafe fn init(&self, sys_handle: *mut std::ffi::c_void) -> AsioBool {
        (windows::core::Interface::vtable(self).init)
        (windows::core::Interface::as_raw(self),sys_handle.into_param().abi())
    }
    #[rustfmt::skip]
    pub unsafe fn get_driver_name_raw(&self, name: *mut std::ffi::c_char) {
        (windows::core::Interface::vtable(self).get_driver_name)
        (windows::core::Interface::as_raw(self),name.into_param().abi())
    }
    #[rustfmt::skip]
    pub unsafe fn get_driver_name(&self, name: &mut AsioName) {
        (windows::core::Interface::vtable(self).get_driver_name)
        (windows::core::Interface::as_raw(self),name.as_mut_ptr().into_param().abi())
    }
    #[rustfmt::skip]
    pub unsafe fn get_driver_version(&self) -> i32 {
        (windows::core::Interface::vtable(self).get_driver_version)
        (windows::core::Interface::as_raw(self))
    }
    #[rustfmt::skip]
    pub unsafe fn get_error_message_raw(&self, name: *mut std::ffi::c_char) {
        (windows::core::Interface::vtable(self).get_error_message)
        (windows::core::Interface::as_raw(self),name.into_param().abi())
    }
    #[rustfmt::skip]
    pub unsafe fn get_error_message(&self, name: &mut AsioErrorMsg) {
        (windows::core::Interface::vtable(self).get_error_message)
        (windows::core::Interface::as_raw(self),name.as_mut_ptr().into_param().abi())
    }
    #[rustfmt::skip]
    pub unsafe fn start(&self) -> AsioError {
        (windows::core::Interface::vtable(self).start)
        (windows::core::Interface::as_raw(self))
    }
    #[rustfmt::skip]
    pub unsafe fn stop(&self) -> AsioError {
        (windows::core::Interface::vtable(self).stop)
        (windows::core::Interface::as_raw(self))
    }
    #[rustfmt::skip]
    pub unsafe fn get_channels(&self, num_input_channels: *mut i32, num_output_channels: *mut i32) -> AsioError {
        (windows::core::Interface::vtable(self).get_channels)
        (windows::core::Interface::as_raw(self), num_input_channels.into_param().abi(), num_output_channels.into_param().abi())
    }
    #[rustfmt::skip]
    pub unsafe fn get_latencies(&self, input_latency: *mut i32, output_latency: *mut i32) -> AsioError {
        (windows::core::Interface::vtable(self).get_latencies)
        (windows::core::Interface::as_raw(self), input_latency.into_param().abi(), output_latency.into_param().abi())
    }
    #[rustfmt::skip]
    pub unsafe fn get_buffer_size(&self, min_size: *mut i32, max_size: *mut i32, preferred_size: *mut i32, granularity: *mut i32) -> AsioError {
        (windows::core::Interface::vtable(self).get_buffer_size)
        (windows::core::Interface::as_raw(self), min_size.into_param().abi(), max_size.into_param().abi(), preferred_size.into_param().abi(), granularity.into_param().abi())
    }
    #[rustfmt::skip]
    pub unsafe fn can_sample_rate(&self, sample_rate: AsioSampleRate) -> AsioError {
        (windows::core::Interface::vtable(self).can_sample_rate)
        (windows::core::Interface::as_raw(self),sample_rate.into_param().abi())
    }
    #[rustfmt::skip]
    pub unsafe fn get_sample_rate(&self, sample_rate: *mut AsioSampleRate) -> AsioError {
        (windows::core::Interface::vtable(self).get_sample_rate)
        (windows::core::Interface::as_raw(self),sample_rate.into_param().abi())
    }
    #[rustfmt::skip]
    pub unsafe fn set_sample_rate(&self, sample_rate: AsioSampleRate) -> AsioError {
        (windows::core::Interface::vtable(self).set_sample_rate)
        (windows::core::Interface::as_raw(self),sample_rate.into_param().abi())
    }
    #[rustfmt::skip]
    pub unsafe fn get_clock_sources_raw(&self, clocks: *mut AsioClockSource, num_clocks: *mut i32) -> AsioError {
        (windows::core::Interface::vtable(self).get_clock_sources)
        (windows::core::Interface::as_raw(self),clocks.into_param().abi(), num_clocks.into_param().abi())
    }
    #[rustfmt::skip]
    pub unsafe fn get_clock_sources(&self, clock_sources: &mut AsioClockSources) -> AsioError {
        (windows::core::Interface::vtable(self).get_clock_sources)
        (windows::core::Interface::as_raw(self),clock_sources.array.as_mut_ptr().into_param().abi(), &mut clock_sources.length.into_param().abi())
    }
    #[rustfmt::skip]
    pub unsafe fn set_clock_source(&self, reference: i32) -> AsioError {
        (windows::core::Interface::vtable(self).set_clock_source)
        (windows::core::Interface::as_raw(self),reference.into_param().abi())
    }
    #[rustfmt::skip]
    pub unsafe fn get_sample_position(&self, samples: *mut AsioSamples, timestamp: *mut AsioTimestamp) -> AsioError {
        (windows::core::Interface::vtable(self).get_sample_position)
        (windows::core::Interface::as_raw(self),samples.into_param().abi(), timestamp.into_param().abi())
    }
    #[rustfmt::skip]
    pub unsafe fn get_channel_info(&self, channel_info: *mut AsioChannelInfo) -> AsioError {
        (windows::core::Interface::vtable(self).get_channel_info)
        (windows::core::Interface::as_raw(self),channel_info.into_param().abi())
    }
    #[rustfmt::skip]
    pub unsafe fn create_buffers(&self, buffer_infos: *mut AsioBufferInfo, num_channels: i32, buffer_size: i32, callbacks: *mut AsioCallbacks) -> AsioError {
        (windows::core::Interface::vtable(self).create_buffers)
        (windows::core::Interface::as_raw(self),buffer_infos.into_param().abi(),num_channels.into_param().abi(),buffer_size.into_param().abi(),callbacks.into_param().abi())
    }
    #[rustfmt::skip]
    pub unsafe fn dispose_buffers(&self) -> AsioError {
        (windows::core::Interface::vtable(self).dispose_buffers)
        (windows::core::Interface::as_raw(self))
    }
    #[rustfmt::skip]
    pub unsafe fn control_panel(&self) -> AsioError {
        (windows::core::Interface::vtable(self).control_panel)
        (windows::core::Interface::as_raw(self))
    }
    #[rustfmt::skip]
    pub unsafe fn future(&self, selector: AsioFutureSelector, opt: *mut std::ffi::c_void) -> AsioError {
        (windows::core::Interface::vtable(self).future)
        (windows::core::Interface::as_raw(self),(selector as i32).into_param().abi(),opt.into_param().abi())
    }
    #[rustfmt::skip]
    pub unsafe fn enable_time_code_read(&self) -> AsioError {
        let null_ptr: *mut std::ffi::c_void = std::ptr::null_mut();
        (windows::core::Interface::vtable(self).future)
        (windows::core::Interface::as_raw(self),(AsioFutureSelector::EnableTimeCodeRead as i32).into_param().abi(), null_ptr.into_param().abi())
    }
    #[rustfmt::skip]
    pub unsafe fn disable_time_code_read(&self) -> AsioError {
        let ptr: *mut std::ffi::c_void = std::ptr::null_mut();
        (windows::core::Interface::vtable(self).future)
        (windows::core::Interface::as_raw(self),(AsioFutureSelector::DisableTimeCodeRead as i32).into_param().abi(), ptr.into_param().abi())
    }
    #[rustfmt::skip]
    pub unsafe fn set_input_monitor(&self, input_monitor: &mut AsioInputMonitor) -> AsioError
    {
        let ptr = std::mem::transmute::<*mut AsioInputMonitor, *mut std::ffi::c_void>(input_monitor);
        (windows::core::Interface::vtable(self).future)
        (windows::core::Interface::as_raw(self),(AsioFutureSelector::SetInputMonitor as i32).into_param().abi(), ptr.into_param().abi())
    }
    #[rustfmt::skip]
    pub unsafe fn transport(&self, transport_params: &mut AsioTransportParameters) -> AsioError
    {
        let ptr = std::mem::transmute::<*mut AsioTransportParameters, *mut std::ffi::c_void>(transport_params);
        (windows::core::Interface::vtable(self).future)
        (windows::core::Interface::as_raw(self),(AsioFutureSelector::Transport as i32).into_param().abi(), ptr.into_param().abi())
    }
    #[rustfmt::skip]
    pub unsafe fn set_input_gain(&self, channel_ctrls: &mut AsioChannelControls) -> AsioError
    {
        let ptr = std::mem::transmute::<*mut AsioChannelControls, *mut std::ffi::c_void>(channel_ctrls);
        (windows::core::Interface::vtable(self).future)
        (windows::core::Interface::as_raw(self),(AsioFutureSelector::SetInputGain as i32).into_param().abi(), ptr.into_param().abi())
    }
    #[rustfmt::skip]
    pub unsafe fn get_input_meter(&self, channel_ctrls: &mut AsioChannelControls) -> AsioError
    {
        let ptr = std::mem::transmute::<*mut AsioChannelControls, *mut std::ffi::c_void>(channel_ctrls);
        (windows::core::Interface::vtable(self).future)
        (windows::core::Interface::as_raw(self),(AsioFutureSelector::GetInputMeter as i32).into_param().abi(), ptr.into_param().abi())
    }
    #[rustfmt::skip]
    pub unsafe fn set_output_gain(&self, channel_ctrls: &mut AsioChannelControls) -> AsioError
    {
        let ptr = std::mem::transmute::<*mut AsioChannelControls, *mut std::ffi::c_void>(channel_ctrls);
        (windows::core::Interface::vtable(self).future)
        (windows::core::Interface::as_raw(self),(AsioFutureSelector::SetOutputGain as i32).into_param().abi(), ptr.into_param().abi())
    }
    #[rustfmt::skip]
    pub unsafe fn get_output_meter(&self, channel_ctrls: &mut AsioChannelControls) -> AsioError
    {
        let ptr = std::mem::transmute::<*mut AsioChannelControls, *mut std::ffi::c_void>(channel_ctrls);
        (windows::core::Interface::vtable(self).future)
        (windows::core::Interface::as_raw(self),(AsioFutureSelector::GetOutputMeter as i32).into_param().abi(), ptr.into_param().abi())
    }
    #[rustfmt::skip]
    pub unsafe fn can_input_monitor(&self) -> AsioError
    {
        let ptr: *mut std::ffi::c_void = std::ptr::null_mut();
        (windows::core::Interface::vtable(self).future)
        (windows::core::Interface::as_raw(self),(AsioFutureSelector::CanInputMonitor as i32).into_param().abi(), ptr.into_param().abi())
    }
    #[rustfmt::skip]
    pub unsafe fn can_time_info(&self) -> AsioError
    {
        let ptr: *mut std::ffi::c_void = std::ptr::null_mut();
        (windows::core::Interface::vtable(self).future)
        (windows::core::Interface::as_raw(self),(AsioFutureSelector::CanTimeInfo as i32).into_param().abi(), ptr.into_param().abi())
    }
    #[rustfmt::skip]
    pub unsafe fn can_time_code(&self) -> AsioError
    {
        let ptr: *mut std::ffi::c_void = std::ptr::null_mut();
        (windows::core::Interface::vtable(self).future)
        (windows::core::Interface::as_raw(self),(AsioFutureSelector::CanTimeCode as i32).into_param().abi(), ptr.into_param().abi())
    }
    #[rustfmt::skip]
    pub unsafe fn can_transport(&self) -> AsioError
    {
        let ptr: *mut std::ffi::c_void = std::ptr::null_mut();
        (windows::core::Interface::vtable(self).future)
        (windows::core::Interface::as_raw(self),(AsioFutureSelector::CanTransport as i32).into_param().abi(), ptr.into_param().abi())
    }
    #[rustfmt::skip]
    pub unsafe fn can_input_gain(&self) -> AsioError
    {
        let ptr: *mut std::ffi::c_void = std::ptr::null_mut();
        (windows::core::Interface::vtable(self).future)
        (windows::core::Interface::as_raw(self),(AsioFutureSelector::CanInputGain as i32).into_param().abi(), ptr.into_param().abi())
    }
    #[rustfmt::skip]
    pub unsafe fn can_input_meter(&self) -> AsioError
    {
        let ptr: *mut std::ffi::c_void = std::ptr::null_mut();
        (windows::core::Interface::vtable(self).future)
        (windows::core::Interface::as_raw(self),(AsioFutureSelector::CanInputMeter as i32).into_param().abi(), ptr.into_param().abi())
    }
    #[rustfmt::skip]
    pub unsafe fn can_output_gain(&self) -> AsioError
    {
        let ptr: *mut std::ffi::c_void = std::ptr::null_mut();
        (windows::core::Interface::vtable(self).future)
        (windows::core::Interface::as_raw(self),(AsioFutureSelector::CanOutputGain as i32).into_param().abi(), ptr.into_param().abi())
    }
    #[rustfmt::skip]
    pub unsafe fn can_output_meter(&self) -> AsioError
    {
        let ptr: *mut std::ffi::c_void = std::ptr::null_mut();
        (windows::core::Interface::vtable(self).future)
        (windows::core::Interface::as_raw(self),(AsioFutureSelector::CanOutputMeter as i32).into_param().abi(), ptr.into_param().abi())
    }
    #[rustfmt::skip]
    pub unsafe fn optional_one(&self) -> AsioError // What does this do?
    {
        let ptr: *mut std::ffi::c_void = std::ptr::null_mut();
        (windows::core::Interface::vtable(self).future)
        (windows::core::Interface::as_raw(self),(AsioFutureSelector::OptionalOne as i32).into_param().abi(), ptr.into_param().abi())
    }
    #[rustfmt::skip]
    pub unsafe fn set_io_format(&self, io_format: &mut AsioIoFormat) -> AsioError
    {
        let ptr = std::mem::transmute::<*mut AsioIoFormat, *mut std::ffi::c_void>(io_format);
        (windows::core::Interface::vtable(self).future)
        (windows::core::Interface::as_raw(self),(AsioFutureSelector::SetIoFormat as i32).into_param().abi(), ptr.into_param().abi())
    }
    #[rustfmt::skip]
    pub unsafe fn get_io_format(&self, io_format: &mut AsioIoFormat) -> AsioError
    {
        let ptr = std::mem::transmute::<*mut AsioIoFormat, *mut std::ffi::c_void>(io_format);
        (windows::core::Interface::vtable(self).future)
        (windows::core::Interface::as_raw(self),(AsioFutureSelector::GetIoFormat as i32).into_param().abi(), ptr.into_param().abi())
    }
    #[rustfmt::skip]
    pub unsafe fn can_do_io_format(&self, io_format: &mut AsioIoFormat) -> AsioError
    {
        let ptr = std::mem::transmute::<*mut AsioIoFormat, *mut std::ffi::c_void>(io_format);
        (windows::core::Interface::vtable(self).future)
        (windows::core::Interface::as_raw(self),(AsioFutureSelector::CanDoIoFormat as i32).into_param().abi(), ptr.into_param().abi())
    }
    #[rustfmt::skip]
    pub unsafe fn can_report_overload(&self) -> AsioError
    {
        let ptr: *mut std::ffi::c_void = std::ptr::null_mut();
        (windows::core::Interface::vtable(self).future)
        (windows::core::Interface::as_raw(self),(AsioFutureSelector::CanReportOverload as i32).into_param().abi(), ptr.into_param().abi())
    }
    #[rustfmt::skip]
    pub unsafe fn get_internal_buffer_samples(&self, internal_buf_info : &mut AsioInternalBufferInfo) -> AsioError
    {
        let ptr = std::mem::transmute::<*mut AsioInternalBufferInfo, *mut std::ffi::c_void>(internal_buf_info);
        (windows::core::Interface::vtable(self).future)
        (windows::core::Interface::as_raw(self),(AsioFutureSelector::GetInternalBufferSamples as i32).into_param().abi(), ptr.into_param().abi())
    }
    #[rustfmt::skip]
    pub unsafe fn output_ready(&self) -> AsioError {
        (windows::core::Interface::vtable(self).output_ready)
        (windows::core::Interface::as_raw(self))
    }
}

impl Drop for AsioDriver {
    fn drop(&mut self) {
        unsafe { windows::Win32::System::Com::CoUninitialize() }
    }
}

#[rustfmt::skip]
#[repr(C)]
pub struct AsioDriverVtbl {
    pub base__: windows::core::IUnknown_Vtbl,
    // virtual ASIOBool init(void *sysHandle) = 0;
    pub init: unsafe extern "system" fn(this: *mut std::ffi::c_void,sysHandle: *mut std::ffi::c_void) -> AsioBool,
    // virtual void getDriverName(char *name) = 0;
    pub get_driver_name: unsafe extern "system" fn(this: *mut std::ffi::c_void, *mut std::ffi::c_char),
    // virtual long getDriverVersion() = 0;
    pub get_driver_version: unsafe extern "system" fn(this: *mut std::ffi::c_void) -> i32,
    // virtual void getErrorMessage(char *string) = 0;	
    pub get_error_message: unsafe extern "system" fn(this: *mut std::ffi::c_void, *mut std::ffi::c_char),
    // virtual ASIOError start() = 0;
    pub start: unsafe extern "system" fn(this: *mut std::ffi::c_void) -> AsioError,
    // virtual ASIOError stop() = 0;
    pub stop: unsafe extern "system" fn(this: *mut std::ffi::c_void) -> AsioError,
    // virtual ASIOError getChannels(long *numInputChannels, long *numOutputChannels) = 0;
    pub get_channels: unsafe extern "system" fn(this: *mut std::ffi::c_void, *mut i32, *mut i32) -> AsioError,
    // virtual ASIOError getLatencies(long *inputLatency, long *outputLatency) = 0;
    pub get_latencies: unsafe extern "system" fn(this: *mut std::ffi::c_void, *mut i32, *mut i32) -> AsioError,
    // 	virtual ASIOError getBufferSize(long *minSize, long *maxSize, long *preferredSize, long *granularity) = 0;
    pub get_buffer_size: unsafe extern "system" fn(this: *mut std::ffi::c_void, *mut i32, *mut i32, *mut i32, *mut i32) -> AsioError,
    // virtual ASIOError canSampleRate(ASIOSampleRate sampleRate) = 0;
    pub can_sample_rate: unsafe extern "system" fn(this: *mut std::ffi::c_void, AsioSampleRate) -> AsioError,
    // virtual ASIOError getSampleRate(ASIOSampleRate *sampleRate) = 0;
    pub get_sample_rate: unsafe extern "system" fn(this: *mut std::ffi::c_void, *mut AsioSampleRate) -> AsioError,
    // virtual ASIOError setSampleRate(ASIOSampleRate sampleRate) = 0;
    pub set_sample_rate: unsafe extern "system" fn(this: *mut std::ffi::c_void, AsioSampleRate) -> AsioError,
    // virtual ASIOError getClockSources(ASIOClockSource *clocks, long *numSources) = 0;
    pub get_clock_sources: unsafe extern "system" fn(this: *mut std::ffi::c_void, *mut AsioClockSource, *mut i32) -> AsioError,
    // virtual ASIOError setClockSource(long reference) = 0;
    pub set_clock_source: unsafe extern "system" fn(this: *mut std::ffi::c_void, i32) -> AsioError,
    // virtual ASIOError getSamplePosition(ASIOSamples *sPos, ASIOTimeStamp *tStamp) = 0;
    pub get_sample_position: unsafe extern "system" fn(this: *mut std::ffi::c_void, *mut AsioSamples, *mut AsioTimestamp) -> AsioError,
    // virtual ASIOError getChannelInfo(ASIOChannelInfo *info) = 0;
    pub get_channel_info: unsafe extern "system" fn(this: *mut std::ffi::c_void, *mut AsioChannelInfo) -> AsioError,
    // virtual ASIOError createBuffers(ASIOBufferInfo *bufferInfos, long numChannels, long bufferSize, ASIOCallbacks *callbacks) = 0;
    pub create_buffers: unsafe extern "system" fn(this: *mut std::ffi::c_void, *mut AsioBufferInfo, i32, i32, *mut AsioCallbacks) -> AsioError,
    // virtual ASIOError disposeBuffers() = 0;
    pub dispose_buffers: unsafe extern "system" fn(this: *mut std::ffi::c_void) -> AsioError,
    // virtual ASIOError controlPanel() = 0;
    pub control_panel: unsafe extern "system" fn(this: *mut std::ffi::c_void) -> AsioError,
    // virtual ASIOError future(long selector,void *opt) = 0;
    pub future: unsafe extern "system" fn(this: *mut std::ffi::c_void, i32, *mut std::ffi::c_void) -> AsioError,
    // virtual ASIOError outputReady() = 0;
    pub output_ready: unsafe extern "system" fn(this: *mut std::ffi::c_void) -> AsioError,
}

unsafe impl windows::core::Interface for AsioDriver {
    type Vtable = AsioDriverVtbl;
}

impl std::clone::Clone for AsioDriver {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

pub unsafe trait NonStaticIidComInterface: windows::core::Interface + Clone {}
unsafe impl NonStaticIidComInterface for AsioDriver {}

pub unsafe fn co_create_instance_non_static_iid<P0, T>(
    rclsid: *const windows::core::GUID,
    punkouter: P0,
    dwclscontext: windows::Win32::System::Com::CLSCTX,
    riid: *const windows::core::GUID,
) -> windows::core::Result<T>
where
    P0: windows::core::IntoParam<windows::core::IUnknown>,
    T: NonStaticIidComInterface,
{
    windows_targets::link ! ( "ole32.dll""system" fn CoCreateInstance ( rclsid : *const windows::core::GUID , punkouter : * mut::core::ffi::c_void , dwclscontext : windows::Win32::System::Com::CLSCTX , riid : *const windows::core::GUID , ppv : *mut *mut ::core::ffi::c_void ) -> windows::core::HRESULT );
    let mut result__ = ::std::ptr::null_mut();
    CoCreateInstance(
        rclsid,
        punkouter.into_param().abi(),
        dwclscontext,
        riid,
        &mut result__,
    )
    .from_abi(result__)
}
