// original: 0x00a0a420 mission_cleanup_load
/// Load each mission-cleanup entry through the stream helper. The manager's
/// primary table has 256 entries at +4; the secondary table has 200 entries
/// at +0x2c04. Each entry is 0x2c bytes. After reading a primary entry, the
/// restore helper runs only when the byte global at 0x0116d27d is zero. The
/// secondary entries are read without restoration. The method is thiscall,
/// takes no stack arguments, and has no specified return value.
lf_checker_rt::export!(thiscall, rw_00a0a420(this: u32) -> () {
    unsafe {
        const ENTRY_SIZE: u32 = 0x2c;
        const PRIMARY_BASE: u32 = 4;
        const PRIMARY_COUNT: u32 = 256;
        const SECONDARY_BASE: u32 = 0x2c04;
        const SECONDARY_COUNT: u32 = 200;
        const LOAD_RESTORE_FLAG: u32 = 0x0116d27d;
        const READ_ENTRY: u32 = 0;
        const RESTORE_ENTRY: u32 = 1;

        #[inline(always)]
        unsafe fn read_restore_flag(file_va: u32) -> u8 {
            let flag = lf_checker_rt::global::<u8>(file_va);
            unsafe { flag.read() }
        }

        for index in 0..PRIMARY_COUNT {
            let entry = this
                .wrapping_add(PRIMARY_BASE)
                .wrapping_add(index.wrapping_mul(ENTRY_SIZE));
            let _ = lf_checker_rt::callee_cdecl!(READ_ENTRY, u32, entry, ENTRY_SIZE);
            if read_restore_flag(LOAD_RESTORE_FLAG) == 0 {
                let _ = lf_checker_rt::callee_thiscall!(
                    RESTORE_ENTRY,
                    u32,
                    this,
                    entry
                );
            }
        }

        for index in 0..SECONDARY_COUNT {
            let entry = this
                .wrapping_add(SECONDARY_BASE)
                .wrapping_add(index.wrapping_mul(ENTRY_SIZE));
            let _ = lf_checker_rt::callee_cdecl!(READ_ENTRY, u32, entry, ENTRY_SIZE);
        }
    }
});
