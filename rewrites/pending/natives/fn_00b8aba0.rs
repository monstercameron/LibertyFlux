// original: 0x00b8aba0 IS_POS_IN_CUTSCENE_BLOCKING_BOUNDS
/// Script native handler `IS_POS_IN_CUTSCENE_BLOCKING_BOUNDS` (hash 0x593A553B).
///
/// Forwards script arguments 0..2 to the engine worker and stores the low byte of its answer (zero-extended) into the return slot.
/// Floating-point arguments are forwarded as raw bits, so they match bit-exactly.
export!(cdecl, rw_00b8aba0(ctx: u32) -> u32 {
    unsafe {
        let args = *(ctx as *const u32).add(2) as *const u32;
        let a0 = *args.add(0);
        let a1 = *args.add(1);
        let a2 = *args.add(2);
        let answer: u32 = callee_cdecl!(1, u32, a0, a1, a2);
        let slot = *(ctx as *const u32) as *mut u32;
        *slot = answer & 0xFF;
        slot as u32
    }
});
