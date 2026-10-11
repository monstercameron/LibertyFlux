// original: 0x00D30C10 CTaskComplexMoveFollowNavMeshRoute::vf11

/// Store the raw bits of the one stack-passed scalar value in the task field
/// at offset 0x40. The receiver arrives in ECX, the value occupies one
/// 32-bit stack slot, and the callee removes that slot. The original performs
/// a scalar load and store, so NaN payloads and signed zero pass through
/// unchanged; it has no observed return value.
lf_checker_rt::export!(thiscall, rw_00D30C10(this: u32, value_bits: u32) -> () {
    const CURRENT_VALUE: u32 = 0x40;
    unsafe { crate::write_u32(this.wrapping_add(CURRENT_VALUE), value_bits); }
});
