// original: 0x008B9600 system_dialog_is_active

/// Report whether a system dialog handle is live.
///
/// Reads the handle word at `HANDLE` and returns 1 when it holds anything
/// but -1, 0 otherwise. Leaf, cdecl, no arguments.
lf_checker_rt::export!(cdecl, rw_008B9600() -> u32 {
    unsafe {
        const HANDLE: u32 = 0x0117_6760;
        const NONE: u32 = 0xFFFF_FFFF;
        let h = (lf_checker_rt::relocated(HANDLE) as *const u32).read_unaligned();
        u32::from(h != NONE)
    }
});
