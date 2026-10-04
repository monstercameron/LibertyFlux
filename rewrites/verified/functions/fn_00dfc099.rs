// original: 0x00dfc099 rand
/// MSVC `rand`: advance the per-thread seed and return bits 16..30.
///
/// The seed lives in the caller's thread-context block (handed back by the
/// intercepted helper) at offset `0x14`. Multiplier `0x343FD`, increment
/// `0x269EC3`, all wrapping.
export!(cdecl, rw_00dfc099() -> u32 {
    unsafe {
        let ctx = callee_cdecl!(1, u32,);
        let slot = (ctx.wrapping_add(0x14)) as *mut u32;
        let next = (*slot).wrapping_mul(0x343FD).wrapping_add(0x269EC3);
        *slot = next;
        (next >> 16) & 0x7FFF
    }
});
