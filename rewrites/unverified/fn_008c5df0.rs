// original: 0x008C5DF0 file_mode_label_pointer
/// Select the file-memory label for the active mode, optionally obtaining the
/// mode through the intercepted helper. The override flag selects mode six;
/// modes five and six also update the object's marker byte.
lf_checker_rt::export!(thiscall, rw_008c5df0(this: u32, use_helper: u32) -> u32 {
    const MODE_HELPER: u32 = 1;
    const MODE_FILE_VA: u32 = 0x0116_0CC8;
    unsafe {
        let mode = if use_helper != 0 {
            lf_checker_rt::callee_thiscall!(MODE_HELPER, u32, this)
        } else {
            lf_checker_rt::global::<u32>(MODE_FILE_VA).read_unaligned()
        };
        ((this + 0x260) as *mut u8).write(0x65);
        let selected = if ((this + 0x263) as *const u8).read() != 0 { 6 } else { mode };
        match selected {
            0 => lf_checker_rt::relocated(0x00E7_F948),
            1 => lf_checker_rt::relocated(0x00E7_F958),
            2 => lf_checker_rt::relocated(0x00E7_F964),
            3 => lf_checker_rt::relocated(0x00E7_F97C),
            4 => lf_checker_rt::relocated(0x00E7_F970),
            5 => { ((this + 0x260) as *mut u8).write(0x72); lf_checker_rt::relocated(0x00E7_F988) },
            6 => { ((this + 0x260) as *mut u8).write(0x6A); lf_checker_rt::relocated(0x00E7_F994) },
            _ => lf_checker_rt::relocated(0x00E7_F9A4),
        }
    }
});
