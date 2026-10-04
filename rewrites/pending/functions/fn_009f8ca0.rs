// original: 0x009f8ca0 selector_notify_dispatch
/// Bits of 1.0f, pushed as the float argument to the notify calls.
const ONE_BITS: u32 = 0x3F800000;

/// Tunable selector dispatch (cdecl/1 -> eax).
///
/// Fixed selectors map to fixed notify ids; otherwise a lookup result selects
/// the notify id through a jump table, with two selectors returning a count.
export!(cdecl, rw_s18f8(sel: u32) -> u32 {
    unsafe {
        // Each intercepted call is made through the immediate-call macro so
        // no callee address is held across branches.
        if sel == 0x33 {
            return callee_cdecl!(2, u32, 0x139, ONE_BITS);
        }
        if sel == 0x32 || sel == 0x31 {
            return callee_cdecl!(2, u32, 0x13A, ONE_BITS);
        }
        let base = callee_cdecl!(1, u32, sel);
        let slot = *(base.wrapping_add(12) as *const u32);
        let v = slot.wrapping_sub(1);
        // Jump-table order read from the binary: 0->0x137, 1->0x138,
        // 2->0x139, 3 returns, 4->0x13A, above 4 returns.
        match v {
            0 => callee_cdecl!(2, u32, 0x137, ONE_BITS),
            1 => callee_cdecl!(2, u32, 0x138, ONE_BITS),
            2 => callee_cdecl!(2, u32, 0x139, ONE_BITS),
            3 => 3,
            4 => callee_cdecl!(2, u32, 0x13A, ONE_BITS),
            _ => v,
        }
    }
});
