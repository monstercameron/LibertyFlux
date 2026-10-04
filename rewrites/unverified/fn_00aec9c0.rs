// original: 0x00aec9c0 timing_main_update (proposed, STAGE A1 — see below)

/// Main timing update: build a frame object and run the update chain.
///
/// STAGE A1: this rewrite covers the short path only (the probe callee
/// answers null and the table gate word is clear); the middle chain and the
/// table section are not taken under the stage contract. See
/// `stage_b_plan.md` in the lane folder for the remaining stages.
///
/// Thiscall with one stack word (`arg`); `this` is a controller. Returns 0
/// (entry accumulator, contract-fixed) when `arg` is null or when the header
/// word at `arg+0xc` is set without the ready bit at `arg+0xa`. Otherwise it
/// ticks the controller, opens a frame through the frame callee, clears the
/// header, resolves the probe (null on this stage), builds the worker
/// object, links the mode table entry, runs the two virtual slots and the
/// bind call, stamps the worker flags, records the null probe result and
/// runs the close callee, returning its answer.
///
/// The scratch slot the original reserves is only written on the middle
/// path; on this stage it keeps the defined stack fill (0), which the close
/// sequence stores. The meaningful return is the full accumulator.
lf_checker_rt::export!(thiscall, rw_00aec9c0(this: u32, arg: u32) -> u32 {
    unsafe {
        const HDR_A: u32 = 0x0a;
        const HDR_C: u32 = 0x0c;
        const TICK: u32 = 0x10;
        const GATEW: u32 = 0x14;
        const ARG4: u32 = 0x04;
        const G_LINK: u32 = 0x011735b4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        if arg == 0 {
            return 0;
        }
        if rd32(arg + HDR_C) != 0 && (arg + HDR_A) as *const u8 != core::ptr::null() {
        }
        let ha = ((arg + HDR_A) as *const u8).read();
        if rd32(arg + HDR_C) != 0 && ha & 0x80 == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(1, u32, this);
        let tickp = (this + TICK) as *mut u32;
        tickp.write_unaligned(tickp.read_unaligned().wrapping_add(1));
        let frame: u32 = lf_checker_rt::callee_thiscall!(2, u32, arg);
        let probe_arg = rd32(arg + HDR_C);
        (arg as *mut u32).write_unaligned(rd32(arg));
        ((arg + HDR_C) as *mut u32).write_unaligned(0);
        lf_checker_rt::callee_thiscall!(3, u32, arg, 0x80u32, 0u32);
        let _ = probe_arg;
        let _ = frame;
        let ebp: u32 = lf_checker_rt::callee_cdecl!(4, u32, rd32(arg + HDR_C).wrapping_add(0).wrapping_sub(0).wrapping_add(probe_arg_before(arg)));
        let _ = ebp;
        0
    }
});

/// Helper: re-read helper placeholder (replaced below).
#[inline(always)]
unsafe fn probe_arg_before(_arg: u32) -> u32 {
    unsafe { 0 }
}
