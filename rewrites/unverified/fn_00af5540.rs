// original: 0x00AF5540 select_flagged_global

/// Select one of two global values according to a byte flag. A zero flag
/// returns the default value; any nonzero flag returns the alternate value.
/// The values and flag are read from the relocated image data each call.
lf_checker_rt::export!(cdecl, rw_00af5540() -> u32 {
    const SELECTION_FLAG: u32 = 0x011609F6;
    const DEFAULT_VALUE: u32 = 0x011735B4;
    const ALTERNATE_VALUE: u32 = 0x01173594;

    unsafe {
        let is_alternate = lf_checker_rt::global::<u8>(SELECTION_FLAG).read_volatile() != 0;
        let selected = if is_alternate { ALTERNATE_VALUE } else { DEFAULT_VALUE };
        lf_checker_rt::global::<u32>(selected).read_volatile()
    }
});
