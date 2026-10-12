// original: 0x00A8ADC0 streaming_make_space_for

/// Bind the common pool and repeatedly probe its opening helper until it
/// succeeds. After a failed probe, ask the stage helper to release room,
/// clear the one-word out slot, and retry. On success, pass the slot to the
/// ready helper. The retry count exits after the 1,001st failed attempt.
///
/// `this` is not read by this routine; `mode` is forwarded to the probe as its
/// third argument. The cookie helper's ECX value depends on the native stack
/// frame and is therefore not compared by this contract.
lf_checker_rt::export!(thiscall, rw_00a8adc0(this: u32, mode: u32) -> u32 {
    unsafe {
        const CALLEE_PROBE: u32 = 1;
        const CALLEE_READY: u32 = 2;
        const CALLEE_COOKIE: u32 = 3;
        const CALLEE_BIND: u32 = 4;
        const CALLEE_STAGE: u32 = 5;
        const POOL_OBJECT: u32 = 0x01173750;
        const PROBE_PATH: u32 = 0x00ea2960;
        const STAGE_BYTES: u32 = 0x20;

        let mut out_word = 0u32;
        let out_pointer = core::ptr::addr_of_mut!(out_word) as u32;
        lf_checker_rt::callee_thiscall!(
            CALLEE_BIND,
            u32,
            lf_checker_rt::relocated(POOL_OBJECT)
        );

        let mut retries = 0u32;
        loop {
            let opened = lf_checker_rt::callee_cdecl!(
                CALLEE_PROBE,
                u32,
                lf_checker_rt::relocated(PROBE_PATH),
                out_pointer,
                mode
            );
            if opened as u8 != 0 {
                lf_checker_rt::callee_cdecl!(CALLEE_READY, u32, out_pointer);
                lf_checker_rt::callee_thiscall!(CALLEE_COOKIE, u32, 0);
                return 1;
            }

            out_word = 0;
            let stage_ready = lf_checker_rt::callee_thiscall!(
                CALLEE_STAGE,
                u32,
                lf_checker_rt::relocated(POOL_OBJECT),
                STAGE_BYTES
            );
            if stage_ready as u8 == 0 {
                retries += 1;
                if retries > 1000 {
                    lf_checker_rt::callee_thiscall!(CALLEE_COOKIE, u32, 0);
                    return 0;
                }
            }
        }
    }
});
