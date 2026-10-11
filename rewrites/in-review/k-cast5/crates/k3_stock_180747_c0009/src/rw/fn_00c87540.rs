// original: 0x00c87540 audio_meter_color (proposed)
///
/// Refreshes the meter colour word at `this+0xf70` from a global meter
/// table. Row `idx` (unsigned dword at file global 0x1174790) starts at
/// file global 0x15e8910 + idx * 0x210; its first three bytes are colour
/// channels (low byte first). Each channel is scaled by the file
/// constants (1/255), then 2.5, then 255.0 (all in the original's order),
/// truncated toward zero, raised by 0x40 (16-bit wraparound) and clamped
/// to 0xFF from above only (unsigned compare; a wrapped-around small
/// value clamps too). The three bytes replace bits 0..23 of the old word,
/// keeping the top byte. No return value. Thiscall, no stack arguments.

lf_checker_rt::export!(thiscall, rw_00c87540(this: u32) -> u32 {
    unsafe {
    #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
    #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
    #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        const IDX: u32 = 0x1174790;
        const TABLE: u32 = 0x15e8910;
        const PITCH: u32 = 0x210;
        const C1: f32 = f32::from_bits(0x3b808081); // 1/255
        const C2: f32 = 2.5;
        const C3: f32 = 255.0;
        const BIAS: u16 = 0x40;
        const MAXC: u32 = 0xff;
        let idx = rd32(lf_checker_rt::relocated(IDX));
        let row = lf_checker_rt::relocated(TABLE).wrapping_add(idx.wrapping_mul(PITCH));
        let px = rd32(row);
        let ch = [px & 0xff, (px >> 8) & 0xff, (px >> 16) & 0xff];
        // note: channel order in the original is R(high byte), G, B(low);
        // the packing below reorders to the same final word.
        let mut out = [0u32; 3];
        for i in 0..3 {
            let f = fmul(fmul(fmul(ch[2 - i] as f32, C1), C2), C3);
            let v = ((f as i32 as u16).wrapping_add(BIAS)) as u32;
            out[i] = if v > MAXC { MAXC } else { v };
        }
        let old = rd32(this.wrapping_add(0xf70));
        let word = (old & 0xff00_0000) | (out[0] << 16) | (out[1] << 8) | out[2];
        wr32(this.wrapping_add(0xf70), word);
        0
    }
});
