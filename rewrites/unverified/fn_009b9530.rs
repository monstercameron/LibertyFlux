// original: 0x009B9530 CCamScriptInstruction_AddInterpCustomSpeedGraphMarker::vf2

/// Execute the AddInterpCustomSpeedGraphMarker script instruction:
/// pass the 32-bit float operand at `this+0x08` to the graph-marker adder
/// (callee 1), with the instruction object itself as `this`.
///
/// The operand travels bit-identical (the original reloads it through the
/// stack); no arithmetic is performed. No return value (thiscall).
lf_checker_rt::export!(thiscall, rw_009B9530(this: u32) -> u32 {
    unsafe {
        const FIELD_VALUE: u32 = 0x08;
        let bits = ((this + FIELD_VALUE) as *const u32).read_unaligned();
        let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, this, bits);
        0
    }
});
