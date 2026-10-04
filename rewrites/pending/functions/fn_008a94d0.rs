// original: 0x008a94d0 audeffect_cycle_index
/// Cycle the table index: `(count + 2) % 3` into `this+0x2c`.
///
/// `count` is the dword at `this+0x30`; the addition wraps. Returns the
/// division quotient (matching exit EAX).
export!(thiscall, rw_008a94d0(this: *mut u8) -> u32 {
    unsafe {
        let dividend = (*(this.add(0x30) as *const u32)).wrapping_add(2);
        *(this.add(0x2c) as *mut u32) = dividend % 3;
        dividend / 3
    }
});
