// original: 0x008aca80 rage::audWaveshaperEffect::vf4
/// audWaveshaperEffect::vf4: address of the waveshaper slot for the current tap.
///
/// Same shape as the delay twin with a stride of 20 bytes past a 6-unit head.
export!(thiscall, rw_008aca80(this_: *const u8) -> u32 {
    unsafe {
        let tap = *(this_.add(0x30) as *const u32);
        let units = tap.wrapping_add(6).wrapping_mul(5);
        (this_ as u32).wrapping_add(units.wrapping_mul(4))
    }
});
