// original: 0x00A6D190 player_settings_task_set_vtable_pair

/// Installs the task's primary and embedded vtable pointers, then tail-calls
/// its base destructor.
///
/// The two relocated addresses are written at offsets `0` and `0x14`. The
/// tail-called thiscall receives `this` and provides the returned EAX value.
lf_checker_rt::export!(thiscall, rw_00a6d190(this: u32) -> u32 {
    unsafe {
        const BASE_DESTRUCTOR: u32 = 1;
        const PRIMARY_VTABLE: u32 = 0x00E9_EF7C;
        const EMBEDDED_VTABLE: u32 = 0x00E9_EFD0;

        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(PRIMARY_VTABLE));
        ((this + 0x14) as *mut u32).write_unaligned(lf_checker_rt::relocated(EMBEDDED_VTABLE));
        lf_checker_rt::callee_thiscall!(BASE_DESTRUCTOR, u32, this)
    }
});
