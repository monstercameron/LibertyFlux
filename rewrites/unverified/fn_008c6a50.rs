// original: 0x008C6A50 stream_slots_sweep
/// Sweep all fourteen streaming slots, staging and publishing each one.
///
/// Copies every flagged slot's 8-byte entry into a scratch staging area
/// (flag at +0xF0 stages through mark one, flag at +0xFE through mark
/// two, either byte copy stopping at the first NUL), runs the clear
/// callee, then publishes each staged entry whose mark is set through
/// the slot callee (mark one) or the alt callee (mark two). Returns the
/// last callee answer. Original: thiscall, no stack words.
lf_checker_rt::export!(thiscall, rw_008c6a50(this: u32) -> u32 {
    unsafe {
        const CLEAR_CALLEE: u32 = 1;
        const SLOT_CALLEE: u32 = 2;
        const ALT_CALLEE: u32 = 3;
        const COOKIE_CALLEE: u32 = 4;
        const SLOT_COUNT: u32 = 14;
        const FLAG_A: u32 = 0xF0;
        const FLAG_B: u32 = 0xFE;
        const ENTRY_BASE: u32 = 0x1EC;
        const ENTRY_STRIDE: u32 = 8;
        let mut stage = [0u8; 112];
        let mut mark_a = [0u8; 14];
        let mut mark_b = [0u8; 14];
        let mut k: u32 = 0;
        while k < SLOT_COUNT {
            let src =
                this.wrapping_add(k.wrapping_mul(ENTRY_STRIDE))
                    .wrapping_add(ENTRY_BASE);
            let dst = stage.as_mut_ptr().add((k * 8) as usize);
            if ((this + k + FLAG_A) as *const u8).read() != 0 {
                mark_a[k as usize] = 1;
                let mut j: u32 = 0;
                loop {
                    let b = ((src + j) as *const u8).read();
                    *dst.add(j as usize) = b;
                    if b == 0 {
                        break;
                    }
                    j += 1;
                }
            }
            if ((this + k + FLAG_B) as *const u8).read() != 0 {
                mark_b[k as usize] = 1;
                let mut j: u32 = 0;
                loop {
                    let b = ((src + j) as *const u8).read();
                    *dst.add(j as usize) = b;
                    if b == 0 {
                        break;
                    }
                    j += 1;
                }
            }
            k += 1;
        }
        let mut last: u32 =
            lf_checker_rt::callee_thiscall!(CLEAR_CALLEE, u32, this, 0);
        let mut i: u32 = 0;
        while i < SLOT_COUNT {
            let entry =
                stage.as_mut_ptr().add((i * 8) as usize) as u32;
            if mark_a[i as usize] != 0 {
                last = lf_checker_rt::callee_thiscall!(
                    SLOT_CALLEE, u32, this, i, entry);
            }
            if mark_b[i as usize] != 0 {
                last = lf_checker_rt::callee_thiscall!(
                    ALT_CALLEE, u32, this, i, entry);
            }
            i += 1;
        }
        lf_checker_rt::callee_cdecl!(COOKIE_CALLEE, u32,);
        last
    }
});
