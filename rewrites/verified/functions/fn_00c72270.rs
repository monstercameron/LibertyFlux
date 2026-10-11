// original: 0x00C72270 copy_animation_tracks

/// Copy the nine selected words of each 0x30-byte track record from
/// `[source, source_end)` to `destination`. The words at offsets `+4`, `+8`
/// and `+0xc` remain untouched. The source interval is expected to contain a
/// whole number of records. This stdcall helper has no semantic return value.
lf_checker_rt::export!(stdcall, rw_00c72270(source: u32, source_end: u32, destination: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        const TRACK_STRIDE: u32 = 0x30;
        if source == source_end { return 0; }
        let mut source_track = source;
        let mut destination_track = destination;
        loop {
            wr32(destination_track, rd32(source_track));
            let word_14 = rd32(source_track + 0x14);
            let word_18 = rd32(source_track + 0x18);
            let word_10 = rd32(source_track + 0x10);
            wr32(destination_track + 0x10, word_10);
            wr32(destination_track + 0x14, word_14);
            wr32(destination_track + 0x18, word_18);

            let word_1c = rd32(source_track + 0x1c);
            wr32(destination_track + 0x1c, word_1c);
            let word_20 = rd32(source_track + 0x20);
            let word_24 = rd32(source_track + 0x24);
            let word_28 = rd32(source_track + 0x28);
            wr32(destination_track + 0x20, word_20);
            wr32(destination_track + 0x24, word_24);
            wr32(destination_track + 0x28, word_28);
            wr32(destination_track + 0x2c, rd32(source_track + 0x2c));

            source_track = source_track.wrapping_add(TRACK_STRIDE);
            destination_track = destination_track.wrapping_add(TRACK_STRIDE);
            if source_track == source_end {
                break;
            }
        }
        0
    }
});
