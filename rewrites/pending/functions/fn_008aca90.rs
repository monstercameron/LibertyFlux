// original: 0x008aca90 rage::audWaveshaperEffect::vf1
/// audWaveshaperEffect::vf1: run shared setup, then install default curves.
///
/// Same call discipline as the delay twin. On success, three identical
/// 20-byte parameter blocks (at `+0x78`, `+0x8c`, `+0xa0`) are filled from the
/// game's default words: two zero gains, a unity gain, a zero counter, a zero
/// flag and the default mode byte.
export!(thiscall, rw_008aca90(this_: *mut u8, a: u32, b: u32) -> u32 {
    unsafe {
        let init: extern "thiscall" fn(*mut u8, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let answer = init(this_, a, b);
        if (answer & 0xFF) == 0 {
            return answer;
        }
        let zero_bits: u32 = *global::<u32>(0x10309B0);
        let one_bits: u32 = *global::<u32>(0x10309BC);
        let mode: u8 = *global::<u8>(0x10309C2);
        for block in [0x78usize, 0x8c, 0xa0] {
            *(this_.add(block) as *mut u32) = zero_bits;
            *(this_.add(block + 4) as *mut u32) = zero_bits;
            *(this_.add(block + 8) as *mut u32) = one_bits;
            *(this_.add(block + 0x0c) as *mut u32) = 0;
            *this_.add(block + 0x10) = 0;
            *this_.add(block + 0x11) = mode;
        }
        (answer & 0xFFFF_FF00) | 1
    }
});
