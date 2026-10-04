// original: 0x00e63cc0 init_tagged_table_and_notify
/// Initialise the 512-entry table at 0x120CA70, then notify with 0xE71620.
///
/// Each 0x14-byte entry gets two zero dwords, one 0xFFFFFFFF dword, one zero
/// byte and one zero dword; the remaining bytes are untouched. Forwards a
/// constant block pointer to the shared notifier (cdecl/1) and returns its
/// answer.
export!(cdecl, rw_00e63cc0() -> u32 {
    unsafe {
        const CURSOR0: u32 = 0x120CA78;
        const COUNT: usize = 0x200;
        const STRIDE: usize = 0x14;
        const BLOCK: u32 = 0xE71620;
        let mut p = relocated(CURSOR0) as *mut u8;
        for _ in 0..COUNT {
            *((p as *mut u32).offset(-2)) = 0;
            *((p as *mut u32).offset(-1)) = 0;
            *(p as *mut u32) = 0xFFFFFFFF;
            *p.add(4) = 0;
            *(p.add(8) as *mut u32) = 0;
            p = p.add(STRIDE);
        }
        callee_cdecl!(1, u32, relocated(BLOCK))
    }
});
