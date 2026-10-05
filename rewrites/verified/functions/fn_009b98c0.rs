// original: 0x009B98C0 CCamScriptInstruction_InterpolateToScriptCam::vf2

/// Execute the InterpolateToScriptCam script instruction: look the target object up through
/// the global camera context (callee id 1, thiscall (no index argument)), then update it; never null in practice; the contract always answers a live object.
///
/// No return value (thiscall, `this` in ECX).
lf_checker_rt::export!(thiscall, rw_009B98C0(this: u32) -> u32 {
    unsafe {
        const MGR: u32 = 0x0103E498;
        const FIELD_WORD: u32 = 0x0C;
        const FIELD_BYTE: u32 = 0x08;
        const OBJ_WORD: u32 = 0x140;
        const OBJ_BYTE: u32 = 0x148;
        const LOOKUP: u32 = 1;
        let obj: u32 = lf_checker_rt::callee_thiscall!(LOOKUP, u32,
            lf_checker_rt::relocated(MGR));
        let word = ((this + FIELD_WORD) as *const u32).read_unaligned();
        ((obj + OBJ_WORD) as *mut u32).write_unaligned(word);
        let byte = ((this + FIELD_BYTE) as *const u8).read();
        ((obj + OBJ_BYTE) as *mut u8).write(byte);
        0
    }
});
