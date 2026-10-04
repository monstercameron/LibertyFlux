// original: 0x00D4EB80 CTaskSimplePutDownObject::dtor (proposed)

// Destructor of CTaskSimplePutDownObject: installs the class vtable pointer, then destroys the members.
///
/// Runs the member cleanup: when the word at `+0x14` is non-null a pointer
/// to that slot goes to intercepted callee 1 with the word in ECX. When the
/// word at `+0x1c` is non-null it is passed in ECX with the float -4.0 to
/// intercepted callee 2, then again in ECX with the object pointer to
/// intercepted callee 3, and the slot is cleared. Finally the sub-object
/// at `+0x20` is destroyed through intercepted callee 4. Finally the base destructor
/// runs, reached by a tail jump in the original; its result is forwarded.
/// Callee return values are discarded.
///
/// Original: 0x00D4EB80 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00d4eb80(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00EE505C;
        const C1: u32 = 1;
        const C2: u32 = 2;
        const C3: u32 = 3;
        const C4: u32 = 4;
        const NEG_FOUR_BITS: u32 = 0xC0800000; // -4.0f
        const BASE: u32 = 5;
        ((this) as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        let w14 = ((this + 0x14) as *const u32).read_unaligned();
        if w14 != 0 {
            lf_checker_rt::callee_thiscall!(C1, u32, w14, this.wrapping_add(0x14));
        }
        let w1c = ((this + 0x1c) as *const u32).read_unaligned();
        if w1c != 0 {
            lf_checker_rt::callee_thiscall!(C2, u32, w1c, NEG_FOUR_BITS);
            let w1c2 = ((this + 0x1c) as *const u32).read_unaligned();
            lf_checker_rt::callee_thiscall!(C3, u32, w1c2, this);
            ((this + 0x1c) as *mut u32).write_unaligned(0);
        }
        lf_checker_rt::callee_thiscall!(C4, u32, this.wrapping_add(0x20));
        lf_checker_rt::callee_thiscall!(BASE, u32, this)
    }
});
