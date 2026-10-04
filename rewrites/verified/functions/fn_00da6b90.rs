// original: 0x00DA6B90 task_vt_eefbcc_dtor (proposed)

/// Destructor of the task class whose virtual table is 0x00EEFBCC: install
/// the table, release each owned member slot (`+0x38`, then `+0x14`) when
/// set, clearing the slot after its release, then tail-jump to the
/// task-header destructor with `this` still in ECX.
///
/// The release calls take the slot addresses; each member word is only
/// tested. The tail jump is expressed as a call whose answer is returned.
/// Original: thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00da6b90(this: u32) -> u32 {
    unsafe {
        const MEMBER_RELEASE: u32 = 1;
        const HEADER_DTOR: u32 = 2;
        const VTABLE: u32 = 0x00EEFBCC;
        const FIRST_OFF: u32 = 0x38;
        const SECOND_OFF: u32 = 0x14;

        (this as *mut u32).write(lf_checker_rt::relocated(VTABLE));
        let first = (this + FIRST_OFF) as *mut u32;
        if first.read() != 0 {
            lf_checker_rt::callee_stdcall!(MEMBER_RELEASE, u32, this + FIRST_OFF);
            first.write(0);
        }
        let second = (this + SECOND_OFF) as *mut u32;
        if second.read() != 0 {
            lf_checker_rt::callee_stdcall!(MEMBER_RELEASE, u32, this + SECOND_OFF);
            second.write(0);
        }
        lf_checker_rt::callee_thiscall!(HEADER_DTOR, u32, this)
    }
});
