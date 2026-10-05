// original: 0x00a8a8b0 pool_stage_at_offset48 (proposed)

/// Run the stage machine on the sub-object at +0x48 with one argument.
///
/// `this` is the pool object, `arg` the stage argument. Forwards both to
/// the stage callee and returns its low byte, as the original does.
///
/// Original: 0x00A8A8B0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00a8a8b0(this: u32, arg: u32) -> u32 {
    unsafe {
        const CALLEE_STAGE: u32 = 1;
        const SUB_OBJECT: u32 = 0x48;
        lf_checker_rt::callee_thiscall!(
            CALLEE_STAGE,
            u32,
            this,
            arg,
            this.wrapping_add(SUB_OBJECT)
        )
    }
});
