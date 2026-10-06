// original: 0x00872AF0 active_registration_release
/// Release the active motion-tree registration and clear its two globals.
///
/// When the pointer global holds a non-null value it is freed through the
/// thread-local manager (TLS slot 0 points at the thread block, whose word
/// at `+8` is the manager object; the free routine is vtable slot `0x0C` of
/// the manager's table, taking the block as its stack argument). Both the
/// pointer global and the count global are then zeroed. Always returns 0.
///
/// Original: 0x00872AF0 (cdecl, no arguments).
lf_checker_rt::export!(cdecl, rw_00872AF0() -> u32 {
    unsafe {
        const ACTIVE_PTR: u32 = 0x1B4AF2C;
        const ACTIVE_COUNT: u32 = 0x1B4AF30;
        const MANAGER_OFF: u32 = 8;
        const FREE_SLOT: u32 = 0x0C;
        let block = lf_checker_rt::global::<u32>(ACTIVE_PTR).read_unaligned();
        if block != 0 {
            let thread = lf_checker_rt::tls_slot(0);
            let manager = ((thread + MANAGER_OFF) as *const u32).read_unaligned();
            let vtable = (manager as *const u32).read_unaligned();
            let target = ((vtable + FREE_SLOT) as *const u32).read_unaligned();
            let free: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            free(manager, block);
        }
        lf_checker_rt::global::<u32>(ACTIVE_PTR).write_unaligned(0);
        lf_checker_rt::global::<u32>(ACTIVE_COUNT).write_unaligned(0);
    }
    0
});
