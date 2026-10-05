// original: 0x00a34fe0 vehicle_feature_gate (proposed)

/// Three nested global on/off gates deciding one byte result.
///
/// If the first gate byte is zero the answer is 1. Otherwise, if the second
/// gate byte is non-zero the answer is 0. Otherwise the answer is 1 exactly
/// when the third gate byte is zero. Cdecl, no arguments, returns AL.
lf_checker_rt::export!(cdecl, rw_00a34fe0() -> u32 {
    unsafe {
        const GATE_A: u32 = 0x012D_FD7D;
        const GATE_B: u32 = 0x0166_9D59;
        const GATE_C: u32 = 0x012B_9C88;
        // The comparison only looks at AL; return the byte in the low lane.
        let a = core::ptr::read(lf_checker_rt::global::<u8>(GATE_A));
        if a == 0 {
            return 1;
        }
        let b = core::ptr::read(lf_checker_rt::global::<u8>(GATE_B));
        if b != 0 {
            return 0;
        }
        let c = core::ptr::read(lf_checker_rt::global::<u8>(GATE_C));
        (c == 0) as u32
    }
});
