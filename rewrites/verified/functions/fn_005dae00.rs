// original: 0x005DAE00 test_active_or_default_flag_mask

/// Test whether any bit in `mask` is present in the selected record's
/// zero-extended 16-bit flags at `+0x20`. Select the indexed record when its
/// byte at `+0x08` is nonzero; otherwise use the configured default index.
/// Both arguments are unsigned cdecl stack words; the result is returned in AL.
lf_checker_rt::export!(cdecl, rw_005dae00(class_index: u32, mask: u32) -> u8 {
    const CLASS_TABLE: u32 = 0x0118_f6f8;
    const DEFAULT_CLASS_INDEX: u32 = 0x0103_4494;
    const ACTIVE_FLAG_OFFSET: usize = 0x08;
    const FLAGS_OFFSET: usize = 0x20;
    unsafe {
        let table = lf_checker_rt::global::<*mut u8>(CLASS_TABLE);
        let requested = table.add(class_index as usize).read_unaligned();
        let selected = if requested.add(ACTIVE_FLAG_OFFSET).read() != 0 {
            requested
        } else {
            let fallback_index = lf_checker_rt::global::<u32>(DEFAULT_CLASS_INDEX).read_unaligned();
            table.add(fallback_index as usize).read_unaligned()
        };
        let flags = selected.add(FLAGS_OFFSET).cast::<u16>().read_unaligned() as u32;
        u8::from(flags & mask != 0)
    }
});
