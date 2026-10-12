// original: 0x00886F90 xml_read_vector3_from_context (proposed)

/// Read a 3-float vector through this context's XML reader.
///
/// Forwards to the vector reader (patched callee) with the same `this`, the
/// reader object at `this + 4`, and the two stack words `a0`, `a1` unchanged.
/// Returns the callee's result.
///
/// Original: 0x00886F90 (thiscall: `this` in ecx, two stack words).
lf_checker_rt::export!(thiscall, rw_00886f90(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        const VECTOR_READER: u32 = 1;
        const READER_OFF: u32 = 4;

        let reader = ((this + READER_OFF) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(VECTOR_READER, u32, this, reader, a0, a1)
    }
});
