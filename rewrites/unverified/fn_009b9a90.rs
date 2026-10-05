// original: 0x009B9A90 CCamScriptInstruction_SequenceEmpty::vf2

/// Execute the SequenceEmpty script instruction: look the sequence object
/// up by the index at `this+0x08` (callee 1), then tail-jump into its
/// emptiness check (callee 2) with the looked-up object as `this`.
///
/// The rewrite issues the tail jump as a normal call. No return value
/// (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_009B9A90(this: u32) -> u32 {
    unsafe {
        const MGR: u32 = 0x0128_E400;
        const FIELD_INDEX: u32 = 0x08;
        let index = ((this + FIELD_INDEX) as *const u32).read_unaligned();
        let obj: u32 = lf_checker_rt::callee_thiscall!(1, u32,
            lf_checker_rt::relocated(MGR), index);
        let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, obj);
        0
    }
});
