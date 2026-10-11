// original: 0x00D30DB0 CTaskComplexMoveCrowdAroundLocation::vf10

/// Store the raw bits of the one stack-passed scalar value in the task field
/// at offset 0x3c. The receiver arrives in ECX, the value occupies one
/// 32-bit stack slot, and the callee removes that slot. The scalar is copied
/// without arithmetic, preserving every input bit; there is no observed
/// return value.
lf_checker_rt::export!(thiscall, rw_00D30DB0(this: u32, value_bits: u32) -> () {
    const CURRENT_VALUE: u32 = 0x3c;
    unsafe { crate::write_u32(this.wrapping_add(CURRENT_VALUE), value_bits); }
});
