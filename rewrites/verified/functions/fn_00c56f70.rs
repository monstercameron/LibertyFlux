// original: 0x00c56f70 task_check_slot_triple_nonzero (proposed)

/// Report whether one of an object's twelve-byte slots holds any nonzero word.
///
/// Slot `idx` starts at `this + idx*12 + 0xb34` and is read as three words;
/// the result is 1 when any of them is nonzero, else 0. Both paths run the
/// stack-cookie check (callee 1, which preserves registers) before returning.
///
/// Original: 0x00c56f70 (thiscall: `this` in ecx, one stack word).
lf_checker_rt::export!(thiscall, rw_00c56f70(this: u32, idx: u32) -> u32 {
    unsafe {
        const SLOT_BASE: u32 = 0xb34;
        const COOKIE_CHECK: u32 = 1;
        let base = this
            .wrapping_add(idx.wrapping_mul(3).wrapping_mul(4))
            .wrapping_add(SLOT_BASE);
        let w0 = (base as *const u32).read_unaligned();
        let w1 = ((base + 4) as *const u32).read_unaligned();
        let w2 = ((base + 8) as *const u32).read_unaligned();
        let r = u32::from(w0 != 0 || w1 != 0 || w2 != 0);
        lf_checker_rt::callee_cdecl!(COOKIE_CHECK, u32,);
        r
    }
});

