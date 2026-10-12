// original: 0x008C4E00 gallery_file_open

/// Open a gallery file using the fixed mode string. A null handle is returned directly; a valid handle triggers the data query and close helpers, then is returned.
lf_checker_rt::export!(cdecl, rw_008c4e00(path: u32) -> u32 {
    const OPEN_CALLEE: u32 = 74;
    const GET_DATA_CALLEE: u32 = 75;
    const CLOSE_CALLEE: u32 = 76;
    const MODE_STRING_VA: u32 = 0x00E7F738;
    let handle = unsafe { lf_checker_rt::callee_cdecl!(OPEN_CALLEE, u32, path, lf_checker_rt::relocated(MODE_STRING_VA)) };
    if handle == 0 {
        return handle;
    }
    let data = unsafe { lf_checker_rt::callee_thiscall!(GET_DATA_CALLEE, u32, handle) };
    let _ = unsafe { lf_checker_rt::callee_thiscall!(CLOSE_CALLEE, u32, handle, data) };
    handle
});
