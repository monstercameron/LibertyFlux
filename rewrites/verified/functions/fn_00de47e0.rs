// original: 0x00DE47E0 UIGTAScrollingRow::UIGTAScrollingRow_2

/// Install the object's vtable pointer and clear the byte at `+0xC7`, then forward the object to the tail callee.
///
/// The vtable address is derived from the relocated original image. The caller-visible state is the vtable word and one cleared lifecycle byte; the remaining control flow is a tail call. `thiscall`; no stack arguments are consumed here.
lf_checker_rt::export!(thiscall, rw_00de47e0(this_ptr: u32) -> u32 {
    const VTABLE_FILE_VA: u32 = 0x00EFE4BC;
    const LIFECYCLE_FLAG: u32 = 0xC7;
    unsafe {
        (this_ptr as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE_FILE_VA));
        ((this_ptr.wrapping_add(LIFECYCLE_FLAG)) as *mut u8).write(0);
    }
    lf_checker_rt::callee_thiscall!(1, u32, this_ptr)
});
