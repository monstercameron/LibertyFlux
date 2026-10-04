// original: 0x00d39010 conditional_key_forward
/// Forward the key to the registry helper only when the slot already holds it.
export!(cdecl, rw_00d39010(a1: u32, a2: u32) -> u32 {
    if unsafe { *(a2 as *const u32) } != a1 {
        return a1;
    }
    callee_thiscall!(1, u32, a2, a1)
});
