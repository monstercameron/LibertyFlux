// original: 0x00a7ce10 find_or_fallback_b
// Find-or-fallback lookup (second site): same shape as the first one,
// run the direct search over (`a1`, `a2`) and fall back to the +0x114
// container with (`a1`, 0, `this`) when the search comes back empty.
export!(thiscall, rw_s13_00a7ce10(this: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        let search: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let found = search(this, a1, a2);
        if found != 0 {
            return found;
        }
        let container = *((this + 0x114) as *const u32);
        let fallback: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(2) as usize);
        fallback(container, a1, 0, this)
    }
});
