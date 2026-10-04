// original: 0x00a00400 ALLOW_MULTIPLE_DRIVEBY_PICKUPS
/// Script native handler `ALLOW_MULTIPLE_DRIVEBY_PICKUPS` (hash 0x7FC02528).
///
/// Forwards script argument 0 to the engine worker and returns its answer.
/// Boolean arguments are coerced to 0/1; the original builds the pushed
/// word inside its own incoming stack slot, so the high bytes repeat the
/// context pointer and are reproduced from `ctx` here.
export!(cdecl, rw_00a00400(ctx: u32) -> u32 {
    unsafe {
        let args = *(ctx as *const u32).add(2) as *const u32;
        let q0 = (ctx & !0xFF) | u32::from(*args != 0);
        let answer: u32 = callee_cdecl!(1, u32, q0);
        answer
    }
});
