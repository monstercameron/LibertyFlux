// original: 0x00c22630 nav_cells_sync_locked

/// Runs the navigation-cell synchronization sequence using one temporary
/// two-word synchronization object. The first intercepted helper receives
/// the relocated global lock object as its sole stack argument; the next two
/// helpers receive the temporary through ECX. The temporary is intentionally
/// uninitialized because its contents are established by the helper sequence.
/// The function has no arguments, no direct data writes, and no semantic return.
lf_checker_rt::export!(cdecl, rw_00c22630() -> () {
    unsafe {
        const TEMP_WORDS: usize = 2;
        const NAV_CELL_LOCK: u32 = 0x016B_8FA4;

        let mut temporary = core::mem::MaybeUninit::<[u32; TEMP_WORDS]>::uninit();
        let temporary_ptr = temporary.as_mut_ptr() as *mut u32 as u32;
        let lock_ptr = lf_checker_rt::relocated(NAV_CELL_LOCK);

        let _ = lf_checker_rt::callee_thiscall!(0, u32, temporary_ptr, lock_ptr);
        let _ = lf_checker_rt::callee_thiscall!(1, u32, temporary_ptr);
        let _ = lf_checker_rt::callee_thiscall!(2, u32, temporary_ptr);
    }
});
