// original: 0x00c18ae0 replay_cmd3_store_string

/// Store command 3 with a bounded string payload, then mark active.
///
/// Returns 0 at once when the active flag at `+0x24` is already set or the
/// readiness gate fails. Otherwise writes command id 3 at buffer `+8`, copies
/// the argument string into the buffer at `+0xc` (at most 0x100 words, the
/// callee's bound), zeroes the word at `+0x20a`, releases the mutex, sets the
/// flag and returns 1.
///
/// Original: 0x00C18AE0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00c18ae0(this: u32, arg: u32) -> u32 {
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
        const COPY_STRING: u32 = 2;
        const BUF_OFF: u32 = 0x28;
        const HANDLE_OFF: u32 = 0x30;
        const ACTIVE_OFF: u32 = 0x24;
        if ((this + ACTIVE_OFF) as *const u8).read() != 0 {
            return 0;
        }
        let ok: u32 = lf_checker_rt::callee_thiscall!(GATE, u32, this);
        if ok & 0xff == 0 {
            return 0;
        }
        let buf = ((this + BUF_OFF) as *const u32).read_unaligned();
        ((buf + 8) as *mut u32).write_unaligned(3);
        lf_checker_rt::callee_cdecl!(COPY_STRING, u32, buf + 0x0c, 0x100, arg, 0xff);
        let buf2 = ((this + BUF_OFF) as *const u32).read_unaligned();
        ((buf2 + 0x20a) as *mut u16).write_unaligned(0);
        let handle = ((this + HANDLE_OFF) as *const u32).read_unaligned();
        release_mutex(handle);
        ((this + ACTIVE_OFF) as *mut u8).write(1);
        1
    }
});
