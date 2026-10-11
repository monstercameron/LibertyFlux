// original: 0x00AC8B60 reset_stream_registry_counts

/// Reset the active streaming registry count and its 16-bit cursor.
///
/// The function writes zero to the dword at `0x154e140` and the word at
/// `0x154e158`. It also returns zero in EAX.
lf_checker_rt::export!(cdecl, rw_00ac8b60() -> u32 {
    const ACTIVE_COUNT: u32 = 0x0015_4e140;
    const REGISTRY_CURSOR: u32 = 0x0015_4e158;

    unsafe {
        lf_checker_rt::global::<u32>(ACTIVE_COUNT).write(0);
        lf_checker_rt::global::<u16>(REGISTRY_CURSOR).write(0);
    }
    0
});
