// original: 0x00934740 query_state_bits
/// 0x00934740: report the set state. 0 when the state flag at +0x169 is
/// clear; otherwise 1 when bit 1 of the accumulated mask is set, else bit 2
/// of a second accumulation as 0 or 2.
export!(thiscall, rw_00934740(this: *const u8) -> u32 {
    callee_thiscall!(1, u32, this as u32);
    if unsafe { core::ptr::read((this as *const u8).add(0x169)) } == 0 {
        return 0;
    }
    let first = callee_thiscall!(2, u32, this as u32);
    if first & 2 != 0 {
        return 1;
    }
    let second = callee_thiscall!(3, u32, this as u32);
    (second & 4) >> 1
});
