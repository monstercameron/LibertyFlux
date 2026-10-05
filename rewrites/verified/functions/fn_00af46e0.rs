// original: 0x00AF46E0 drawable_bind_slot (proposed)

/// Resolve a slot id through the registry and store it at `this+8`.
///
/// Calls the registry callee (object fixed by the original, slot id on the
/// stack) and writes the result to offset 8 of this object. Returns the
/// callee's result.
///
/// Original: 0x00AF46E0 (thiscall, one stack word, one direct callee).
lf_checker_rt::export!(thiscall, rw_00af46e0(this: u32, slot: u32) -> u32 {
    unsafe {
        const REGISTRY_CALLEE: u32 = 1;
        const REGISTRY_OBJ: u32 = 0x01173750;
        const SLOT_OFF: u32 = 8;
        let r = lf_checker_rt::callee_thiscall!(REGISTRY_CALLEE, u32, lf_checker_rt::relocated(REGISTRY_OBJ), slot);
        ((this.wrapping_add(SLOT_OFF)) as *mut u32).write_unaligned(r);
        r
    }
});
