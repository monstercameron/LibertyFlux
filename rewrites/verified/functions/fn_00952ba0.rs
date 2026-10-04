// original: 0x00952ba0 state_is_a
/// Report whether the object's state byte holds the first recognised
/// combination: bits 2 and 3 set and none of the bits in 0x73 set.
export!(thiscall, rw_00952ba0(this: *const u8) -> u32 {
    unsafe {
        let v = *this.add(0x2d);
        ((v & 0x0c) == 0x0c && (v & 0x73) == 0) as u32
    }
});
