// original: 0x00c18e60 replay_cmd5_bounded

/// Store command 5 when the channel is active and the size is in range.
///
/// Returns 0 at once when the active flag at `+0x24` is clear, when the third
/// argument is above 0x7e9000 (compared UNSIGNED: high-bit values fail), or
/// when the readiness gate fails. Otherwise writes command id 5 at buffer
/// `+8`, stores the first argument at `+0x20c`, the size at `+0x210` and the
/// fourth argument's bits at `+0x214`, calls the payload helper with
/// (buffer `+0x21c`, second argument, size), releases the mutex and returns 1.
///
/// Original: 0x00C18E60 (thiscall, four stack words).
lf_checker_rt::export!(thiscall, rw_00c18e60(this: u32, a0: u32, a1: u32, size: u32, a3: u32) -> u32 {
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
        const PAYLOAD: u32 = 2;
        const BUF_OFF: u32 = 0x28;
        const HANDLE_OFF: u32 = 0x30;
        const ACTIVE_OFF: u32 = 0x24;
        const SIZE_LIMIT: u32 = 0x7e_9000;
        if ((this + ACTIVE_OFF) as *const u8).read() == 0 {
            return 0;
        }
        if size > SIZE_LIMIT {
            return 0;
        }
        let ok: u32 = lf_checker_rt::callee_thiscall!(GATE, u32, this);
        if ok & 0xff == 0 {
            return 0;
        }
        let buf = ((this + BUF_OFF) as *const u32).read_unaligned();
        ((buf + 8) as *mut u32).write_unaligned(5);
        ((buf + 0x20c) as *mut u32).write_unaligned(a0);
        ((buf + 0x210) as *mut u32).write_unaligned(size);
        ((buf + 0x214) as *mut u32).write_unaligned(a3);
        lf_checker_rt::callee_cdecl!(PAYLOAD, u32, buf + 0x21c, a1, size);
        let handle = ((this + HANDLE_OFF) as *const u32).read_unaligned();
        release_mutex(handle);
        1
    }
});
