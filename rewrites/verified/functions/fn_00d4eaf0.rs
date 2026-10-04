// original: 0x00D4EAF0 CTaskComplexPickUpObject::dtor (proposed)

// Destructor of CTaskComplexPickUpObject: installs the class vtable pointer, then destroys the members.
///
/// Runs the member cleanup: when the word at `+0x14` is non-null a pointer
/// to that slot goes to intercepted callee 1 with the word in ECX, then the
/// sub-objects at `+0x3c` and `+0x30` are destroyed through intercepted
/// callees 2 and 3, in that order. Finally the base destructor
/// runs, reached by a tail jump in the original; its result is forwarded.
/// Callee return values are discarded.
///
/// Original: 0x00D4EAF0 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00d4eaf0(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00EE5004;
        const C1: u32 = 1;
        const C2: u32 = 2;
        const C3: u32 = 3;
        const BASE: u32 = 4;
        ((this) as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        let w14 = ((this + 0x14) as *const u32).read_unaligned();
        if w14 != 0 {
            lf_checker_rt::callee_thiscall!(C1, u32, w14, this.wrapping_add(0x14));
        }
        lf_checker_rt::callee_thiscall!(C2, u32, this.wrapping_add(0x3c));
        lf_checker_rt::callee_thiscall!(C3, u32, this.wrapping_add(0x30));
        lf_checker_rt::callee_thiscall!(BASE, u32, this)
    }
});
