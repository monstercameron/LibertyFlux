// original: 0x00AF4700 drawable_release_slot (proposed)

/// Release the slot stored at `this+8` through the registry.
///
/// Calls the registry callee (object fixed by the original) with the word
/// at offset 8 of this object. The stack word is popped but never read.
/// Returns the callee's result.
///
/// Original: 0x00AF4700 (thiscall, one unread stack word, one direct callee).
lf_checker_rt::export!(thiscall, rw_00af4700(this: u32, _u: u32) -> u32 {
    unsafe {
        const REGISTRY_CALLEE: u32 = 1;
        const REGISTRY_OBJ: u32 = 0x01173750;
        const SLOT_OFF: u32 = 8;
        let slot = ((this.wrapping_add(SLOT_OFF)) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(REGISTRY_CALLEE, u32, lf_checker_rt::relocated(REGISTRY_OBJ), slot)
    }
});
