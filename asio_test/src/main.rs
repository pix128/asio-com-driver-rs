use asio_driver;

unsafe extern "C" fn buffer_switch(double_buffer_idx: i32, direct_process: asio_driver::AsioBool) {
    println!(
        "Buffer Switch, double_buffer_idx: {}, direct_process: {:?}",
        double_buffer_idx, direct_process
    )
}

unsafe extern "C" fn sample_rate_did_change(sample_rate: asio_driver::AsioSampleRate) {
    println!("Sample Rate Changed: {}", sample_rate)
}

#[allow(unused_variables)]
unsafe extern "C" fn asio_message(
    selector: asio_driver::AsioMessageSelector,
    value: i32,
    message: *mut std::ffi::c_void,
    opt: *mut f64,
) -> i32 {
    println!("Asio Message, Selector: {:?}, Value: {}", selector, value);
    return selector as i32;
}

#[allow(unused_variables)]
unsafe extern "C" fn buffer_switch_time_info(
    params: *mut asio_driver::AsioTime,
    double_buffer_index: i32,
    direct_process: asio_driver::AsioBool,
) -> *mut asio_driver::AsioTime {
    println!("Buffer Switch Time Info");
    return params;
}

// Driver GUIDs are found in \HKLM\SOFTWARE\ASIO\
#[allow(dead_code)]
const ASIO4ALL_V2_GUID: asio_driver::GUID =
    asio_driver::GUID::from_u128(0x232685C6_6548_49D8_846D_4141A3EF7560);
#[allow(dead_code)]
const REALTEK_ASIO_GUID: asio_driver::GUID =
    asio_driver::GUID::from_u128(0xA80362FF_CE76_4DD9_874A_704C57BF0D6A);
#[allow(dead_code)]
const FL_STUDIO_ASIO_GUID: asio_driver::GUID =
    asio_driver::GUID::from_u128(0x188135E1_7171_3434_854F_01A3C71F3DF9);

