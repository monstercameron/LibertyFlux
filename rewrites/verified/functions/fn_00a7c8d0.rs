// original: 0x00a7c8d0 find_or_fallback_a
// Find-or-fallback lookup: run the direct search over (`a1`, `a2`) and
// return its result when nonzero; otherwise retry through the container
// at +0x114 with (`a1`, 0, `this`) and return that answer.
export!(thiscall, rw_s13_00a7c8d0(this: u32, a1: u32, a2: u32) -> u32 {
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
