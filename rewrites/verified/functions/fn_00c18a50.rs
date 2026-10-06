// original: 0x00c18a50 replay_cmd6_store_string

/// Store command 6: a bounded string, two argument words and a helper call.
///
/// Returns 0 when the readiness gate fails. Otherwise writes command id 6 at
/// buffer `+8`, copies the first argument string into the buffer at `+0xc`,
/// zeroes the word at `+0x20a`, stores the second argument at `+0x20c` and
/// the fourth at `+0x210`, calls the payload helper with
/// (buffer `+0x21c`, third argument, fourth argument), releases the mutex and
/// returns 1.
///
/// Original: 0x00C18A50 (thiscall, four stack words).
lf_checker_rt::export!(thiscall, rw_00c18a50(this: u32, a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
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
        const PAYLOAD: u32 = 3;
        const BUF_OFF: u32 = 0x28;
        const HANDLE_OFF: u32 = 0x30;
        let ok: u32 = lf_checker_rt::callee_thiscall!(GATE, u32, this);
        if ok & 0xff == 0 {
            return 0;
        }
        let buf = ((this + BUF_OFF) as *const u32).read_unaligned();
        ((buf + 8) as *mut u32).write_unaligned(6);
        lf_checker_rt::callee_cdecl!(COPY_STRING, u32, buf + 0x0c, 0x100, a0, 0xff);
        let buf = ((this + BUF_OFF) as *const u32).read_unaligned();
        ((buf + 0x20a) as *mut u16).write_unaligned(0);
        ((buf + 0x20c) as *mut u32).write_unaligned(a1);
        ((buf + 0x210) as *mut u32).write_unaligned(a3);
        lf_checker_rt::callee_cdecl!(PAYLOAD, u32, buf + 0x21c, a2, a3);
        let handle = ((this + HANDLE_OFF) as *const u32).read_unaligned();
        release_mutex(handle);
        1
    }
});
