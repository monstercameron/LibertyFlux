// original: 0x00D4EAC0 CTaskComplexPickUpAndCarryObject::dtor (proposed)

// Destructor of CTaskComplexPickUpAndCarryObject: installs the class vtable pointer, then destroys the members.
///
/// Runs the member cleanup: when the word at `+0x14` is non-null a pointer
/// to that slot goes to intercepted callee 1 with the word in ECX, then the
/// sub-object at `+0x30` is destroyed through intercepted callee 2. Finally the base destructor
/// runs, reached by a tail jump in the original; its result is forwarded.
/// Callee return values are discarded.
///
/// Original: 0x00D4EAC0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00d4eac0(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00EE50B4;
        const C1: u32 = 1;
        const C2: u32 = 2;
        const BASE: u32 = 3;
        ((this) as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        let w14 = ((this + 0x14) as *const u32).read_unaligned();
        if w14 != 0 {
            lf_checker_rt::callee_thiscall!(C1, u32, w14, this.wrapping_add(0x14));
        }
        lf_checker_rt::callee_thiscall!(C2, u32, this.wrapping_add(0x30));
        lf_checker_rt::callee_thiscall!(BASE, u32, this)
    }
});
