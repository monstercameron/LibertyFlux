// original: 0x00c5cce0 CTaskSimpleBeHit_dtor (proposed)

/// Destructor body of the be-hit simple task.
///
/// Writes the class virtual table at `+0`. When the word at `+0x24` is
/// non-null it is passed in ecx to callee 2 with the constant -4.0 and then
/// to callee 3 with `this`, and the slot is cleared. The held reference at
/// `+0x14` is released through callee 1 when non-null. Finally the base task
/// destructor runs (callee 5, reached by a tail jump in the original).
/// Returns nothing.
///
/// Original: 0x00c5cce0 (thiscall: `this` in ecx, no stack words).
lf_checker_rt::export!(thiscall, rw_00c5cce0(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xecb224;
        const MEMBER: u32 = 0x14;
        const AUX: u32 = 0x24;
        const AUX_CONST: u32 = 0xc0800000; // -4.0
        const RELEASE: u32 = 1;
        const AUX_A: u32 = 2;
        const AUX_B: u32 = 3;
        const BASE_DTOR: u32 = 5;
        let aux = ((this + AUX) as *const u32).read_unaligned();
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        if aux != 0 {
            lf_checker_rt::callee_thiscall!(AUX_A, u32, aux, AUX_CONST);
            let aux2 = ((this + AUX) as *const u32).read_unaligned();
            lf_checker_rt::callee_thiscall!(AUX_B, u32, aux2, this);
            ((this + AUX) as *mut u32).write_unaligned(0);
        }
        let held = ((this + MEMBER) as *const u32).read_unaligned();
        if held != 0 {
            lf_checker_rt::callee_thiscall!(RELEASE, u32, held, this + MEMBER);
        }
        lf_checker_rt::callee_thiscall!(BASE_DTOR, u32, this);
        0
    }
});

