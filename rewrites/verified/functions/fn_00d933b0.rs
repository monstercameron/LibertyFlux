// original: 0x00D933B0 mainloop_update_gate (proposed)

/// Run the gated periodic updates, then tail-call the frame worker.
///
/// `this` points to an object with two generation counters at `+0xe98` and
/// `+0xe9c`. Two independent time gates compare the global tick against a
/// stored timestamp: gate 1 fires when `tick - last1 >= thresh1`, gate 2
/// when `tick - last2 >= thresh2` (unsigned wraparound subtraction). A fired
/// gate stores the current tick first, so it cannot fire twice on one tick.
///
/// When gate 1 fires: if the two counters differ, a resync callee runs, then
/// two update callees run in order. The heartbeat callee runs on every call.
/// When the enable flag byte is set and gate 2 fires, the slow callee runs.
/// Control then passes to the frame worker with the same object pointer; its
/// return value is this function's return value (a tail jump, so no stack is
/// used for the transfer).
///
/// Edge cases: a zero threshold fires every call; the unsigned compare makes
/// a wrapped tick work. The tick is re-read for gate 2, so a heartbeat that
/// advances it can open gate 2 in the same call.
///
/// Original: 0x00D933B0 (thiscall, no stack arguments; the listed size of 762
/// bytes covers the following function too, the real body is about 116 bytes
/// ending in the tail jump).
lf_checker_rt::export!(thiscall, rw_00D933B0(this: u32) -> u32 {
    unsafe {
        const TICK: u32 = 0x011735b4;
        const THRESH1: u32 = 0x010482d4;
        const LAST1: u32 = 0x016b8f9c;
        const ENABLE: u32 = 0x016b7bf3;
        const THRESH2: u32 = 0x010482c8;
        const LAST2: u32 = 0x016b7c58;
        const GEN_A: u32 = 0xe98;
        const GEN_B: u32 = 0xe9c;
        const CALLEE_RESYNC: u32 = 1;
        const CALLEE_UPD1: u32 = 2;
        const CALLEE_UPD2: u32 = 3;
        const CALLEE_HEARTBEAT: u32 = 4;
        const CALLEE_SLOW: u32 = 5;
        const CALLEE_FRAME: u32 = 6;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        let tick = rd32(lf_checker_rt::relocated(TICK));
        if tick.wrapping_sub(rd32(lf_checker_rt::relocated(LAST1))) >= rd32(lf_checker_rt::relocated(THRESH1)) {
            wr32(lf_checker_rt::relocated(LAST1), tick);
            if rd32(this.wrapping_add(GEN_B)) != rd32(this.wrapping_add(GEN_A)) {
                lf_checker_rt::callee_thiscall!(CALLEE_RESYNC, u32, this);
            }
            lf_checker_rt::callee_thiscall!(CALLEE_UPD1, u32, this);
            lf_checker_rt::callee_thiscall!(CALLEE_UPD2, u32, this);
        }
        lf_checker_rt::callee_cdecl!(CALLEE_HEARTBEAT, u32,);
        if unsafe { (lf_checker_rt::relocated(ENABLE) as *const u8).read() } != 0 {
            let tick2 = rd32(lf_checker_rt::relocated(TICK));
            if tick2.wrapping_sub(rd32(lf_checker_rt::relocated(LAST2))) >= rd32(lf_checker_rt::relocated(THRESH2)) {
                wr32(lf_checker_rt::relocated(LAST2), tick2);
                lf_checker_rt::callee_cdecl!(CALLEE_SLOW, u32,);
            }
        }
        lf_checker_rt::callee_thiscall!(CALLEE_FRAME, u32, this)
    }
});
