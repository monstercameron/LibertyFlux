// original: 0x00c62340 Anim_GetTotalTime
/// Total animation time: the duration word of the bound animation.
///
/// When the tag at `+0x44` is 1, reads the duration at `+0xC` of the animation
/// bound at `+0x40`. Any other tag makes the original read through a null
/// pointer, which faults; the rewrite performs the same faulting read so both
/// sides fault identically.
export!(thiscall, rw_00c62340(this: u32) -> f32 {
    unsafe {
        let tag = *((this + 0x44) as *const u16);
        if tag == 1 {
            let anim = *((this + 0x40) as *const u32);
            f32::from_bits(*((anim + 0xC) as *const u32))
        } else {
            core::ptr::read_volatile(0xC as *const f32)
        }
    }
});
