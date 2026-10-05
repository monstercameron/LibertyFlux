// original: 0x00b4eec0 clear_slot_118h (proposed)

/// Release the helper object kept in slot 0x118 and clear the slot.
///
/// `this + 0x118` holds a pointer (or null). When non-null it is released
/// through callee 1 (thiscall on the object, passed the address of the
/// slot) and the slot is cleared. A null slot does nothing. No meaningful
/// return value.
///
/// Original: 0x00b4eec0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00b4eec0(this: u32) -> u32 {
    unsafe {
        const SLOT: u32 = 0x118;
        const RELEASE: u32 = 1;
        let slot = this + SLOT;
        let obj = (slot as *const u32).read_unaligned();
        if obj != 0 {
            lf_checker_rt::callee_thiscall!(RELEASE, u32, obj, slot);
            (slot as *mut u32).write_unaligned(0);
        }
        0
    }
});
