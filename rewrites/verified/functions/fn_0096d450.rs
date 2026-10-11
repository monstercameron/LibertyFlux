// original: 0x0096D450 copy_timing_sample_record

/// Marks the object sample state active and copies four words from the source
/// record to offsets `0x2f40`, `0x2f44`, `0x2f48` and `0x2f4c`. Float-shaped
/// words are copied bit for bit; the last copied word is returned in EAX.
lf_checker_rt::export!(thiscall, rw_0096d450(this: u32, source: u32) -> u32 {
    unsafe {
        (this.wrapping_add(0x2ef5) as *mut u8).write(1);
        let mut last = 0u32;
        for (source_offset, target_offset) in [(0u32, 0x2f40u32), (4, 0x2f44), (8, 0x2f48), (12, 0x2f4c)] {
            last = (source.wrapping_add(source_offset) as *const u32).read_unaligned();
            (this.wrapping_add(target_offset) as *mut u32).write_unaligned(last);
        }
        last
    }
});
