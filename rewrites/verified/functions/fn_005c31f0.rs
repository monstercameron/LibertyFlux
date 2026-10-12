// original: 0x005C31F0 poll_time_and_window_state

/// Poll the window clock when the thread-local mode bit is set, advancing the
/// saved timestamp only after an unsigned elapsed interval of at least 15.
/// The clock helper writes a 64-bit counter and a 32-bit timestamp through two
/// pointers to local storage. If the elapsed interval qualifies, the function
/// runs its update helper and optionally polls the configured callback. The
/// callback result is ignored; a zero callback-state byte changes to one. It then polls the
/// optional window object, stores its boolean result as the current active
/// state, and notifies the state helper with the inverse boolean. Its dynamic
/// function pointers, TLS state, object table, globals, and callback results
/// are supplied by the checker contract.
lf_checker_rt::export!(cdecl, rw_005c31f0() -> () {
    const INIT_CALLEE: u32 = 1;
    const TIME_SOURCE_CALLEE: u32 = 2;
    const TIME_QUERY_CALLEE: u32 = 3;
    const TIME_FALLBACK_CALLEE: u32 = 4;
    const ADVANCE_CALLEE: u32 = 5;
    const CALLBACK_CALLEE: u32 = 6;
    const WINDOW_METHOD_CALLEE: u32 = 7;
    const NOTIFY_CALLEE: u32 = 8;

    const TLS_SLOT: usize = 0;
    const TLS_MODE_OFFSET: usize = 0x8D0;
    const MIN_ELAPSED_TICKS: u32 = 15;

    const TIME_SOURCE_VA: u32 = 0x017ACD20;
    const TIME_QUERY_VA: u32 = 0x017ACD00;
    const TIME_FALLBACK_VA: u32 = 0x00E73474;
    const LAST_TIME_VA: u32 = 0x01BB5648;
    const CALLBACK_ENABLED_VA: u32 = 0x019F0B48;
    const CALLBACK_TARGET_VA: u32 = 0x019F0AF0;
    const CALLBACK_STATE_VA: u32 = 0x019F0BDF;
    const WINDOW_OBJECT_VA: u32 = 0x019F2654;
    const ACTIVE_STATE_VA: u32 = 0x01BB6818;

    let init_call: extern "cdecl" fn() -> u32 = unsafe {
        core::mem::transmute(lf_checker_rt::callee_addr(INIT_CALLEE) as usize)
    };
    let _: u32 = init_call();

    let tls_base = lf_checker_rt::tls_slot(TLS_SLOT) as *const u8;
    let tls_mode = unsafe {
        core::ptr::read_unaligned(tls_base.wrapping_add(TLS_MODE_OFFSET) as *const u32)
    };
    if (tls_mode >> 1) & 1 != 0 {
        let time_source =
            unsafe { lf_checker_rt::global::<u32>(TIME_SOURCE_VA).read_unaligned() };
        let current_time = if time_source != 0 {
            let time_source_call: extern "cdecl" fn() -> u32 = unsafe {
                core::mem::transmute(lf_checker_rt::callee_addr(TIME_SOURCE_CALLEE) as usize)
            };
            let _: u32 = time_source_call();

            let _query_target =
                unsafe { lf_checker_rt::global::<u32>(TIME_QUERY_VA).read_unaligned() };
            let mut counter_pair = 0u64;
            let mut timestamp = 0u32;
            let counter_ptr = (&mut counter_pair as *mut u64) as u32;
            let timestamp_ptr = (&mut timestamp as *mut u32) as u32;
            let _: u32 = lf_checker_rt::callee_cdecl!(
                TIME_QUERY_CALLEE,
                u32,
                counter_ptr,
                timestamp_ptr
            );
            timestamp
        } else {
            let _fallback_target =
                unsafe { lf_checker_rt::global::<u32>(TIME_FALLBACK_VA).read_unaligned() };
            let time_fallback_call: extern "cdecl" fn() -> u32 = unsafe {
                core::mem::transmute(lf_checker_rt::callee_addr(TIME_FALLBACK_CALLEE) as usize)
            };
            time_fallback_call()
        };

        let last_time =
            unsafe { lf_checker_rt::global::<u32>(LAST_TIME_VA).read_unaligned() };
        if current_time.wrapping_sub(last_time) >= MIN_ELAPSED_TICKS {
            let advance_call: extern "cdecl" fn() -> u32 = unsafe {
                core::mem::transmute(lf_checker_rt::callee_addr(ADVANCE_CALLEE) as usize)
            };
            let _: u32 = advance_call();

            let callback_enabled = unsafe {
                lf_checker_rt::global::<u8>(CALLBACK_ENABLED_VA).read_unaligned()
            };
            let callback_target = unsafe {
                lf_checker_rt::global::<u32>(CALLBACK_TARGET_VA).read_unaligned()
            };
            if callback_enabled != 0 && callback_target != 0 {
                let callback_call: extern "cdecl" fn() -> u32 = unsafe {
                    core::mem::transmute(lf_checker_rt::callee_addr(CALLBACK_CALLEE) as usize)
                };
                let _: u32 = callback_call();
                let previous_callback_state = unsafe {
                    lf_checker_rt::global::<u8>(CALLBACK_STATE_VA).read_unaligned()
                };
                let callback_state = if previous_callback_state == 0 {
                    1
                } else {
                    previous_callback_state
                };
                unsafe {
                    lf_checker_rt::global::<u8>(CALLBACK_STATE_VA)
                        .write_unaligned(callback_state);
                }
            }

            unsafe {
                lf_checker_rt::global::<u32>(LAST_TIME_VA).write_unaligned(current_time);
            }
        }
    }

    let window_object =
        unsafe { lf_checker_rt::global::<u32>(WINDOW_OBJECT_VA).read_unaligned() };
    let is_active = if window_object != 0 {
        lf_checker_rt::callee_thiscall!(
            WINDOW_METHOD_CALLEE,
            u32,
            window_object,
            window_object
        ) as u8
            != 0
    } else {
        false
    };
    let active_byte = u8::from(is_active);
    unsafe {
        lf_checker_rt::global::<u8>(ACTIVE_STATE_VA).write_unaligned(active_byte);
    }
    let notification_value = u32::from(!is_active);
    let _: u32 = lf_checker_rt::callee_cdecl!(NOTIFY_CALLEE, u32, notification_value);
});
