// original: 0x008904f0 audio_node_flag10_field90
/// Return this object's +0x90 field unless its tag node has flag bit 4 set.
///
/// Same shape as [`rw_008904b0`] with the flag mask 0x10 and the field at
/// `this+0x90`.
export!(thiscall, rw_008904f0(this: u32) -> u32 {
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
        if ((((node + 0xe8) as *const u8).read()) & 0x10) != 0 {
            0
        } else {
            ((this + 0x90) as *const u32).read()
        }
    }
});
