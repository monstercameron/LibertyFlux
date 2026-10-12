// original: 0x00b5c1e0 CEventSoundDynamic::vf0

/// Deleting destructor for a dynamic sound event: the same reset as the
/// sibling (child slot, handle, base step through scripted callees 1-3),
/// then, when bit 0 of the flag word is set, release the object through the
/// pool in the global at 0x016669c8 (scripted callee 4, thiscall).
/// Returns `this`.
lf_checker_rt::export!(thiscall, rw_00b5c1e0(this: u32, flag: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00eb_1114;
        const CHILD: u32 = 0x30;
        const HANDLE: u32 = 0x3c;
        const POOL: u32 = 0x0166_69c8;
        let child = (this.wrapping_add(CHILD) as *const u32).read_unaligned();
        (this as *mut u32).write_unaligned(VTABLE);
        if child != 0 {
            lf_checker_rt::callee_thiscall!(1, u32, child, this.wrapping_add(CHILD));
            (this.wrapping_add(CHILD) as *mut u32).write_unaligned(0);
        }
        let handle = (this.wrapping_add(HANDLE) as *const u32).read_unaligned();
        if handle != 0 {
            lf_checker_rt::callee_cdecl!(2, u32, handle);
            (this.wrapping_add(HANDLE) as *mut u32).write_unaligned(0);
        }
        lf_checker_rt::callee_thiscall!(3, u32, this);
        if flag & 1 != 0 {
            let pool = lf_checker_rt::global::<u32>(POOL).read_unaligned();
            lf_checker_rt::callee_thiscall!(4, u32, pool, this);
        }
        this
    }
});
