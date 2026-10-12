// original: 0x00b5c1a0 sound_event_reset_tail (proposed)

/// Reset a dynamic sound event: install the table; when the child slot at
/// `+0x30` is non-null release it (scripted callee 1, thiscall with the old
/// value) and clear the slot; when the handle at `+0x3c` is non-null close
/// it (scripted callee 2, cdecl) and clear it; then tail-call the base step
/// (scripted callee 3) and return its result.
lf_checker_rt::export!(thiscall, rw_00b5c1a0(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00eb_1114;
        const CHILD: u32 = 0x30;
        const HANDLE: u32 = 0x3c;
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
        lf_checker_rt::callee_thiscall!(3, u32, this)
    }
});
