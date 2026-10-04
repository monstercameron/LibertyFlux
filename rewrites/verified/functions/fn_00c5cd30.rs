// original: 0x00c5cd30 CTaskSimpleNewGangDriveBy_dtor (proposed)

/// Destructor body of the new-gang-driveby simple task.
///
/// Writes the class virtual table at `+0`, releases the held reference at
/// `+0x20` through callee 1 when non-null, then when the word at `+0x50` is
/// non-null passes it in ecx to callee 2 with the constant -4.0 and to callee
/// 3 with `this`, clearing the slot. Destroys the embedded member at `+0x14`
/// (callee 4) and runs the base task destructor (callee 5, reached by a tail
/// jump in the original). Returns nothing.
///
/// Original: 0x00c5cd30 (thiscall: `this` in ecx, no stack words).
lf_checker_rt::export!(thiscall, rw_00c5cd30(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0xecb434;
        const MEMBER: u32 = 0x20;
        const AUX: u32 = 0x50;
        const SUB: u32 = 0x14;
        const AUX_CONST: u32 = 0xc0800000; // -4.0
        const RELEASE: u32 = 1;
        const AUX_A: u32 = 2;
        const AUX_B: u32 = 3;
        const SUB_DTOR: u32 = 4;
        const BASE_DTOR: u32 = 5;
        let held = ((this + MEMBER) as *const u32).read_unaligned();
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        if held != 0 {
            lf_checker_rt::callee_thiscall!(RELEASE, u32, held, this + MEMBER);
        }
        let aux = ((this + AUX) as *const u32).read_unaligned();
        if aux != 0 {
            lf_checker_rt::callee_thiscall!(AUX_A, u32, aux, AUX_CONST);
            let aux2 = ((this + AUX) as *const u32).read_unaligned();
            lf_checker_rt::callee_thiscall!(AUX_B, u32, aux2, this);
            ((this + AUX) as *mut u32).write_unaligned(0);
        }
        lf_checker_rt::callee_thiscall!(SUB_DTOR, u32, this + SUB);
        lf_checker_rt::callee_thiscall!(BASE_DTOR, u32, this);
        0
    }
});

