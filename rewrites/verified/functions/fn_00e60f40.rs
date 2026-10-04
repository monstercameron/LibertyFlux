// original: 0x00e60f40 init_block_19f8014
/// Reset the state block at 0x019F8014, then notify the shared helper.
///
/// Zeroes the seven counter words, stores the default limit 100, clears flag
/// bit 0, then forwards the block descriptor to the shared helper (cdecl/1,
/// stubbed) and returns its answer. The two loads the original performs
/// right after zeroing are dead (both read back zero) and are omitted.
export!(cdecl, rw_00e60f40() -> u32 {
    unsafe {
        const BASE: u32 = 0x019F8014;
        for off in [0x00u32, 0x04, 0x08, 0x0C, 0x10, 0x14, 0x18] {
            *global::<u32>(BASE + off) = 0;
        }
        *global::<u32>(BASE + 0x1C) = 100;
        *global::<u8>(BASE + 0x20) &= !1;
        callee_cdecl!(1, u32, relocated(0x00E6FDB0))
    }
});
