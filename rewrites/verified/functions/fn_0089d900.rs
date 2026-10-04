// original: 0x0089d900 audio_voice_setup_4phase
/// Four-phase voice setup on an audio object (original 0x0089D900).
///
/// Runs up to four guarded phases (latch, primary, secondary, tertiary),
/// each resolving a voice-table entry for its sub-slot and issuing setup
/// calls when the entry is live. Returns 1 when any phase reports a live
/// voice, otherwise tears down through the release callee and returns 0.
/// The low return byte carries the verdict; the upper bytes carry the
/// last call result.
export!(thiscall, rw_89d900(this: *mut u8, a1: u32) -> u32 {
    unsafe {
        let base: u32 = *global::<u32>(0x115d988);
        let stride: u32 = *global::<u32>(0x115d964);
        let sel = (*this.add(0x40)) as u32;
        let row_at = base
            .wrapping_add(sel.wrapping_mul(0x6f40))
            .wrapping_add(0x6f10);
        // Mirrors the original's eax: every call result and every voice
        // computation lands here, because the tail returns whatever eax
        // happens to hold (low byte replaced). Initialized like the
        // original's entry eax only matters on paths that never write it,
        // which always end at the release call below.
        let mut ex: u32 = 0;
        let mut fe: u8 = 0;
        let mut ff: u8 = 0;
        if (*this.add(0x39) & 4) != 0 && *this.add(0xb0) == 0 {
            *((this.add(0xb0)) as *mut u16) = 0x101;
            let sa = *this.add(0x49);
            if sa != 0xff {
                let row = *(row_at as *const u32);
                let va = row.wrapping_add(stride.wrapping_mul(sa as u32));
                ex = va;
                if va != 0 {
                    let ra: u32 = callee_thiscall!(1, u32, this as u32, 1, a1);
                    ex = ra;
                    ex = callee_thiscall!(3, u32, ra);
                }
            } else {
                ex = 0xff;
            }
        }
        let sb = *this.add(0x48);
        if sb != 0xff {
            let row = *(row_at as *const u32);
            let vb = row.wrapping_add(stride.wrapping_mul(sb as u32));
            ex = vb;
            if vb != 0 {
                let wb = *((vb.wrapping_add(6)) as *const u16);
                if wb == 2 {
                    let rb: u32 = callee_thiscall!(4, u32, vb, a1);
                    ex = rb;
                    fe = rb as u8;
                    if fe == 0 {
                        *this.add(0xb2) = 1;
                        let rb2: u32 = callee_thiscall!(2, u32, this as u32, 2);
                        ex = rb2;
                        if rb2 != 0 {
                            let rb3: u32 = callee_thiscall!(1, u32, this as u32, 2, a1);
                            ex = rb3;
                            ex = callee_thiscall!(3, u32, rb3);
                        }
                    }
                }
            }
        }
        let sc = *this.add(0x49);
        if sc != 0xff {
            let row = *(row_at as *const u32);
            let vc = row.wrapping_add(stride.wrapping_mul(sc as u32));
            ex = vc;
            if vc != 0 && *this.add(0xb1) != 0 {
                let wc = *((vc.wrapping_add(6)) as *const u16);
                if wc == 2 {
                    let rc1: u32 = callee_thiscall!(1, u32, this as u32, 1, 0x80000000);
                    ex = rc1;
                    ex = callee_thiscall!(7, u32, rc1);
                    let rc2: u32 = callee_thiscall!(1, u32, this as u32, 1, a1);
                    ex = rc2;
                    ex = callee_thiscall!(5, u32, rc2);
                    ff = ex as u8;
                }
            }
        }
        // The original's third flag lives in a stack slot it zeroed and
        // never wrote again; only a live tertiary phase sets al_tail.
        // A skipped tertiary phase instead clears eax's low byte.
        let mut al_tail: u8 = 0;
        let mut d_full = false;
        let sd = *this.add(0x4a);
        if sd != 0xff {
            let row = *(row_at as *const u32);
            let vd = row.wrapping_add(stride.wrapping_mul(sd as u32));
            ex = vd;
            if vd != 0 && *this.add(0xb2) != 0 {
                let wd = *((vd.wrapping_add(6)) as *const u16);
                if wd == 2 {
                    let rd1: u32 = callee_thiscall!(1, u32, this as u32, 2, 0x80000000);
                    ex = rd1;
                    ex = callee_thiscall!(7, u32, rd1);
                    let rd2: u32 = callee_thiscall!(1, u32, this as u32, 2, a1);
                    ex = rd2;
                    ex = callee_thiscall!(6, u32, rd2);
                    al_tail = ex as u8;
                    d_full = true;
                }
            }
        }
        if !d_full {
            ex &= 0xffffff00;
        }
        if fe != 0 || ff != 0 || al_tail != 0 {
            return (ex & 0xffffff00) | 1;
        }
        ex = callee_thiscall!(8, u32, this as u32);
        ex & 0xffffff00
    }
});