pub fn main() {
    unsafe {
        let asio_driver_guid = ASIO4ALL_V2_GUID;

        let asio_driver: asio_driver::AsioDriver =
            asio_driver::AsioDriver::new(asio_driver_guid).unwrap();

        // Init
        let init = asio_driver.init(std::ptr::null_mut());
        println!("Init: {:?}", init);

        // Show Control Panel
        let control_panel = asio_driver.control_panel();
        println!("Control Panel: {:?}", control_panel);

        // Get Driver Name
        let mut driver_name = asio_driver::AsioName::new();
        asio_driver.get_driver_name(&mut driver_name);
        println!("Driver Name: {}", driver_name.to_str().unwrap());

        // Get Driver Version
        let driver_version = asio_driver.get_driver_version();
        println!("Driver Version: {}", driver_version);

        // Can Sample Rate
        let sample_rate: asio_driver::AsioSampleRate = 44100.0;
        let can_sample_rate = asio_driver.can_sample_rate(sample_rate);
        println!("Can Sample Rate: {:?} ({})", can_sample_rate, sample_rate);

        // Set Sample Rate
        let set_sample_rate = asio_driver.set_sample_rate(sample_rate);
        println!("Set Sample Rate: {:?} ({})", set_sample_rate, sample_rate);

        // Get Sample Rate
        let mut sample_rate: asio_driver::AsioSampleRate = 0.0;
        let get_sample_rate = asio_driver.get_sample_rate(&mut sample_rate);
        println!("Get Sample Rate: {:?} ({})", get_sample_rate, sample_rate);

        // Num Channels
        let mut num_input_channels: i32 = 0;
        let mut num_output_channels: i32 = 0;
        let get_channels =
            asio_driver.get_channels(&mut num_input_channels, &mut num_output_channels);
        let num_channels = num_input_channels + num_output_channels;
        println!("Get Channels: {:?}", get_channels);
        println!(
            "Inputs: {}, Outputs: {}, Total: {}",
            num_input_channels, num_output_channels, num_channels
        );

        // Get Info For Input Channels
        for i in 0..num_input_channels {
            let mut input_ch = asio_driver::AsioChannelInfo::new_input(i);
            let get_input_channel = asio_driver.get_channel_info(&mut input_ch);
            println!("Get Input {} Info: {:?}", i, get_input_channel);
            println!("Name: {}", input_ch.name.to_str().unwrap());
        }

        // Get Info For Output Channels
        for i in 0..num_output_channels {
            let mut output_ch = asio_driver::AsioChannelInfo::new_output(i);
            let get_output_channel = asio_driver.get_channel_info(&mut output_ch);
            println!("Get Output {} Info: {:?}", i, get_output_channel);
            println!("Name: {}", output_ch.name.to_str().unwrap());
        }

        // Latencies
        let mut input_latency: i32 = 0;
        let mut output_latency: i32 = 0;
        let get_latencies = asio_driver.get_latencies(&mut input_latency, &mut output_latency);
        println!("Get Latencies: {:?}", get_latencies);
        println!("Input: {}, Output: {}", input_latency, output_latency);

        // Get Clock Sources
        let mut clock_sources = asio_driver::AsioClockSources::new();
        let get_clock_sources = asio_driver.get_clock_sources(&mut clock_sources);
        println!("Get Clock Sources: {:?}", get_clock_sources);
        for cs in clock_sources.iter() {
            let cs_name = std::ffi::CStr::from_ptr(cs.name.as_ptr()).to_str().unwrap();
            println!("Clock Source: {} {}", cs.index, cs_name);
        }

        // Buffer Size
        let mut buf_min_size: i32 = 0;
        let mut buf_max_size: i32 = 0;
        let mut buf_preferred_size: i32 = 0;
        let mut buf_granularity: i32 = 0;
        let get_buffer_size = asio_driver.get_buffer_size(
            &mut buf_min_size,
            &mut buf_max_size,
            &mut buf_preferred_size,
            &mut buf_granularity,
        );
        println!("Get Buffer Size: {:?}", get_buffer_size);
        println!(
            "Min: {}, Max: {}, Preferred: {}, Granularity: {}",
            buf_min_size, buf_max_size, buf_preferred_size, buf_granularity
        );

        // Construct Callbacks Struct
        let mut callbacks = asio_driver::AsioCallbacks {
            buffer_switch,
            sample_rate_did_change,
            asio_message,
            buffer_switch_time_info,
        };

        // Construct Buffer Info Structs
        let mut buf_infos = Vec::new();
        buf_infos.reserve_exact(num_channels as usize);
        for i in 0..num_output_channels {
            buf_infos.push(asio_driver::AsioBufferInfo::new_output(i));
        }
        for i in 0..num_input_channels {
            buf_infos.push(asio_driver::AsioBufferInfo::new_input(i));
        }

        // Create Buffers
        let create_buffers = asio_driver.create_buffers(
            buf_infos.as_mut_ptr(),
            num_channels,
            buf_preferred_size,
            &mut callbacks,
        );
        println!("Create Buffers: {:?}", create_buffers);

        // Start
        let start = asio_driver.start();
        println!("Start: {:?}", start);

        // Cram random bytes into output buffers to generate terrible noises
        let mut val: u8 = 0;
        for _ in 0..15000 {
            for ch in 0..(num_output_channels as usize) {
                for j in 0..(buf_preferred_size as usize) {
                    for k in 0..2 as usize {
                        let void_ptr = buf_infos[ch].buffers[k];
                        let u8_ptr =
                            std::mem::transmute::<*mut std::ffi::c_void, *mut u8>(void_ptr).add(j);
                        *u8_ptr = val;
                        val = val.wrapping_add(64);
                    }
                }
            }
        }

        // Dispose Buffers
        let dispose_buffers = asio_driver.dispose_buffers();
        println!("Dispose Buffers: {:?}", dispose_buffers);

        // Stop
        let stop = asio_driver.stop();
        println!("Stop: {:?}", stop);

        // // Get Error Message
        // let mut err_msg = asio_driver::AsioErrorMsg::new();
        // asio_driver.get_error_message(&mut err_msg);
        // println!("Error Message: {}", err_msg.to_str().unwrap());

        // // Get Sample Position
        // let mut samples: asio_driver::AsioSamples = 0;
        // let mut timestamp: asio_driver::AsioTimestamp = 0;
        // let get_sample_position = asio_driver.get_sample_position(&mut samples, &mut timestamp);
        // println!("Get Sample Position: {:?}", get_sample_position);
        // println!("Samples: {}", samples);
        // println!("Timestamp: {}", timestamp);

        // // Output Ready
        // let output_ready = asio_driver.output_ready();
        // println!("Output Ready: {:?}", output_ready);

        // let control_panel = asio_driver.control_panel();
        // println!("Control Panel: {:?}", control_panel);
    }
}
