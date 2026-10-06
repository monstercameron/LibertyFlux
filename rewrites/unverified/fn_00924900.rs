// original: 0x00924900 run_or_queue_cmd_dc (proposed)

/// Run-or-enqueue trampoline for a render command: run it now on the direct
/// path, or post it as a command object when the thread is queuing.
///
/// Reads the thread-local queue flag (TLS index from `TLS_INDEX`, flag word
/// at `+0x8cc`). When the flag is clear, calls the direct handler (callee 1) with `(a1, a2, 1, 5)` and returns its result.
/// Otherwise allocates a command object (callee 2, `ALLOC_SIZE` bytes); a null
/// is posted as-is (callee 3). A live object gets the treatment below and is
/// then posted (callee 3), whose answer is returned: installs the base vtable, mixes the global counter into the tag at `+0x04`, then stores `a1` at `+0x08` and copies four dwords from `*a2` to `+0x10`, posts the object (callee 3).
/// The tag mixes the object's incoming `+0x04` word with the global counter
/// at `COUNTER` (masked to 14 bits) and the counter is incremented.
///
/// Original: 0x00924900 (cdecl, 2 stack words).
lf_checker_rt::export!(cdecl, rw_00924900(a1: u32, a2: u32) -> u32 {
    unsafe {
        const TLS_INDEX: u32 = 0x017A_BA14;
        const FLAG_OFF: u32 = 0x8cc;
        const COUNTER: u32 = 0x0103_27A0;
        const BASE_VT: u32 = 0x00E7_E048;
        const FINAL_VT: u32 = 0x00E8_62DC;
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
            lf_checker_rt::callee_cdecl!(1, u32, a1, a2, 1u32, 5u32)
        } else {
            let obj: u32 = lf_checker_rt::callee_cdecl!(2, u32, ALLOC_SIZE, 0u32);
            if obj == 0 {
                lf_checker_rt::callee_cdecl!(3, u32, 0u32)
            } else {
                let mut tag = rd(obj.wrapping_add(4));
                
                wr(obj, rel(BASE_VT));
                tag ^= rd(rel(COUNTER));
                tag &= TAG_MASK;
                wr(obj.wrapping_add(4), rd(obj.wrapping_add(4)) ^ tag);
                wr(rel(COUNTER), rd(rel(COUNTER)).wrapping_add(1));
                wr(obj, rel(FINAL_VT));
                wr(obj.wrapping_add(8), a1);
                for i in 0..4u32 {
                    wr(obj.wrapping_add(0x10 + i * 4), rd(a2.wrapping_add(i * 4)));
                }
                lf_checker_rt::callee_cdecl!(3, u32, obj)
            }
        }
    }
});
