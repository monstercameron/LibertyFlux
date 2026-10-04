// original: 0x00890550 audio_walk_flag8
/// Walk the node chain to the first node whose tag node has flag bit 3 set.
///
/// Starting at this object, each step checks the current record's tag node
/// (second row table) for flag bit 3 at +0xe7 and otherwise follows the link
/// byte at +5 through the first row table. A 0xff tag skips the flag test, a
/// 0xff link ends the walk. Returns the final record's tag byte (the low byte
/// of EAX; the upper bytes keep whatever the caller passed in EAX).
export!(thiscall, rw_00890550(this: u32) -> u32 {
    unsafe {
        let mut cur = this;
        loop {
            let tag = ((cur + 4) as *const u8).read();
            if tag != 0xff {
                let stride = *global::<u32>(0x0115D968);
                let variant = ((cur + 0x40) as *const u8).read() as u32;
                let base = *global::<u32>(0x0115D988);
                let row = ((base
                    .wrapping_add(variant.wrapping_mul(0x6f40))
                    .wrapping_add(0x6f14)) as *const u32)
                    .read();
                let node = stride.wrapping_mul(tag as u32).wrapping_add(row);
                if ((((node + 0xe7) as *const u8).read()) & 8) != 0 {
                    return tag as u32;
                }
            }
            let link = ((cur + 5) as *const u8).read();
            if link == 0xff {
                return tag as u32;
            }
            let stride = *global::<u32>(0x0115D964);
            let variant = ((cur + 0x40) as *const u8).read() as u32;
            let base = *global::<u32>(0x0115D988);
            let row = ((base
                .wrapping_add(variant.wrapping_mul(0x6f40))
                .wrapping_add(0x6f10)) as *const u32)
                .read();
            cur = stride.wrapping_mul(link as u32).wrapping_add(row);
        }
    }
});
