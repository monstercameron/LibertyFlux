// original: 0x00a3a950 vehicle_pair_test (proposed)

/// Decide whether a vehicle pair passes the fast path or needs the slow test.
///
/// Returns 0 only when every fast check passes: `a1` is non-null, its
/// `+0x28` word has exactly bits `0xc0` of `0x3c0`, its byte at `+0x219`
/// is non-zero, and the selected key (its `+0x1bc` word when the low
/// nibble of its `+0x1e2` byte is at least 2, else 0) equals `obj[0]` --
/// and the kind lookup (id 1, cdecl/1) on `a3` yields a record whose
/// `+0xc` word is neither 3 nor 4. Any failure falls through to the slow
/// test (id 2, thiscall/3) on `(obj[0], a1, a3, a4), whose non-zero answer
/// becomes 1. Thiscall/4 (the second word is unread), returns AL.
lf_checker_rt::export!(thiscall, rw_00a3a950(obj: u32, a1: u32, _a2: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        const KIND_MASK: u32 = 0x3C0;
        const KIND_WANT: u32 = 0xC0;
        const KIND_LOOKUP: u32 = 1;
        const SLOW_TEST: u32 = 2;
        let slow = if a1 == 0 {
            true
        } else if core::ptr::read_unaligned((a1 + 0x28) as *const u32) & KIND_MASK != KIND_WANT {
            true
        } else if core::ptr::read((a1 + 0x219) as *const u8) == 0 {
            true
        } else {
            let nib = core::ptr::read((a1 + 0x1E2) as *const u8) & 0xF;
            let key = if nib >= 2 {
                core::ptr::read_unaligned((a1 + 0x1BC) as *const u32)
            } else {
                0
            };
            if key != core::ptr::read_unaligned(obj as *const u32) {
                true
            } else {
                let rec: u32 = lf_checker_rt::callee_cdecl!(KIND_LOOKUP, u32, a3);
                let v = core::ptr::read_unaligned((rec + 0xC) as *const u32);
                v == 3 || v == 4
            }
        };
        if !slow {
            return 0;
        }
        let this = core::ptr::read_unaligned(obj as *const u32);
        // The original tests only the low byte of the answer.
        (lf_checker_rt::callee_thiscall!(SLOW_TEST, u32, this, a1, a3, a4) & 0xFF != 0) as u32
    }
});
