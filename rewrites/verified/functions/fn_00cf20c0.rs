// original: 0x00CF20C0 playstat_event_name

/// Return the event-name pointer for a play-stat event identifier.
///
/// The cdecl argument is an unsigned table index. Indices zero through 46
/// read a pointer from the image's event-name table and use it when nonzero;
/// every out-of-range index and every empty entry returns the relocated
/// `STAT_UNKNOWN` string. The result is a pointer-sized 32-bit image address.
lf_checker_rt::export!(cdecl, rw_fn_00cf20c0(event_id: u32) -> u32 {
    const EVENT_NAME_TABLE_VA: u32 = 0x00ed_e0e0;
    const EVENT_NAME_LIMIT: u32 = 0x2e;
    const UNKNOWN_NAME_VA: u32 = 0x00ed_e448;
    let entry = if event_id <= EVENT_NAME_LIMIT {
        unsafe {
            lf_checker_rt::global::<u32>(EVENT_NAME_TABLE_VA)
                .add(event_id as usize)
                .read_unaligned()
        }
    } else {
        0
    };
    if entry != 0 { entry } else { lf_checker_rt::relocated(UNKNOWN_NAME_VA) }
});
