// original: 0x00e61190 init_block_19fe68c
/// Reset the state block at 0x019FE68C, then notify the shared helper.
///
/// Same shape as [`rw_00e60f40`] at a third block address.
export!(cdecl, rw_00e61190() -> u32 {
    unsafe {
        const BASE: u32 = 0x019FE68C;
        for off in [0x00u32, 0x04, 0x08, 0x0C, 0x10, 0x14, 0x18] {
            *global::<u32>(BASE + off) = 0;
        }
        *global::<u32>(BASE + 0x1C) = 100;
        *global::<u8>(BASE + 0x20) &= !1;
        callee_cdecl!(1, u32, relocated(0x00E6FFA0))
    }
});
