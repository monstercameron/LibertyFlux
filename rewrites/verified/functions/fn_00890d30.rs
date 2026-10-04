// original: 0x00890d30 audio_mark_slots
/// Flag every populated slot node whose kind word is 2.
///
/// Visits the eight slot bytes at `this+0x48..+0x4f`, skipping 0xff slots.
/// For each live slot the node is resolved through the first row table; when
/// the node is non-null and its kind word at +6 equals 2, bit 2 is set at
/// +0x39 and bit 4 at +0x38. Returns the row-table base, or 0xff when the
/// last slot is empty, matching what the original leaves in EAX.
export!(thiscall, rw_00890d30(this: u32) -> u32 {
    unsafe {
        let stride = *global::<u32>(0x0115D964);
        let variant = ((this + 0x40) as *const u8).read() as u32;
        let base = *global::<u32>(0x0115D988);
        for i in 0..8u32 {
            let slot = ((this + 0x48 + i) as *const u8).read();
            if slot == 0xff {
                continue;
            }
            let row = ((base
                .wrapping_add(variant.wrapping_mul(0x6f40))
                .wrapping_add(0x6f10)) as *const u32)
                .read();
            let node = stride.wrapping_mul(slot as u32).wrapping_add(row);
            if node == 0 {
                continue;
            }
            if ((node + 6) as *const u16).read_unaligned() != 2 {
                continue;
            }
            let p39 = (node + 0x39) as *mut u8;
            p39.write(p39.read() | 4);
            let p38 = (node + 0x38) as *mut u8;
            p38.write(p38.read() | 0x10);
        }
        if ((this + 0x4f) as *const u8).read() == 0xff {
            0xff
        } else {
            base
        }
    }
});
