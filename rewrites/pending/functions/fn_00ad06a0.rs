// original: 0x00ad06a0 set_flag8_when_active
/// Set flag bit 3 once the object is active and the peer allows it.
///
/// Returns early when the bit is already set or the field at 0x160 does not
/// clear zero; otherwise a peer byte decides, and the bit is set only when
/// its 0x10 bit is clear. Returns a per-path leftover register value.
export!(thiscall, rw_00ad06a0(this: *mut u8, peer: *const u8) -> u32 {
    unsafe {
        let flags = *(this.add(0x164) as *const u32);
        if flags & 8 != 0 {
            return flags >> 3;
        }
        let speed = *(this.add(0x160) as *const f32);
        if !(speed > *global::<f32>(0x00FE8628)) {
            return flags >> 3;
        }
        if *peer.add(0xf17) & 0x10 != 0 {
            return peer as u32;
        }
        *(this.add(0x164) as *mut u32) = flags | 8;
        peer as u32
    }
});
