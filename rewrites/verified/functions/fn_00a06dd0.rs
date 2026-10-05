// original: 0x00a06dd0 NativeImpl_SET_OBJECT_PROOFS (native)
/// Pack five proof flags into an object's proof field.
///
/// Resolves `handle` through the object pool, then packs the low bits of
/// the five proof words into bits 6..12 of the dword at object+0x118,
/// keeping the masked-off old bits. The packing order from the top bit is
/// proof4, proof6, proof5, proof3, proof2, matching the original's
/// shift-and-or chain exactly. Returns the masked old field. Cdecl.
lf_checker_rt::export!(cdecl, rw_00a06dd0(handle: u32, proof2: u32, proof3: u32,
                                          proof4: u32, proof5: u32, proof6: u32) -> u32 {
    unsafe {
        const POOL_GLOBAL: u32 = 0x01632c60;
        const POOL_LOOKUP: u32 = 0;
        const PROOF_FIELD: u32 = 0x118;
        const KEEP_MASK: u32 = 0xffff_ec3f;
        const FIELD_SHIFT: u32 = 6;
        let pool = (lf_checker_rt::global::<u32>(POOL_GLOBAL) as *const u32).read_unaligned();
        let obj = lf_checker_rt::callee_thiscall!(POOL_LOOKUP, u32, pool, handle);
        let mut packed = (proof4 & 1) << 3 | (proof6 & 1);
        packed = (packed << 1) | (proof5 & 1);
        packed = (packed << 1) | (proof3 & 1);
        packed = (packed << 1) | (proof2 & 1);
        let slot = (obj + PROOF_FIELD) as *mut u32;
        let old = slot.read_unaligned() & KEEP_MASK;
        slot.write_unaligned(old | (packed << FIELD_SHIFT));
        old
    }
});
