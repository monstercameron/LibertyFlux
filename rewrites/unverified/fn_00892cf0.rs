// original: 0x00892CF0 audsound_match_voice_probed
/// Classifies the voice request under the lock, gated by a probe comparison.
///
/// Takes the lock callee (cdecl, the dword at `this+0x38`). Computes the
/// gate bit: 1 when the enable byte at `this+0x4d` is clear, otherwise calls
/// the probe callee (thiscall, one stack word: `v`) and takes 0 when its
/// answer is NEGATIVE (signed); when the dword at `this+0x6c` is negative
/// (signed) calls the probe again with `this+0x68` and takes 1 when the two
/// answers are equal, otherwise takes 1 when `this+0x6c` equals the first
/// answer. Then classifies like the unprobed matcher: 0 when both flag bytes
/// at 0x47/0x46 are clear, `w` equals the word at 0x42 and the gate is set;
/// 1 when (`w` equals the word at 0x42, or the word at 0x40 after missing
/// 0x42) and the gate is set; 2 when `w` equals the word at 0x44; 3
/// otherwise; but 2 when both dwords at 0x94 and 0x9c are nonzero regardless.
/// Releases the lock on every path and returns the class (0-3).
/// Original: 0x00892CF0 (thiscall, two stack words: w, v).
export!(thiscall, rw_00892CF0(this: *mut u8, w: u32, v: u32) -> u32 {
    unsafe {
        const LOCK: u32 = 1;
        const UNLOCK: u32 = 2;
        const PROBE: u32 = 3;
        const MUTEX: usize = 0x38;
        let m = *(this.add(MUTEX) as *const u32);
        let _: u32 = callee_cdecl!(LOCK, u32, m);
        let gate = if *this.add(0x4d) == 0 {
            1u32
        } else {
            let a1: u32 = callee_thiscall!(PROBE, u32, this as u32, v);
            if (a1 as i32) < 0 {
                0
            } else {
                let g = *(this.add(0x6c) as *const u32);
                if (g as i32) < 0 {
                    let a2: u32 =
                        callee_thiscall!(PROBE, u32, this as u32, *(this.add(0x68) as *const u32));
                    if a2 == a1 { 1 } else { 0 }
                } else if g == a1 {
                    1
                } else {
                    0
                }
            }
        };
        let w = w as u16;
        let w42 = *(this.add(0x42) as *const u16);
        let mut edi = 3u32;
        if *this.add(0x47) == 0 && *this.add(0x46) == 0 && w == w42 && gate != 0 {
            edi = 0;
        } else if (w == w42 || w == *(this.add(0x40) as *const u16)) && gate != 0 {
            edi = 1;
        } else if w == *(this.add(0x44) as *const u16) {
            edi = 2;
        }
        if *(this.add(0x94) as *const u32) != 0 && *(this.add(0x9c) as *const u32) != 0 {
            edi = 2;
        }
        let _: u32 = callee_cdecl!(UNLOCK, u32, m);
        edi
    }
});
