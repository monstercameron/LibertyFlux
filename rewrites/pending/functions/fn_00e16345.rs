// original: 0x00e16345 file_open_stored
/// Open the fixed-path file for reading and store the handle.
///
/// Calls the seven-argument open routine reached through the import slot
/// (stdcall/7, planted stub id 1) with constant arguments: the path pointer
/// (relocated with the image), read access, read/write sharing, default
/// security, open-existing disposition, no flags and no template. Stores the
/// returned handle in the module global and returns it.
export!(cdecl, rw_00e16345() -> u32 {
    unsafe {
        const OPEN_SLOT: u32 = 0xE73154;
        const HANDLE_GLOBAL: u32 = 0x1059510;
        const PATH: u32 = 0xF13AE4;
        const READ_ACCESS: u32 = 0x40000000;
        const SHARE_RW: u32 = 3;
        const OPEN_EXISTING: u32 = 3;
        let target = *global::<u32>(OPEN_SLOT);
        let open: extern "stdcall" fn(u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        let handle = open(relocated(PATH), READ_ACCESS, SHARE_RW, 0, OPEN_EXISTING, 0, 0);
        *global::<u32>(HANDLE_GLOBAL) = handle;
        handle
    }
});
