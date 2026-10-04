// original: 0x008900F0 
// 008900F0 audSound params apply: fold the randomized words into the output
// fields, scaling the first pair by the fixed factor.
export!(thiscall, rw_008900f0(this: *mut u8, inp: *const u8, _unused: u32) -> () {
    unsafe {
        let w = |off: usize| *(inp.add(off) as *const u16);
        let coef = *global::<f32>(0xFE870C);
        let jitter = |v: u32| callee_cdecl!(1, u32, v.wrapping_neg(), v);
        let wa = w(0x11);
        let ra = if wa != 0 { jitter(wa as u32) } else { 0 };
        *(this.add(0x1C) as *mut f32) =
            (ra as i32 as f32) * coef + (w(0xF) as i16 as f32) * coef;
        let wb = w(0x15);
        let rb = if wb != 0 { jitter(wb as u32) } else { 0 };
        *(this.add(0x20) as *mut u16) = w(0x13).wrapping_add(rb as u16);
        if w(0x17) as i16 == -1 {
            *(this.add(0xA) as *mut u16) = 0xFFFF;
        } else {
            let wc = w(0x19);
            let rc = if wc != 0 { jitter(wc as u32) } else { 0 };
            let n = (rc as u16 as i16 as i32) + (w(0x17) as i16 as i32) + 0x168;
            *(this.add(0xA) as *mut u16) = (n % 0x168) as u16;
        }
        let wd = w(0x1D);
        let rd = if wd != 0 { jitter(wd as u32) } else { 0 };
        *(this.add(0x58) as *mut u32) = (w(0x1B) as u32).wrapping_add(rd);
        let e = *(inp.add(0x23) as *const u32);
        let base = *(inp.add(0x1F) as *const u32);
        *(this.add(0x68) as *mut u32) = if e == 0 {
            base
        } else {
            base.wrapping_add(jitter(e))
        };
    }
});
