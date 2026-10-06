// original: 0x0087c400 crmt_orientation_init (proposed)
/// Initialise an orientation record from one input sample, in one of two
/// sparse layouts picked by a global's sign.
///
/// Reads the selector float (global file VA 0x018D2174, relocated, shared
/// by both sides): a strictly positive selector takes the positive layout,
/// anything else (zero, negative, NaN) the negative layout. Both layouts
/// feed the float at `src+0x04` through two SSE helpers (intercepted
/// callees 1 and 2: float in XMM0 via the stack transport, fresh float
/// answer in XMM0 read through the stub's eax bit-mirror) and store the two
/// answers plus a sign-flipped copy of the second (exact bit flip, matching
/// the original's xor-with-sign-mask lane for lane) into complementary
/// sparse slots with 0.0/1.0 sentinels; slots `+0x0c` and `+0x1c` are never
/// written. The sign mask the original xors with lives at file VA
/// 0x00FE8FA0; pefile's relocation walk shows no fixup for these two read
/// sites, but the worker resolves the read to the correct mask on every
/// trial (the negation-drop mutant is caught), so the worker's own walk
/// covers them. Computes no return value. The only comparison is the
/// ordered selector sign test.
///
/// Original: fastcall/0 (this in ecx, source in edx), four direct calls.
export!(fastcall, rw_0087c400(this: u32, src: u32) -> u32 {
    /// Branch selector global (file VA; relocated at load).
    const SEL_OFF: u32 = 0x018D2174;
    /// Input float offset in the source block.
    const IN_OFF: u32 = 0x04;
    /// Positive-one bits for the sentinel slots.
    const ONE: u32 = 0x3F800000;
    unsafe {
        let sel = *global::<f32>(SEL_OFF);
        let inp = ((src + IN_OFF) as *const u32).read_unaligned();
        // Exact sign flip, matching the original's xor with the 0x80000000
        // mask lane (the original xors four lanes but stores only the low
        // one, so the upper lanes are unobserved). Relocation note: pefile's
        // walk shows no fixup for the two mask reads, but the coordinator's
        // reloc_check (the worker's method) reports all 3 absolute operands
        // relocated, and the read resolves correctly on every trial.
        let neg = |b: u32| b ^ 0x80000000;
        if sel > 0.0 {
            let a = callee_cdecl!(1, u32, inp);
            let b = callee_cdecl!(2, u32, inp);
            ((this + 0x00) as *mut u32).write_unaligned(a);
            ((this + 0x04) as *mut u32).write_unaligned(0);
            ((this + 0x08) as *mut u32).write_unaligned(neg(b));
            ((this + 0x10) as *mut u32).write_unaligned(0);
            ((this + 0x14) as *mut u32).write_unaligned(ONE);
            ((this + 0x18) as *mut u32).write_unaligned(0);
            ((this + 0x20) as *mut u32).write_unaligned(b);
            ((this + 0x28) as *mut u32).write_unaligned(a);
            ((this + 0x24) as *mut u32).write_unaligned(0);
        } else {
            let a = callee_cdecl!(1, u32, inp);
            let b = callee_cdecl!(2, u32, inp);
            ((this + 0x00) as *mut u32).write_unaligned(a);
            ((this + 0x04) as *mut u32).write_unaligned(b);
            ((this + 0x08) as *mut u32).write_unaligned(0);
            ((this + 0x10) as *mut u32).write_unaligned(neg(b));
            ((this + 0x14) as *mut u32).write_unaligned(a);
            ((this + 0x18) as *mut u32).write_unaligned(0);
            ((this + 0x20) as *mut u32).write_unaligned(0);
            ((this + 0x28) as *mut u32).write_unaligned(ONE);
            ((this + 0x24) as *mut u32).write_unaligned(0);
        }
        0
    }
});
