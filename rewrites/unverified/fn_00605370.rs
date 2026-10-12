// original: 0x00605370 rage::fragManagerBase::vf0

/// Deleting destructor for the fragment-manager base: reset the two
/// virtual tables, clear the registered-manager global, destroy the
/// embedded member, then free the object when the flag asks for it.
///
/// `this` in ECX, one stack argument holding the MSVC deleting-destructor
/// flag. The object header gets `VTAB_MAIN` and the member at +0x10 gets
/// `VTAB_MEMBER`; the global at `MANAGER_REG` (file VA 0x018B7448) is
/// cleared. The member destructor runs next (callee 1, thiscall with
/// `this + 4`, no stack arguments). The flag is tested only after that:
/// the header is always re-pointed at `VTAB_BASE` first, and only when
/// bit 0 of the flag is set is the object released through the allocator
/// reached via TLS slot 0 (its release entry is checker callee 2).
/// Returns `this` unchanged. Thiscall.
lf_checker_rt::export!(thiscall, rw_00605370(this: u32, flag: u32) -> u32 {
    unsafe {
        const MEMBER: u32 = 0x04;
        const MEMBER_VTAB_OFF: u32 = 0x10;
        const VTAB_MAIN: u32 = 0x00FE1894;
        const VTAB_MEMBER: u32 = 0x00FE1878;
        const VTAB_BASE: u32 = 0x00E86AFC;
        const MANAGER_REG: u32 = 0x018B7448;
        const MEMBER_CALLEE: u32 = 1;
        const ALLOC: u32 = 0x08;
        const FREE_SLOT: u32 = 0x0c;
        // The release entry is checker callee 2, reached through the planted vtable.

        (this as *mut u32).write(lf_checker_rt::relocated(VTAB_MAIN));
        lf_checker_rt::global::<u32>(MANAGER_REG).write(0);
        ((this + MEMBER_VTAB_OFF) as *mut u32).write(lf_checker_rt::relocated(VTAB_MEMBER));
        let _: u32 = lf_checker_rt::callee_thiscall!(MEMBER_CALLEE, u32, this.wrapping_add(MEMBER));
        (this as *mut u32).write(lf_checker_rt::relocated(VTAB_BASE));
        if flag & 1 != 0 {
            let state = lf_checker_rt::tls_slot(0);
            let alloc = ((state + ALLOC) as *const u32).read();
            let vtab = (alloc as *const u32).read();
            let free: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute((((vtab + FREE_SLOT) as *const u32).read()) as usize);
            let _: u32 = free(alloc, this);
        }
        this
    }
});
