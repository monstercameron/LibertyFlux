// original: 0x00AC2DD0 stream_rotate_buffers (proposed)

/// Rotate the streaming buffer cursors and hand off to the pump.
///
/// The original advances the 3-slot cursor (wrapping 2 to 0), publishes the
/// previous base, computes the new base, clears the new slot, then advances
/// the 2-slot cursor the same way, publishes the taken count and clears the
/// done flag (cdecl, no arguments), tail-calling the pump. The rewrite calls
/// the pump through the checker and returns its answer.
lf_checker_rt::export!(cdecl, rw_00AC2DD0() -> u32 {
    unsafe {
        const CUR0: u32 = 0x0150E244;
        const PREV_BASE: u32 = 0x0103EED8;
        const CUR_BASE: u32 = 0x0103EEDC;
        const BASE0: u32 = 0x01510BA0;
        const STRIDE0: u32 = 0x14000;
        const SLOT0: u32 = 0x0154DFD4;
        const CUR1: u32 = 0x0150E250;
        const BASE1REG: u32 = 0x0103EEE8;
        const BASE1: u32 = 0x0150E290;
        const STRIDE1: u32 = 0x1400;
        const TAKEN_IN: u32 = 0x0154CBBC;
        const TAKEN: u32 = 0x0150E240;
        const QUEUED: u32 = 0x0150E248;
        const SLOT1: u32 = 0x0154DFE0;
        const DONE: u32 = 0x0150E255;
        const PUMP: u32 = 1;
        let c0 = (lf_checker_rt::relocated(CUR0) as *const u32).read_unaligned();
        let n0 = c0.wrapping_add(1);
        let n0 = if n0 == 3 { 0 } else { n0 };
        let base = (lf_checker_rt::relocated(CUR_BASE) as *const u32).read_unaligned();
        (lf_checker_rt::relocated(PREV_BASE) as *mut u32).write_unaligned(base);
        let a0 = n0.wrapping_mul(STRIDE0).wrapping_add(lf_checker_rt::relocated(BASE0));
        (lf_checker_rt::relocated(CUR0) as *mut u32).write_unaligned(n0);
        (lf_checker_rt::relocated(SLOT0).wrapping_add(n0.wrapping_mul(4)) as *mut u32)
            .write_unaligned(0);
        let c1 = (lf_checker_rt::relocated(CUR1) as *const u32).read_unaligned();
        let n1 = c1.wrapping_add(1);
        let n1 = if n1 == 2 { 0 } else { n1 };
        (lf_checker_rt::relocated(CUR_BASE) as *mut u32).write_unaligned(a0);
        let a1 = n1.wrapping_mul(STRIDE1).wrapping_add(lf_checker_rt::relocated(BASE1));
        (lf_checker_rt::relocated(BASE1REG) as *mut u32).write_unaligned(a1);
        let taken = (lf_checker_rt::relocated(TAKEN_IN) as *const u32).read_unaligned();
        (lf_checker_rt::relocated(QUEUED) as *mut u32).write_unaligned(0);
        (lf_checker_rt::relocated(CUR1) as *mut u32).write_unaligned(n1);
        (lf_checker_rt::relocated(SLOT1).wrapping_add(n1.wrapping_mul(4)) as *mut u32)
            .write_unaligned(0);
        (lf_checker_rt::relocated(TAKEN) as *mut u32).write_unaligned(taken);
        (lf_checker_rt::relocated(TAKEN_IN) as *mut u32).write_unaligned(0);
        (lf_checker_rt::relocated(DONE) as *mut u8).write(0);
        lf_checker_rt::callee_cdecl!(PUMP, u32,)
    }
});
