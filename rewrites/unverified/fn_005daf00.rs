// original: 0x005DAF00 read_active_or_default_word_4c

/// Read one 32-bit field from the requested class record when it is active,
/// otherwise read that field from the configured default class record.
/// `class_index` is an unsigned table index; no bounds check is performed.
/// This cdecl function leaves its one stack argument for the caller to pop.
lf_checker_rt::export!(cdecl, rw_005daf00(class_index: u32) -> u32 {
    const CLASS_TABLE: u32 = 0x0118f6f8;
    const DEFAULT_CLASS_INDEX: u32 = 0x01034494;
    const ACTIVE_FLAG_OFFSET: usize = 0x08;
    const FIELD_OFFSET: usize = 0x4c;

    unsafe {
        let table = lf_checker_rt::global::<*mut u8>(CLASS_TABLE);
        let requested = table.add(class_index as usize).read_unaligned();
        let selected = if requested.add(ACTIVE_FLAG_OFFSET).read() != 0 {
            requested
        } else {
            let fallback_index = lf_checker_rt::global::<u32>(DEFAULT_CLASS_INDEX).read_unaligned();
            table.add(fallback_index as usize).read_unaligned()
        };
        selected.add(FIELD_OFFSET).cast::<u32>().read_unaligned()
    }
});
