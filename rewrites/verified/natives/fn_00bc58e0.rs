// original: 0x00bc58e0 FREEZE_CAR_POSITION
/// Script native handler `FREEZE_CAR_POSITION` (hash 0x295C4C52).
///
/// Forwards script arguments 0..1 to the engine worker and returns its answer.
/// Boolean arguments are coerced to 0/1; the original builds the pushed
/// word inside its own incoming stack slot, so the high bytes repeat the
/// context pointer and are reproduced from `ctx` here.
export!(cdecl, rw_00bc58e0(ctx: u32) -> u32 {
    unsafe {
        let args = *(ctx as *const u32).add(2) as *const u32;
        let a0 = *args.add(0);
        let a1 = *args.add(1);
        let q1 = (ctx & !0xFF) | u32::from(a1 != 0);
        let answer: u32 = callee_cdecl!(1, u32, a0, q1);
        answer
    }
});
