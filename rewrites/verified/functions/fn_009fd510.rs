// original: 0x009FD510 frag_pool_init (proposed)

/// Initialise a frag pool object for `count` twelve-byte elements.
///
/// Computes `count * 12`, saturating to all-bits-set on overflow (the
/// saturation itself is not drivable under the checker: any overflowing
/// count would also loop that many times against the stubs), obtains the
/// backing store from the allocator callee, links the object's inline heads
/// (`+8` to `+0x0c`, `+0x10` to self, `+0x20` to `+0x24`, `+0x28` to
/// `+0x18`, `+0x30` to the store), then initialises each element from last
/// to first through the element callee on the head at `+0x18`. Returns the
/// last element callee's answer, or `obj + 0x24` when `count` is zero (left
/// over from the address computation).
///
/// Original: 0x009FD510 (thiscall, `obj` in `ecx`, `count` one stack word).
lf_checker_rt::export!(thiscall, rw_009FD510(obj: u32, count: u32) -> u32 {
    unsafe {
        const ELEM: u32 = 12;
        const STORE_OFF: u32 = 0x30;
        const HEAD_OFF: u32 = 0x18;
        let size = count.checked_mul(ELEM).unwrap_or(0xFFFF_FFFF);
        let mem = lf_checker_rt::callee_cdecl!(1, u32, size);
        ((obj + STORE_OFF) as *mut u32).write_unaligned(mem);
        ((obj + 8) as *mut u32).write_unaligned(obj + 0x0C);
        ((obj + 0x10) as *mut u32).write_unaligned(obj);
        ((obj + 0x20) as *mut u32).write_unaligned(obj + 0x24);
        ((obj + 0x28) as *mut u32).write_unaligned(obj + HEAD_OFF);
        // No elements: the original returns obj+0x24 (left over from the
        // address computation above), not the store.
        let mut r = obj + 0x24;
        let mut i = count;
        while i > 0 {
            i -= 1;
            r = lf_checker_rt::callee_thiscall!(2, u32, obj + HEAD_OFF, mem + i * ELEM);
        }
        r
    }
});
