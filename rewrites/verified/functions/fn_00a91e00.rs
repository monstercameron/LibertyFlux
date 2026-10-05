// original: 0x00a91e00 stream_global_table_get

/// Global dword table lookup.
///
/// `index` selects one dword from the game's global table at file VA
/// 0x12FB278 (streaming subsystem globals). No calls, no writes.
/// Original: 0x00A91E00 (cdecl, one stack word), 12 bytes.
lf_checker_rt::export!(cdecl, rw_00a91e00(index: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x12FB278;
        let base = lf_checker_rt::global::<u32>(TABLE);
        base.offset(index as isize).read()
    }
});
