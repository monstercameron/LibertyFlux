// original: 0x00c18c20 replay_cmd4_clear

/// Deactivate the channel: command word 4, then clear the active flag.
///
/// Returns 0 at once when the active flag at `+0x24` is already clear or the
/// readiness gate fails. Otherwise writes command id 4 at buffer `+8`,
/// releases the mutex, clears the flag and returns 1.
///
/// Original: 0x00C18C20 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00c18c20(this: u32) -> u32 {
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
        const ACTIVE_OFF: u32 = 0x24;
        if ((this + ACTIVE_OFF) as *const u8).read() == 0 {
            return 0;
        }
        let ok: u32 = lf_checker_rt::callee_thiscall!(GATE, u32, this);
        if ok & 0xff == 0 {
            return 0;
        }
        let buf = ((this + BUF_OFF) as *const u32).read_unaligned();
        ((buf + 8) as *mut u32).write_unaligned(4);
        let handle = ((this + HANDLE_OFF) as *const u32).read_unaligned();
        release_mutex(handle);
        ((this + ACTIVE_OFF) as *mut u8).write(0);
        1
    }
});
