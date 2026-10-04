// original: 0x00a7cea0 stamp_value_merge_flags
// Stamp a value onto the node and merge flag bits: when the low word
// of `val` is positive, store it at +0x138 with the current global
// stamp at +0x134, run the link step over the +0x90 block (with `sel`,
// or the +0x10 block when `sel` is 0), set bit 7 of +0x13c from `b7`'s
// low bit, and toggle bit 0 of +0x13d when it disagrees with `tog`.
// Returns the toggled bit. When the word is zero or negative only
// +0x134 is written (-1) and the word itself is returned (contracts
// pin entry EAX to 0; the original passes its upper half through).
export!(thiscall, rw_s13_00a7cea0(
    this: *mut u8,
    sel: u32,
    val: u32,
    b7: u32,
    tog: u32,
) -> u32 {
    unsafe {
        if (val & 0xFFFF) as u16 as i16 <= 0 {
            *(this.add(0x134) as *mut u32) = 0xFFFF_FFFF;
            return val & 0xFFFF;
        }
        *(this.add(0x138) as *mut u16) = (val & 0xFFFF) as u16;
        *(this.add(0x134) as *mut u32) = *global::<u32>(0x11735C4);
        let block = if sel != 0 {
            sel
        } else {
            (this as u32).wrapping_add(0x10)
        };
        let link: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        link((this as u32).wrapping_add(0x90), block);
        let merged = (*this.add(0x13C) & 0x7F) | (((b7 & 0xFF) as u8) << 7);
        *this.add(0x13C) = merged;
        let bit = (*this.add(0x13D) ^ ((tog & 0xFF) as u8)) & 1;
        *this.add(0x13D) ^= bit;
        bit as u32
    }
});
