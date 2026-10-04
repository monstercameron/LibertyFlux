// original: 0x00D4F2F0 task_fire_when_vector_set (proposed)

// Fires intercepted callee 1 with a pointer to the vector at `+0x20` when any
/// of its three floats is non-zero (a NaN counts as non-zero, matching the
/// original's unordered-compare dispatch; negative zero counts as zero).
/// Returns nothing. When all three are zero the original jumps to a shared
/// block outside this function, so that path is out of reach of the proof and
/// the contract always sets at least one component.
///
/// Original: 0x00D4F2F0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00d4f2f0(this: u32) -> u32 {
    unsafe {
        const C1: u32 = 1;
        let x = ((this + 0x20) as *const f32).read_unaligned();
        let y = ((this + 0x24) as *const f32).read_unaligned();
        let z = ((this + 0x28) as *const f32).read_unaligned();
        if x != 0.0 || y != 0.0 || z != 0.0 {
            lf_checker_rt::callee_cdecl!(C1, u32, this.wrapping_add(0x20));
        }
        0
    }
});
