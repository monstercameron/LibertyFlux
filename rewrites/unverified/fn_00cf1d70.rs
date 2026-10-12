// original: 0x00CF1D70 stat_passes_mode_check

/// Check whether a statistic identifier is enabled in the current mode.
///
/// If the mode global is not 2, or the unsigned identifier falls outside
/// `0x121..=0x1cc`, the result is true. Otherwise, the function indexes the
/// image-resident selector byte table and returns whether that byte is
/// nonzero. The selector table is read from the mapped image so changes to its
/// values are observed. The result is a Boolean in AL.
lf_checker_rt::export!(cdecl, rw_fn_00cf1d70(stat_id: u32) -> u32 {
    const MODE_GLOBAL_VA: u32 = 0x0116_0cc8;
    const FIRST_MODE_ID: u32 = 0x121;
    const LAST_MODE_ID: u32 = 0x1cc;
    const SELECTOR_TABLE_VA: u32 = 0x00cf_1da8;

    let current_mode = unsafe { lf_checker_rt::global::<u32>(MODE_GLOBAL_VA).read_unaligned() };
    let relative_id = stat_id.wrapping_sub(FIRST_MODE_ID);
    if current_mode != 2 || relative_id > LAST_MODE_ID - FIRST_MODE_ID {
        return 1;
    }
    let selector = unsafe {
        lf_checker_rt::global::<u8>(SELECTOR_TABLE_VA)
            .add(relative_id as usize)
            .read()
    };
    u32::from(selector != 0)
});
