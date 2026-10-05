// original: 0x009B9AD0 CCamScriptInstruction_SequenceStopProcessing::vf2

/// Execute the SequenceStopProcessing script instruction: look the target object up through
/// the camera manager singleton (callee id 1, thiscall with the index at `this+0x08`), then update it; never null in practice; the contract always answers a live object.
///
/// No return value (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_009B9AD0(this: u32) -> u32 {
    unsafe {
        const MGR: u32 = 0x0128E400;
        const FIELD_INDEX: u32 = 0x08;
        const LOOKUP: u32 = 1;
        let index = ((this + FIELD_INDEX) as *const u32).read_unaligned();
        let obj: u32 = lf_checker_rt::callee_thiscall!(LOOKUP, u32,
            lf_checker_rt::relocated(MGR), index);
            ((obj + 0x80) as *mut u32).write_unaligned(1);
        0
    }
});
