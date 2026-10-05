// original: 0x009B9800 CCamScriptInstruction_ForceTelescopeCam::vf2

/// Execute the ForceTelescopeCam script instruction: look the target object up through
/// the global camera context (callee id 1, thiscall (no index argument)), then update it; never null in practice; the contract always answers a live object.
///
/// No return value (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_009B9800(this: u32) -> u32 {
    unsafe {
        // Bit 4 of the flags byte at OBJ_FLAGS is set to bit 0 of the
        // instruction operand: the original's xor/and/xor sequence with the
        // operand shifted left by 4.
        const MGR: u32 = 0x0103E498;
        const FIELD_OPERAND: u32 = 0x08;
        const OBJ_FLAGS: u32 = 0x1C4;
        const FLAG_BIT: u8 = 0x10;
        const LOOKUP: u32 = 1;
        let obj: u32 = lf_checker_rt::callee_thiscall!(LOOKUP, u32,
            lf_checker_rt::relocated(MGR));
        let want = ((this + FIELD_OPERAND) as *const u8).read() & 1;
        let cell = (obj + OBJ_FLAGS) as *mut u8;
        cell.write((cell.read() & !FLAG_BIT) | (want << 4));
        0
    }
});
