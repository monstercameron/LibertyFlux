// original: 0x008d88e0 file_stage_88e0
/// Run one fixed-this stage, then tail-call the next stage.
///
/// The original ends in a jump; the rewrite makes the same call normally,
/// which the checker observes identically.
export!(cdecl, rw_008d88e0() -> u32 {
    unsafe {
        /// Fixed `this` of the stage call (file VA).
        const FIRST_THIS: u32 = 0x0117374C;
        let _: u32 = callee_thiscall!(1, u32, relocated(FIRST_THIS));
        callee_cdecl!(2, u32,)
    }
});
