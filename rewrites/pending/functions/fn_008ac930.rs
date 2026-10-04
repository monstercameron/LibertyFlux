// original: 0x008ac930 rage::audDelayEffect::vf4
/// audDelayEffect::vf4: address of the delay slot for the current tap.
///
/// The tap index at `this+0x30` selects a slot; each step advances 72 bytes
/// past a 15-unit header. Pure address arithmetic, wrapping like the original.
export!(thiscall, rw_008ac930(this_: *const u8) -> u32 {
    unsafe {
        let tap = *(this_.add(0x30) as *const u32);
        let bytes = tap.wrapping_mul(9).wrapping_add(15).wrapping_mul(8);
        (this_ as u32).wrapping_add(bytes)
    }
});
