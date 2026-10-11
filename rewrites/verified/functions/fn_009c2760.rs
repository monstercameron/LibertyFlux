// original: 0x009C2760 NativeImpl_STOP_CUTSCENE

/// Update the active cutscene timer when the active flag is set, invoke the
/// corresponding stop helpers, and use the mode and request flags to select a
/// timed stop call. The first cdecl argument's low byte also gates the timed
/// stop call. Store the final stop code in the request-status global and
/// return the EAX value left by the selected path.
lf_checker_rt::export!(cdecl, rw_009c2760(context_flag_word: u32) -> u32 {
    unsafe {
        const CUTSCENE_ACTIVE: u32 = 0x0129_576f;
        const CUTSCENE_ARGUMENT: u32 = 0x0129_5764;
        const CUTSCENE_TIME: u32 = 0x0129_5740;
        const CUTSCENE_DELTA: u32 = 0x00fe_8c44;
        const STREAMING_INDEX: u32 = 0x0129_4730;
        const STREAMING_TABLE: u32 = 0x0129_3e58;
        const CUTSCENE_MODE: u32 = 0x0129_5738;
        const REQUEST_FLAGS: u32 = 0x0129_577c;
        const STOP_ENABLED: u32 = 0x0129_5774;
        const REQUEST_STATUS: u32 = 0x0129_475c;
        const TIMER_FIELD: u32 = 0x10;
        const STOP_MANAGER: u32 = 0x0128_2fa8;
        const TIMER_MANAGER: u32 = 0x0128_e310;

        let active = lf_checker_rt::global::<u8>(CUTSCENE_ACTIVE).read_unaligned() != 0;
        let cutscene_argument = lf_checker_rt::relocated(CUTSCENE_ARGUMENT);

        if active {
            let current_time = lf_checker_rt::global::<f32>(CUTSCENE_TIME).read_unaligned();
            let time_delta = lf_checker_rt::global::<f32>(CUTSCENE_DELTA).read_unaligned();
            let current_time = core::hint::black_box(current_time);
            let time_delta = core::hint::black_box(time_delta);
            let next_time = core::hint::black_box(current_time + time_delta);

            let table_index = lf_checker_rt::global::<u32>(STREAMING_INDEX).read_unaligned();
            let table_slot = STREAMING_TABLE.wrapping_add(table_index.wrapping_mul(4));
            let context = lf_checker_rt::global::<u32>(table_slot).read_unaligned();
            (context.wrapping_add(TIMER_FIELD) as usize as *mut f32)
                .write_unaligned(next_time);

            let _ = lf_checker_rt::callee_cdecl!(1, u32, cutscene_argument);
            let _ = lf_checker_rt::callee_thiscall!(
                2,
                u32,
                lf_checker_rt::relocated(STOP_MANAGER),
                1u32,
            );
        } else {
            let _ = lf_checker_rt::callee_cdecl!(3, u32, cutscene_argument);
        }

        let mode = lf_checker_rt::global::<u32>(CUTSCENE_MODE).read_unaligned();
        let request_flags = lf_checker_rt::global::<u32>(REQUEST_FLAGS).read_unaligned();
        let mut argument_flag = context_flag_word as u8;
        let mut result = mode;

        if mode == 1 {
            argument_flag = 0;
        } else {
            result = mode.wrapping_sub(1);
            let current_index = lf_checker_rt::global::<u32>(STREAMING_INDEX).read_unaligned();
            if current_index == result {
                argument_flag = 0;
            }
        }

        let skip_for_argument = argument_flag != 0 && request_flags & 1 == 0;
        let active_byte = lf_checker_rt::global::<u8>(CUTSCENE_ACTIVE).read_unaligned() != 0;
        let skip_for_request = request_flags & 0x200 != 0 && !active_byte;

        if !skip_for_argument && !skip_for_request {
            let mut timeout = 0x320u32;
            if !active_byte {
                if request_flags & 8 != 0 {
                    timeout = 0x9c4;
                }
                if request_flags & 4 != 0 {
                    timeout = 0xfa;
                }
            }
            let delay = timeout.wrapping_sub(0x3c);
            result = lf_checker_rt::callee_thiscall!(
                4,
                u32,
                lf_checker_rt::relocated(TIMER_MANAGER),
                delay,
                0u32,
                1u32,
            );
        }

        let should_report_pending =
            lf_checker_rt::global::<u8>(STOP_ENABLED).read_unaligned() != 0;
        let status = if should_report_pending && argument_flag != 0 {
            9u32
        } else {
            10u32
        };
        lf_checker_rt::global::<u32>(REQUEST_STATUS).write_unaligned(status);
        result
    }
});
