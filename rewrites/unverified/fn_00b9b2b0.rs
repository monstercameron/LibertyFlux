// original: 0x00b9b2b0 NativeImpl_GENERATE_DIRECTIONS_2

/// Generates directions at a position, through two callee out-params.
///
/// Passes the input triple (`x`, `y`, `z`) plus two frame out-slots to
/// `GEN` on the global object `OBJ` as (input, out1, out2), then stores
/// out2's word through `slot_b` and out1's word through `slot_a` (note
/// the swapped copy order relative to the argument order). Either
/// out-pointer may be null, in which case that store faults exactly like
/// the original. Returns out1's word (the last value loaded, not the
/// callee's answer, which the loads clobber).
///
/// All three buffer pointers are skipped call arguments; the input triple
/// is snapshot-verified and the out-slots are written by the stub's
/// scripted words (see `narrowed`).
///
/// Original: 0x00B9B2B0 (cdecl, five stack words, returns u32 in eax).
lf_checker_rt::export!(cdecl, rw_00b9b2b0(x: u32, y: u32, z: u32, slot_a: u32, slot_b: u32) -> u32 {
    const OBJ: u32 = 0x01177A80;
    const GEN: u32 = 1;
    unsafe {
        let mut input = [x, y, z];
        let mut out1 = [0u32; 1];
        let mut out2 = [0u32; 1];
        let _: u32 = lf_checker_rt::callee_thiscall!(
            GEN,
            u32,
            lf_checker_rt::relocated(OBJ),
            input.as_mut_ptr() as u32,
            out1.as_mut_ptr() as u32,
            out2.as_mut_ptr() as u32
        );
        (slot_b as *mut u32).write_unaligned(out2[0]);
        (slot_a as *mut u32).write_unaligned(out1[0]);
        out1[0]
    }
});
