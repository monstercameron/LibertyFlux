// original: 0x00D4DD10 CTaskSimpleShakeFist::dtor (proposed)

// Destructor of CTaskSimpleShakeFist: installs the class vtable pointer, then destroys the members.
///
/// Runs the member cleanup: when the word at `+0x18` is non-null it is
/// passed in ECX with the object pointer to intercepted callee 1, then the
/// slot is cleared; when the word at `+0x1c` is non-null a pointer to that
/// slot goes to intercepted callee 2 with the word in ECX. Finally the base destructor
/// runs, reached by a tail jump in the original; its result is forwarded.
/// Callee return values are discarded.
///
/// Original: 0x00D4DD10 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00d4dd10(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00EE4C8C;
        const C1: u32 = 1;
        const C2: u32 = 2;
        const BASE: u32 = 3;
        ((this) as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        let w18 = ((this + 0x18) as *const u32).read_unaligned();
        if w18 != 0 {
            lf_checker_rt::callee_thiscall!(C1, u32, w18, this);
            ((this + 0x18) as *mut u32).write_unaligned(0);
        }
        let w1c = ((this + 0x1c) as *const u32).read_unaligned();
        if w1c != 0 {
            lf_checker_rt::callee_thiscall!(C2, u32, w1c, this.wrapping_add(0x1c));
        }
        lf_checker_rt::callee_thiscall!(BASE, u32, this)
    }
});
