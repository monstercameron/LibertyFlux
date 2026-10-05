// original: 0x009B9980 CCamScriptInstruction_PropagateCam::vf2

/// Execute the PropagateCam script instruction: look the target object up through
/// the camera manager singleton (callee id 1, thiscall with the index at `this+0x08`), then update it; returns early when the lookup misses (null answer).
///
/// No return value (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_009B9980(this: u32) -> u32 {
    unsafe {
        // Bit 3 of the flags byte at OBJ_FLAGS is set to bit 0 of the
        // instruction operand: the original's xor/and/xor sequence with the
        // operand shifted left by 3.
        const MGR: u32 = 0x0128E400;
        const FIELD_OPERAND: u32 = 0x0C;
        const OBJ_FLAGS: u32 = 0x13C;
        const FLAG_BIT: u8 = 0x08;
        const LOOKUP: u32 = 1;
        let index = ((this + 0x08) as *const u32).read_unaligned();
        let obj: u32 = lf_checker_rt::callee_thiscall!(LOOKUP, u32,
            lf_checker_rt::relocated(MGR), index);
        if obj != 0 {
        let want = ((this + FIELD_OPERAND) as *const u8).read() & 1;
        let cell = (obj + OBJ_FLAGS) as *mut u8;
        cell.write((cell.read() & !FLAG_BIT) | (want << 3));
        }
        0
    }
});
