// original: 0x006073b0 streaming_copy_matrix_block

/// Copies the eight populated words of a streaming matrix block from the
/// source in instruction order. Gaps at byte offsets 12 and 28 remain intact;
/// EAX returns the last copied source word.
lf_checker_rt::export!(thiscall, rw_006073b0(this: u32, source: u32) -> u32 {
    const SOURCE_OFFSETS: [u32; 8] = [0, 4, 8, 16, 20, 24, 32, 36];
    const TARGET_OFFSETS: [u32; 8] = [0x120, 0x124, 0x128, 0x130, 0x134, 0x138, 0x140, 0x144];
    unsafe {
        let mut last = 0;
        let mut index = 0;
        while index < SOURCE_OFFSETS.len() {
            last = (source.wrapping_add(SOURCE_OFFSETS[index]) as *const u32).read_unaligned();
            (this.wrapping_add(TARGET_OFFSETS[index]) as *mut u32).write_unaligned(last);
            index += 1;
        }
        last = (source.wrapping_add(40) as *const u32).read_unaligned();
        (this.wrapping_add(0x148) as *mut u32).write_unaligned(last);
        last
    }
});
