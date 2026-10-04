// original: 0x008abfb0 rage::audBiquadFilterEffect::vf1
/// Second-stage initialiser for a biquad filter effect.
///
/// Runs the shared base initialiser (callee 1) with the two forwarded
/// arguments. When the base answer's low byte is zero the object is left
/// untouched and the answer is returned as is. Otherwise the default
/// coefficient block (four coefficients, a flag word and a mode byte, read
/// from the game's default table) is stamped into each of the three
/// per-channel parameter blocks, and the stored flag word with its low byte
/// forced to 1 is returned.
export!(thiscall, rw_008abfb0(this: *mut u8, a: u32, b: u32) -> u32 {
    unsafe {
        let answer: u32 = callee_thiscall!(1, u32, this as u32, a, b);
        if answer & 0xFF == 0 {
            return answer;
        }
        let c0 = *global::<u32>(0x0103086C);
        let c1 = *global::<u32>(0x01030878);
        let c2 = *global::<u32>(0x01030884);
        let c3 = *global::<u32>(0x01030890);
        let flags = *global::<u32>(0x01030860);
        let mode = *global::<u8>(0x01030896);
        for ch in 0..3u32 {
            let blk = this.add((0x78 + ch * 0x18) as usize);
            *(blk.add(0x00) as *mut u32) = c0;
            *(blk.add(0x04) as *mut u32) = c1;
            *(blk.add(0x08) as *mut u32) = c2;
            *(blk.add(0x0C) as *mut u32) = c3;
            *(blk.add(0x10) as *mut u32) = flags;
            *blk.add(0x14) = mode;
        }
        (flags & 0xFFFF_FF00) | 1
    }
});
