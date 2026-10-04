// original: 0x00cf41b0 CTaskSimpleClimb::dtor (proposed)

/// Destructor body for a simple climb task: stamps the vtable, releases the
/// reference-counted slot at `+0x64` when occupied (callee takes the occupant
/// in ecx and the slot address on the stack), then tail-calls the shared base
/// destructor with the object.
///
/// Original: 0x00cf41b0 (thiscall: ecx holds the object, no stack words).
lf_checker_rt::export!(thiscall, rw_00cf41b0(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00edf154;
        const SLOT_OFFSET: u32 = 0x64;
        const RELEASE_CALLEE: u32 = 1;
        const BASE_DTOR: u32 = 2;
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        let slot = this.wrapping_add(SLOT_OFFSET);
        let old = (slot as *const u32).read_unaligned();
        if old != 0 {
            lf_checker_rt::callee_thiscall!(RELEASE_CALLEE, u32, old, slot);
        }
        lf_checker_rt::callee_thiscall!(BASE_DTOR, u32, this)
    }
});
