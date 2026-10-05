// original: 0x009BA140 CCamScriptInstruction_SetFOV::vf2

/// Execute the SetFOV script instruction: look the target object up through
/// the camera manager singleton (callee id 1, thiscall with the index at `this+0x08`), then update it; returns early when the lookup misses (null answer).
///
/// No return value (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_009BA140(this: u32) -> u32 {
    unsafe {
        const MGR: u32 = 0x0128E400;
        const FIELD_INDEX: u32 = 0x08;
        const FIELD_VALUE: u32 = 0x0C;
        const OBJ_SLOT: u32 = 0x60;
        const LOOKUP: u32 = 1;
        let index = ((this + FIELD_INDEX) as *const u32).read_unaligned();
        let obj: u32 = lf_checker_rt::callee_thiscall!(LOOKUP, u32,
            lf_checker_rt::relocated(MGR), index);
        if obj != 0 {
        let value = ((this + FIELD_VALUE) as *const u32).read_unaligned();
        ((obj + OBJ_SLOT) as *mut u32).write_unaligned(value);
        }
        0
    }
});
