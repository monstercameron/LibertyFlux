// original: 0x008904b0 audio_node_flag8_field78
/// Return this object's +0x78 field unless its tag node has flag bit 3 set.
///
/// Resolves the tag node through the second row table (`stride_b * tag +
/// row_b[variant]`, tag from `this+4`); a 0xff tag means no node. When the
/// node's flag byte at +0xe8 has bit 3 set the result is null, otherwise the
/// field at `this+0x78`.
export!(thiscall, rw_008904b0(this: u32) -> u32 {
    unsafe {
        let tag = ((this + 4) as *const u8).read();
        let node = if tag == 0xff {
            0
        } else {
            let stride = *global::<u32>(0x0115D968);
            let variant = ((this + 0x40) as *const u8).read() as u32;
            let base = *global::<u32>(0x0115D988);
            let row = ((base
                .wrapping_add(variant.wrapping_mul(0x6f40))
                .wrapping_add(0x6f14)) as *const u32)
                .read();
            stride.wrapping_mul(tag as u32).wrapping_add(row)
        };
        if ((((node + 0xe8) as *const u8).read()) & 8) != 0 {
            0
        } else {
            ((this + 0x78) as *const u32).read()
        }
    }
});
