// original: 0x00CAA6C0 slot_probe_kind (proposed)

/// Return the kind of the inner record's first settled slot, or -1.
///
/// Resolves the first non-null slot of the sub-record at `this+0x20` (see
/// the slot helper). When every slot is null returns -1; otherwise calls
/// virtual slot 1 of the settled object with that object as `this` (a tail
/// call in the original) and returns its result.
///
/// Original: 0x00CAA6C0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00caa6c0(this: u32) -> u32 {
    unsafe {
        const RESOLVER: u32 = 1;
        const INNER: u32 = 0x20;
        const KIND_SLOT: u32 = 0x04;
        const NONE: u32 = 0xFFFF_FFFF;
        let obj = lf_checker_rt::callee_thiscall!(RESOLVER, u32, this.wrapping_add(INNER));
        if obj == 0 {
            return NONE;
        }
        let vtable = (obj as *const u32).read_unaligned();
        let slot = (vtable.wrapping_add(KIND_SLOT) as *const u32).read_unaligned();
        let kind: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        kind(obj)
    }
});
