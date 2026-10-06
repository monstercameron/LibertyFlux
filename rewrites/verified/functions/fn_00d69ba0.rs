// original: 0x00d69ba0 CReplayWidget::vf0
/// Deleting destructor slot of the replay widget (original 0x00D69BA0,
/// thiscall/1).
///
/// Stamps the widget vtable (file VA 0x00EEA88C, passed relocated), then
/// frees the object through callee 1 when the low bit of `flags` is set.
/// Returns the object pointer.
lf_checker_rt::export!(thiscall, rw_00d69ba0(this_ptr: u32, flags: u32) -> u32 {
    unsafe {
        const VTABLE_WIDGET: u32 = 0x00EEA88C;
        (this_ptr as *mut u32)
            .write_unaligned(lf_checker_rt::relocated(VTABLE_WIDGET));
        if (flags & 1) != 0 {
            lf_checker_rt::callee_cdecl!(1, u32, this_ptr);
        }
        this_ptr
    }
});
