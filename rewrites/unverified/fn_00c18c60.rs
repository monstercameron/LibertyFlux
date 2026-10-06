// original: 0x00c18c60 replay_cmd1_copy4

/// Store command 1 with a four-word payload copied from the argument.
///
/// Returns 0 when the readiness gate fails. Otherwise writes command id 1 at
/// buffer `+8`, copies four dwords from `arg` to buffer `+0x20c..+0x218`,
/// releases the mutex and returns 1.
///
/// Original: 0x00C18C60 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00c18c60(this: u32, arg: u32) -> u32 {
    unsafe {
        const RELEASE_MUTEX_SLOT: u32 = 0x00e7_31b0;
        #[inline(always)]
        unsafe fn release_mutex(handle: u32) -> u32 {
            unsafe {
                let slot = lf_checker_rt::relocated(RELEASE_MUTEX_SLOT) as *const u32;
                let f: extern "stdcall" fn(u32) -> u32 =
                    core::mem::transmute(slot.read() as usize);
                f(handle)
            }
        }
        const GATE: u32 = 1;
        const BUF_OFF: u32 = 0x28;
        const HANDLE_OFF: u32 = 0x30;
        const PAYLOAD_OFF: u32 = 0x20c;
        let ok: u32 = lf_checker_rt::callee_thiscall!(GATE, u32, this);
        if ok & 0xff == 0 {
            return 0;
        }
        let buf = ((this + BUF_OFF) as *const u32).read_unaligned();
        ((buf + 8) as *mut u32).write_unaligned(1);
        for i in 0..4u32 {
            let w = ((arg + i * 4) as *const u32).read_unaligned();
            ((buf + PAYLOAD_OFF + i * 4) as *mut u32).write_unaligned(w);
        }
        let handle = ((this + HANDLE_OFF) as *const u32).read_unaligned();
        release_mutex(handle);
        1
    }
});
