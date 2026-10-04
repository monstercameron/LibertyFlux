// original: 0x008ac350 rage::audReverbEffect::vf1
/// Second-stage initialiser for a reverb effect.
///
/// Runs the shared base initialiser (callee 1) with the two forwarded
/// arguments; a zero low byte returns the answer unchanged. Otherwise four
/// parameter words are fetched from the attached parameter block through an
/// unaligned header, fanned out to the three per-channel blocks (keeping a
/// fourth copy at the staging slots), the tail flag is cleared, and each
/// channel is then refreshed through the virtual refresh hook (slot 5 of the
/// object's table) followed by the direct refresh helper (callee 3).
/// Returns the last helper answer with its low byte forced to 1.
export!(thiscall, rw_008ac350(this: *mut u8, a: u32, b: u32) -> u32 {
    unsafe {
        let answer: u32 = callee_thiscall!(1, u32, this as u32, a, b);
        if answer & 0xFF == 0 {
            return answer;
        }
        *this.add(0xC4) = 0;
        let src = *(this.add(4) as *const u32) as *const u8;
        let d0 = (src.add(0x0F) as *const u32).read_unaligned();
        let d1 = (src.add(0x13) as *const u32).read_unaligned();
        let d2 = (src.add(0x17) as *const u32).read_unaligned();
        let d3 = (src.add(0x1B) as *const u32).read_unaligned();
        for &off in &[0x74usize, 0x88, 0x9C, 0xB0] {
            *(this.add(off) as *mut u32) = d0;
            *(this.add(off + 4) as *mut u32) = d1;
            *(this.add(off + 8) as *mut u32) = d2;
            *(this.add(off + 12) as *mut u32) = d3;
        }
        let vtable = *(this as *const u32);
        let slot = *(((vtable as *const u8).add(0x14)) as *const u32);
        let hook: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        let mut last = 0u32;
        for _ in 0..3 {
            hook(this as u32);
            last = callee_thiscall!(3, u32, this as u32);
        }
        (last & 0xFFFF_FF00) | 1
    }
});
