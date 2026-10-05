// original: 0x00c80ca0 scenario_slot_bind (proposed) — UNVERIFIED (deferred)

// NOTE: deferred with reason `frame_pointer_args` (plus encrypted bytes): the
// function passes pointers to its own aligned stack frame to two callees.
// Kept for a future checker or re-run; never passed, not verified.

/// Bind record `a0` to slot `a2`, resolving -1 through the slot allocator.
///
/// `c1([a0+0x20]+0x30)` failing (zero low byte) proceeds; a non-zero low byte
/// returns 0. Slot -1 resolves via `c2(a1, a0)`, whose -1 also returns 0. Two
/// scratch words on the stack gather `c3`'s outputs, then `c4` consumes them
/// with nine arguments and its result is returned.
///
/// Original: cdecl, three stack words (plain `ret`).
lf_checker_rt::export!(cdecl, rw_00c80ca0(a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const UNRESOLVED: u32 = 0xffff_ffff;
        const C1: u32 = 1;
        const C2: u32 = 2;
        const C3: u32 = 3;
        const C4: u32 = 4;
        let key = ((a0 + 0x20) as *const u32).read_unaligned().wrapping_add(0x30);
        let placed: u32 = lf_checker_rt::callee_cdecl!(C1, u32, key);
        if (placed & 0xff) != 0 {
            return 0;
        }
        let mut id = a2;
        if id == UNRESOLVED {
            id = lf_checker_rt::callee_cdecl!(C2, u32, a1, a0);
            if id == UNRESOLVED {
                return 0;
            }
        }
        let mut out_a = [0u32; 4];
        let mut out_b = [0u32; 4];
        let mid: u32 = lf_checker_rt::callee_cdecl!(
            C3, u32, id, out_b.as_mut_ptr() as u32, out_a.as_mut_ptr() as u32, a0);
        lf_checker_rt::callee_cdecl!(C4, u32, id, 0u32, mid, out_b.as_mut_ptr() as u32, 0u32,
            1u32, out_a.as_mut_ptr() as u32, a0, 0u32)
    }
});
