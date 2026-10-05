// original: 0x00AC6470 stream_open_named (proposed)

/// Open the named streaming source, releasing any previous handle.
///
/// The original stores the current word (`cur`) to the current-name global
/// and opens `name` through the open callee (cdecl, three words: `name`,
/// `cur`, `previous`). With no previous handle it returns the open answer;
/// otherwise it releases the previous handle through its virtual release
/// slot `+8` and slot `+0`, then returns `previous`.
lf_checker_rt::export!(cdecl, rw_00AC6470(name: u32, cur: u32, previous: u32) -> u32 {
    unsafe {
        const CURRENT: u32 = 0x0154E000;
        const OPEN: u32 = 1;
        const RELEASE_SLOT: u32 = 8;
        const CLOSE_SLOT: u32 = 0;
        (lf_checker_rt::relocated(CURRENT) as *mut u32).write_unaligned(cur);
        let handle = lf_checker_rt::callee_cdecl!(OPEN, u32, name, 0u32, 0u32, 0xffff_ffffu32, 0u32);
        if previous == 0 {
            return handle;
        }
        if handle != 0 {
            let vt = (handle as *const u32).read_unaligned();
            let release: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute((vt.wrapping_add(RELEASE_SLOT) as *const u32).read_unaligned()
                    as usize);
            release(handle);
            let vt2 = (handle as *const u32).read_unaligned();
            let close: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute((vt2.wrapping_add(CLOSE_SLOT) as *const u32).read_unaligned()
                    as usize);
            close(handle, 1);
        }
        previous
    }
});
