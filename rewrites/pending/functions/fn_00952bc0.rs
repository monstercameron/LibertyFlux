// original: 0x00952bc0 state_is_b
/// Report whether the object's state byte holds the second recognised
/// combination: bits 1, 2 and 3 set and none of the bits in 0x71 set.
export!(thiscall, rw_00952bc0(this: *const u8) -> u32 {
    unsafe {
        let v = *this.add(0x2d);
        ((v & 0x0e) == 0x0e && (v & 0x71) == 0) as u32
    }
});
