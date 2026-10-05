// original: 0x009B9500 CCamScriptInstruction_ActivateViewport::vf2

/// Execute the ActivateViewport script instruction: look the target object up through
/// the camera manager singleton (callee id 1, thiscall with the index at `this+0x08`), then update it; never null in practice; the contract always answers a live object.
///
/// No return value (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_009B9500(this: u32) -> u32 {
    unsafe {
        // Bit 0 of the flags byte at OBJ_FLAGS is set to bit 0 of the
        // instruction operand: the original's xor/and/xor sequence with the
        // operand nothing; bit 0 is used directly.
        const MGR: u32 = 0x0128E400;
        const FIELD_OPERAND: u32 = 0x0C;
        const OBJ_FLAGS: u32 = 0x558;
        const FLAG_BIT: u8 = 0x01;
        const LOOKUP: u32 = 1;
        let index = ((this + 0x08) as *const u32).read_unaligned();
        let obj: u32 = lf_checker_rt::callee_thiscall!(LOOKUP, u32,
            lf_checker_rt::relocated(MGR), index);
        let want = ((this + FIELD_OPERAND) as *const u8).read() & 1;
        let cell = (obj + OBJ_FLAGS) as *mut u8;
        cell.write((cell.read() & !FLAG_BIT) | (want << 0));
        0
    }
});
