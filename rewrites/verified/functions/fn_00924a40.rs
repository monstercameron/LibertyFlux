// original: 0x00924A40 run_or_queue_cmd_ctor (proposed)

/// Run-or-enqueue trampoline for a render command: run it now on the direct
/// path, or post it as a command object when the thread is queuing.
///
/// Reads the thread-local queue flag (TLS index from `TLS_INDEX`, flag word
/// at `+0x8cc`). When the flag is clear, calls the direct handler (callee 1, thiscall) with `(a1, a2, a3)` and returns its result.
/// Otherwise allocates a command object (callee 2, `ALLOC_SIZE` bytes); a null
/// is posted as-is (callee 3). A live object gets the treatment below and is
/// then posted (callee 3), whose answer is returned: runs the object constructor (callee 4, thiscall) with `(obj, a1, a2, a3)` and posts its result (callee 3).
/// The tag mixes the object's incoming `+0x04` word with the global counter
/// at `COUNTER` (masked to 14 bits) and the counter is incremented.
///
/// Original: 0x00924A40 (cdecl, 3 stack words).
lf_checker_rt::export!(cdecl, rw_00924A40(a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const TLS_INDEX: u32 = 0x017A_BA14;
        const FLAG_OFF: u32 = 0x8cc;
        const COUNTER: u32 = 0x0103_27A0;
        const BASE_VT: u32 = 0x00E7_E048;
        const ALLOC_SIZE: u32 = 0x20;
        const TAG_MASK: u32 = 0x3fff;
        #[inline(always)]
        unsafe fn rd(p: u32) -> u32 {
            unsafe { (p as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr(p: u32, v: u32) {
            unsafe { (p as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn rel(file_va: u32) -> u32 {
            lf_checker_rt::relocated(file_va)
        }
        let idx = rd(rel(TLS_INDEX));
        let tls = lf_checker_rt::tls_slot(idx as usize);
        if rd(tls.wrapping_add(FLAG_OFF)) == 0 {
            lf_checker_rt::callee_thiscall!(1, u32, a1, a2, a3)
        } else {
            let obj: u32 = lf_checker_rt::callee_cdecl!(2, u32, ALLOC_SIZE, 0u32);
            if obj == 0 {
                lf_checker_rt::callee_cdecl!(3, u32, 0u32)
            } else {
                let built: u32 =
                    lf_checker_rt::callee_thiscall!(4, u32, obj, a1, a2, a3);
                lf_checker_rt::callee_cdecl!(3, u32, built)
            }
        }
    }
});
