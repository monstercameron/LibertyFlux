// original: 0x009e2520 audio_tracker_revoice
/// Re-voice the tracker for a new voice/count pair: poll the
/// virtual state, tear down or retune the old voice, install the new pair,
/// and report through the pool. Returns the report answer. (thiscall/2; three
/// virtual calls go through fabricated tables like the original.)
export!(thiscall, rw_009e2520(this: *mut u8, a: u32, b: u32) -> u32 {
    unsafe {
        let link = *((this.add(0x34)) as *const u32);
        let mut flag: u8 = if link != 0 {
            (((*((link.wrapping_add(0x24)) as *const u32)) >> 27) & 1) as u8
        } else {
            0
        };
        let vt = *(this as *const u32);
        let t1 = *((vt.wrapping_add(0x10)) as *const u32);
        let f1: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(t1 as usize);
        if f1(this as u32) & 0xFF != 0 {
            let w30 = *((this.add(0x30)) as *const u32);
            let w38 = *((this.add(0x38)) as *const i32);
            flag = if w30 != 0 && w38 > 0 { 1 } else { 0 };
        }
        if a != 0 && (b as i32) > 0 {
            if flag == 0 && link != 0 {
                let lvt = *(link as *const u32);
                let t2 = *((lvt.wrapping_add(0x18)) as *const u32);
                let f2: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(t2 as usize);
                f2(link);
            }
            if *((this.add(0x30)) as *const u32) != a {
                if flag != 0 {
                    callee_thiscall!(1, u32, this as u32);
                }
            } else if b != *((this.add(0x38)) as *const u32) {
                callee_thiscall!(2, u32, this as u32, b);
            }
            callee_thiscall!(3, u32, this as u32, a, b);
            callee_thiscall!(4, u32, a, this as u32);
        } else if flag != 0 {
            callee_thiscall!(1, u32, this as u32);
            if link != 0 {
                let lvt = *(link as *const u32);
                let t3 = *((lvt.wrapping_add(0x10)) as *const u32);
                let f3: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(t3 as usize);
                f3(link);
            }
        }
        if *((this.add(0x30)) as *const u32) != 0
            && *((this.add(0x38)) as *const i32) > 0
        {
            *this.add(0x3f) = 0;
            let voice = *((this.add(0x30)) as *const u32);
            callee_thiscall!(5, u32, voice);
        }
        callee_thiscall!(6, u32, this as u32)
    }
});
