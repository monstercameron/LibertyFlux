// original: 0x00e1632e file_close_stored
/// Close the stored handle unless it is a no-handle sentinel.
///
/// Reads the handle kept in the module global; `-1` and `-2` mean there is
/// nothing to close and are returned unchanged. Otherwise forwards the value
/// to the one-argument close routine reached through the import slot
/// (stdcall/1, planted stub id 1) and returns its answer.
export!(cdecl, rw_00e1632e() -> u32 {
    unsafe {
        const HANDLE_GLOBAL: u32 = 0x1059510;
        const CLOSE_SLOT: u32 = 0xE73160;
        const NO_HANDLE_1: u32 = 0xFFFFFFFF;
        const NO_HANDLE_2: u32 = 0xFFFFFFFE;
        let handle = *global::<u32>(HANDLE_GLOBAL);
        if handle == NO_HANDLE_1 || handle == NO_HANDLE_2 {
            return handle;
        }
        let target = *global::<u32>(CLOSE_SLOT);
        let close: extern "stdcall" fn(u32) -> u32 =
            core::mem::transmute(target as usize);
        close(handle)
    }
});
