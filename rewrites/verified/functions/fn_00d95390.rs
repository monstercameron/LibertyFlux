// original: 0x00d95390 phase_ring_dispatch (proposed)

/// Advance a six-state ring by trying each state's successor handlers in
/// turn until one accepts.
///
/// `this` points to an object whose phase word at `+0xC2C` holds 0..5. When
/// the reset global is positive the phase is first forced to 0. A phase
/// above 5 returns 0 with no calls. Otherwise the five handler callees are
/// tried in the ring order starting just after the current phase (handler
/// ids in ring order are 4, 0, 1, 2, 3); the first handler returning nonzero
/// wins: the phase becomes that handler's successor phase (id 4 -> 1,
/// 0 -> 2, 1 -> 3, 2 -> 4, 3 -> 5) and its exact answer is returned. When
/// every handler answers 0 the phase is unchanged and 0 is returned.
///
/// Original: 0x00d95390 (thiscall, no stack arguments, jump-table dispatch
/// over six entries with states 0 and 5 sharing one block, full-eax result).
lf_checker_rt::export!(thiscall, rw_00d95390(this: u32) -> u32 {
    unsafe {
        const PHASE: u32 = 0xc2c;
        const RESET: u32 = 0x16b6af4;
        // Try-order of handler ids per phase; states 0 and 5 share a row.
        const ORDER: [[u32; 5]; 6] = [
            [4, 0, 1, 2, 3],
            [0, 1, 2, 3, 4],
            [1, 2, 3, 4, 0],
            [2, 3, 4, 0, 1],
            [3, 4, 0, 1, 2],
            [4, 0, 1, 2, 3],
        ];
        // Successor phase per winning handler id.
        const NEXT: [u32; 5] = [2, 3, 4, 5, 1];
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        if (rd32(lf_checker_rt::relocated(RESET)) as i32) > 0 {
            wr32(this + PHASE, 0);
        }
        let phase = rd32(this + PHASE);
        if phase > 5 {
            return 0;
        }
        for k in 0..5usize {
            let id = ORDER[phase as usize][k];
            let r: u32 = lf_checker_rt::callee_thiscall!(id, u32, this);
            if r != 0 {
                wr32(this + PHASE, NEXT[id as usize]);
                return r;
            }
        }
        0
    }
});
