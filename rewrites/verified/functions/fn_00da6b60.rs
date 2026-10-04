// original: 0x00DA6B60 CTaskComplexFleeAndDive::dtor (proposed)

/// Destructor of the CTaskComplexFleeAndDive task: install this class's virtual table,
/// release the owned member at `+0x30` when it is set, then tail-jump
/// to the task-header destructor with `this` still in ECX.
///
/// The release call takes the member slot address; the member word itself is
/// only tested, never read further. The tail jump is expressed as a call
/// whose answer is returned. Original: thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00da6b60(this: u32) -> u32 {
    unsafe {
        const MEMBER_RELEASE: u32 = 1;
        const HEADER_DTOR: u32 = 2;
        const VTABLE: u32 = 0x00EEFB1C;
        const MEMBER_OFF: u32 = 0x30;

        let slot = (this + MEMBER_OFF) as *const u32;
        (this as *mut u32).write(lf_checker_rt::relocated(VTABLE));
        if slot.read() != 0 {
            lf_checker_rt::callee_stdcall!(MEMBER_RELEASE, u32, this + MEMBER_OFF);
        }
        lf_checker_rt::callee_thiscall!(HEADER_DTOR, u32, this)
    }
});
