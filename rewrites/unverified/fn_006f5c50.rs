// original: 0x006F5C50 input_batch_flush (proposed)

/// Flush a batch of input work for `this`, returning 9 on the stage-1 path.
///
/// STAGE 1 (partial): covers the timer prelude (both clock paths), the outer
/// list skeleton with both inner loops pinned empty, the skip of the timed
/// wait, attach and backlog blocks, the six bit-writer calls that serialize
/// the batch header, and the flag merge. The inner record loop, the timed
/// wait block, the attach block and the backlog block are NOT covered: the
/// contract pins their gates off, and the rewrite faults if they open.
/// Thiscall with two stack words; the incoming arg1 slot is reused as a
/// local, so the stack check is off.
lf_checker_rt::export!(thiscall, rw_006F5C50(this: u32, a0: u32, _a1: u32) -> u32 {
    unsafe {
        const ID_CLOCK_FN: u32 = 6;
        const ID_CLOCK_Q: u32 = 7;
        const ID_TICK: u32 = 8;
        const ID_BITW: u32 = 9;
        const G_CLOCK_FN: u32 = 0x017A_CD20;
        const G_CLOCK_Q: u32 = 0x017A_CD00;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn fault() -> ! {
            unsafe {
                core::ptr::read_volatile(0 as *const u8);
            }
            unreachable!("stage gate opened");
        }

        // Timer prelude (same shape as input_device_reset).
        let tick = if rd32(lf_checker_rt::relocated(G_CLOCK_FN)) != 0 {
            lf_checker_rt::callee_cdecl!(ID_CLOCK_FN, u32,);
            let mut q = [0u32; 2];
            let mut qi = [0u32; 1];
            lf_checker_rt::callee_cdecl!(
                ID_CLOCK_Q, u32, q.as_mut_ptr() as u32, qi.as_mut_ptr() as u32
            );
            qi[0]
        } else {
            lf_checker_rt::callee_stdcall!(ID_TICK, u32,)
        };
        let _ = tick;
        // Outer skeleton: both lists must be empty in stage 1.
        if rd32(this.wrapping_add(0x44).wrapping_add(8)) != 0 {
            fault();
        }
        if rd32(this + 0x20) == 3 {
            if rd32(this.wrapping_add(0x50).wrapping_add(8)) != 0 {
                fault();
            }
        }
        // Timed wait, attach and backlog blocks must be skipped.
        if rd32(this + 0x38) != 0 {
            fault();
        }
        if rd32(this + 0x74) != 0 {
            fault();
        }
        if rd32(this + 0x3c) != 0 {
            fault();
        }
        // Backlog-clear flag merge.
        let b88 = rd8(this + 0x88) & 0xEF;
        wr8(this + 0x88, b88);
        if b88 & 2 != 0 {
            wr8(this + 0x88, b88 & 0xFD);
        }
        // Tail merge: reloads bl from the flag byte, masks to 0x20 and
        // xors it back, i.e. flag bit 5 := bl0. Stage 1 always has bl0 = 0
        // (both arg1-slot stores on this path write 0; the 1-store is in
        // the skipped inner body, guarded above), so bit 5 is cleared.
        let b88b = rd8(this + 0x88);
        let bl0 = 0u8;
        let bl = (bl0 << 5) ^ b88b;
        wr8(this + 0x88, b88b ^ (bl & 0x20));
        if b88 & 2 != 0 && rd32(this + 0x20) == 3 {
            let s18 = rd32(this + 0x18);
            let edi_w = rd16(this + 0x82) as u32;
            let ebx_v = rd32(this + 0x7c);
            lf_checker_rt::callee_cdecl!(ID_BITW, u32, a0, 9, 0x0a, 0);
            lf_checker_rt::callee_cdecl!(ID_BITW, u32, a0, 2, 4, 0x0a);
            lf_checker_rt::callee_cdecl!(ID_BITW, u32, a0, s18, 4, 0x0e);
            lf_checker_rt::callee_cdecl!(ID_BITW, u32, a0, 0, 0x10, 0x12);
            lf_checker_rt::callee_cdecl!(ID_BITW, u32, a0, edi_w, 0x10, 0x22);
            lf_checker_rt::callee_cdecl!(ID_BITW, u32, a0, ebx_v, 0x16, 0x32);
            return 9;
        }
        0
    }
});
