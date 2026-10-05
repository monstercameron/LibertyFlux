// original: 0x008C6DC0 key6_is_greater
/// Decide whether one six-word sort key sorts strictly after another.
///
/// Compares the signed dwords at `this` and `other` pair by pair from
/// offset 0; the first differing pair decides. Returns 1 when `this` is
/// greater, 0 otherwise (less or equal). Only the low result byte is
/// behaviour. Original: thiscall, one stack word.
lf_checker_rt::export!(thiscall, rw_008c6dc0(this: u32, other: u32) -> u32 {
    unsafe {
        for i in 0..6u32 {
            let a = ((this + i * 4) as *const i32).read_unaligned();
            let b = ((other + i * 4) as *const i32).read_unaligned();
            if a != b {
                return (a > b) as u32;
            }
        }
        0
    }
});
