// original: 0x00bebdf0 compare_slots_24_28
/// Classify two signed dwords at offsets 0x24 and 0x28 into 0-3.
///
/// With `a = [this+0x24]` and `b = [this+0x28]` as signed: returns 1 when
/// `a > 0` and `a == b`, 2 when `a > 0` and `a > b`, 3 when `a <= 0` and
/// `b > 0`, otherwise 0. (The original's tail has a dead conditional move
/// that can only produce 0; the two `je`/`jle` arms on the `a <= 0, b > 0`
/// path are unreachable.) Reads only, no stores. Thiscall, no stack
/// arguments.
export!(thiscall, rw_00bebdf0(this: u32) -> u32 {
    unsafe {
        let a = ((this + 0x24) as *const i32).read_unaligned();
        let b = ((this + 0x28) as *const i32).read_unaligned();
        if a > 0 {
            if a == b {
                1
            } else if a > b {
                2
            } else {
                0
            }
        } else if b > 0 {
            3
        } else {
            0
        }
    }
});
