// original: 0x00c17f10 CCamReplay::vf6

/// Reset the camera-replay interpolation state (vtable slot 6).
///
/// Clears the position and parameter words, resets the two flag bytes, fills
/// the 20-entry blend table with index-sentinel/max-blend pairs, then stamps
/// the tail constants. Leaf: no calls, no globals. Returns nothing meaningful.
export!(thiscall, rw_00c17f10(this: u32) -> u32 {
    unsafe {
        let bytes = this as *mut u8;
        let words = this as *mut u32;
        // Flag byte: clear bit 0, set bit 1.
        let flags = bytes.add(0x39b).read();
        bytes.add(0x39b).write((flags & 0xfe) | 0x02);
        // Clear the position block.
        for off in [0x2c4usize, 0x2c8, 0x2c0, 0x2cc] {
            words.add(off / 4).write(0);
        }
        // Clear the parameter block.
        for off in [0x288usize, 0x284, 0x280] {
            words.add(off / 4).write(0);
        }
        // Second flag byte: keep bits 1, 4, 6 and 7, set bit 1; set bit 2 on the first.
        let mode = bytes.add(0x398).read();
        bytes.add(0x39b).write(bytes.add(0x39b).read() | 0x04);
        bytes.add(0x398).write((mode & 0xd2) | 0x02);
        // Clear the tail block.
        for off in [0x37cusize, 0x380, 0x384, 0x388] {
            words.add(off / 4).write(0);
        }
        // Sentinel pair ahead of the blend table.
        words.add(0x2d4 / 4).write(0xffff_ffff);
        words.add(0x2d8 / 4).write(0xffff_fc18);
        // Blend table: 20 entries of (index sentinel, max blend).
        let mut entry = words.add(0x2e0 / 4);
        for _ in 0..0x14 {
            entry.sub(1).write(0xffff_ffff);
            entry.write(0x7f7f_ffff);
            entry = entry.add(2);
        }
        // Clear bit 6 on both flag bytes.
        bytes.add(0x398).write(bytes.add(0x398).read() & 0xbf);
        bytes.add(0x39b).write(bytes.add(0x39b).read() & 0xbf);
        // Tail constants and zero counters.
        for off in [0x38cusize, 0x390, 0x394] {
            words.add(off / 4).write(7);
        }
        words.add(0x2b0 / 4).write(0x4248_0000);
        words.add(0x2ac / 4).write(0);
        bytes.add(0x399).write(0);
    }
    0
});
