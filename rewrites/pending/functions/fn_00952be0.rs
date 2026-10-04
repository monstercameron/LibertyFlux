// original: 0x00952be0 state_is_c
/// Report whether the object's state byte holds the third recognised
/// combination: bits 1 and 2 set and none of the bits in 0x79 set.
export!(thiscall, rw_00952be0(this: *const u8) -> u32 {
    unsafe {
        let v = *this.add(0x2d);
        ((v & 0x06) == 0x06 && (v & 0x79) == 0) as u32
    }
});
