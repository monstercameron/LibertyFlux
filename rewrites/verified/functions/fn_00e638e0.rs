// original: 0x00e638e0 zero_entries_and_notify
/// Zero the 0x3840-entry table at 0x11A9D20, then notify with 0xE714E0.
///
/// Each 12-byte entry contributes two zeroed dwords; the remaining four bytes
/// per entry are left untouched. Forwards a constant block pointer to the
/// shared notifier (cdecl/1) and returns its answer.
export!(cdecl, rw_00e638e0() -> u32 {
    unsafe {
        const TABLE: u32 = 0x11A9D20;
        const COUNT: usize = 0x3840;
        const STRIDE: usize = 0xC;
        const BLOCK: u32 = 0xE714E0;
        let mut p = relocated(TABLE) as *mut u8;
        for _ in 0..COUNT {
            *(p as *mut u32) = 0;
            *(p.add(4) as *mut u32) = 0;
            p = p.add(STRIDE);
        }
        callee_cdecl!(1, u32, relocated(BLOCK))
    }
});
