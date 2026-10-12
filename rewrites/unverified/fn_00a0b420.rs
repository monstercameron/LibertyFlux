// original: 0x00a0b420 mission_cleanup_save
/// Serialize the manager's 256 primary records and 200 secondary records in
/// order. Each record is 44 bytes. The validity helper receives the manager,
/// the record pointer, and a mode flag (one for primary records, zero for
/// secondary records). A valid record is forwarded directly; an invalid one
/// is replaced by the helper-initialized default record. The file writer is
/// called once for every record. Thiscall with no stack arguments and no
/// specified return value.
lf_checker_rt::export!(thiscall, rw_00a0b420(this: u32) -> () {
    unsafe {
        const PRIMARY_BASE: u32 = 4;
        const PRIMARY_COUNT: u32 = 256;
        const SECONDARY_BASE: u32 = 0x2c04;
        const SECONDARY_COUNT: u32 = 200;
        const ENTRY_SIZE: u32 = 0x2c;
        const INIT_DEFAULT: u32 = 0;
        const IS_VALID: u32 = 1;
        const WRITE_ENTRY: u32 = 2;
        const CHECK_COOKIE: u32 = 3;
        const SECURITY_COOKIE_VA: u32 = 0x01057fb4;

        let mut default_record = [0u32, u32::MAX, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let default_ptr = default_record.as_mut_ptr() as u32;
        let _ = lf_checker_rt::callee_thiscall!(INIT_DEFAULT, u32, default_ptr);

        for index in 0..PRIMARY_COUNT {
            let entry = this
                .wrapping_add(PRIMARY_BASE)
                .wrapping_add(index.wrapping_mul(ENTRY_SIZE));
            let valid = lf_checker_rt::callee_thiscall!(IS_VALID, u32, this, entry, 1);
            let data = if valid != 0 { entry } else { default_ptr };
            let _ = lf_checker_rt::callee_cdecl!(WRITE_ENTRY, u32, data, ENTRY_SIZE);
        }

        for index in 0..SECONDARY_COUNT {
            let entry = this
                .wrapping_add(SECONDARY_BASE)
                .wrapping_add(index.wrapping_mul(ENTRY_SIZE));
            let valid = lf_checker_rt::callee_thiscall!(IS_VALID, u32, this, entry, 0);
            let data = if valid != 0 { entry } else { default_ptr };
            let _ = lf_checker_rt::callee_cdecl!(WRITE_ENTRY, u32, data, ENTRY_SIZE);
        }

        let cookie = lf_checker_rt::global::<u32>(SECURITY_COOKIE_VA).read();
        let _ = lf_checker_rt::callee_thiscall!(CHECK_COOKIE, u32, cookie);
    }
});
