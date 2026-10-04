// original: 0x00660FC0 snGamerList_scan (proposed)

/// Scan a list of gamer pairs against the task's slots and mark the hits.
///
/// `this` is the task, `arr` points at `n` 8-byte pairs. The manager at
/// `this+0x60` must be outside states 2-3, or hold a changed gamer pair
/// (`+0xbf0`/`+0xbf4` against `+0xc30`/`+0xc34`), and `n` must be
/// positive; otherwise there is nothing to do. Each pair is compared in
/// order against the slots at `this+0x2a0` (8 bytes each, `this+0x540` of
/// them); the first slot whose two words both match is a hit, and the
/// scan moves on after marking byte `this+0x520 + slot`.
///
/// A hit whose flag word at `this+0x420 + slot*8` is 1 first resolves the
/// slot (callee 1) on an object chosen by a global switch byte (zero: a
/// null object, otherwise a fixed image object, both plus `0x10`) with
/// two scratch words and a zero; the call fills three scratch words
/// whose first selects a slow path (compared against the flag address,
/// unsigned below) that always just marks, and whose second must be a
/// live object. Otherwise the object's virtual
/// slot `+0x28` runs (callee 2): only the wanted marker reaches virtual
/// slot `+0x2c` (callee 3) with the slot address and 1.
///
/// Original: 0x00660FC0 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00660fc0(this: u32, arr: u32, n: u32) -> u32 {
    unsafe {
        const MGR: u32 = 0x60;
        const SLOTS: u32 = 0x2a0;
        const FLAGS: u32 = 0x420;
        const MARKS: u32 = 0x520;
        const NSLOTS: u32 = 0x540;
        const GSWITCH: u32 = 0x18b8309;
        const GOBJ: u32 = 0x19f3a10;
        const MARKER: u32 = 0x01c9_a0b0;
        const C_RESOLVE: u32 = 1;
        const C_VT28: u32 = 2;
        const C_VT2C: u32 = 3;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let mgr = rd32(this.wrapping_add(MGR));
        let st = rd32(mgr.wrapping_add(0x50)) as i32;
        if st >= 2 && st <= 3 {
            if rd32(mgr.wrapping_add(0xbf0)) == rd32(mgr.wrapping_add(0xc30))
                && rd32(mgr.wrapping_add(0xbf4)) == rd32(mgr.wrapping_add(0xc34))
            {
                return 0;
            }
        }
        let count = n as i32;
        if count <= 0 {
            return 0;
        }
        let mut i = 0i32;
        while i < count {
            let slots = rd32(this.wrapping_add(NSLOTS)) as i32;
            if slots > 0 {
                let mut s = 0i32;
                loop {
                    let base = arr.wrapping_add((i as u32).wrapping_mul(8));
                    let lo = rd32(base);
                    let hi = rd32(base.wrapping_add(4));
                    let sp = this
                        .wrapping_add(SLOTS)
                        .wrapping_add((s as u32).wrapping_mul(8));
                    if lo == rd32(sp) && hi == rd32(sp.wrapping_add(4)) {
                        let flagp = this
                            .wrapping_add(FLAGS)
                            .wrapping_add((s as u32).wrapping_mul(8));
                        if rd32(flagp) == 1 {
                            let gbyte =
                                lf_checker_rt::global::<u8>(GSWITCH).read();
                            let base = if gbyte == 0 {
                                0
                            } else {
                                lf_checker_rt::relocated(GOBJ)
                            };
                            let mut scratch = [0u32; 8];
                            let _: u32 = lf_checker_rt::callee_thiscall!(
                                C_RESOLVE,
                                u32,
                                base.wrapping_add(0x10),
                                scratch.as_mut_ptr() as u32,
                                (scratch.as_mut_ptr() as u32).wrapping_add(2),
                                0
                            );
                            let w3 = scratch[0];
                            let w4 = scratch[1];
                            if w4 != 0 && flagp >= w3 {
                                let slot = rd32(rd32(w4).wrapping_add(0x28));
                                let f: extern "thiscall" fn(u32) -> u32 =
                                    core::mem::transmute(slot as usize);
                                let _ = C_VT28;
                                if f(w4) == MARKER {
                                    let slot2 =
                                        rd32(rd32(w4).wrapping_add(0x2c));
                                    let f2: extern "thiscall" fn(u32, u32, u32) -> u32 =
                                        core::mem::transmute(slot2 as usize);
                                    let _ = C_VT2C;
                                    let _: u32 = f2(w4, sp, 1);
                                }
                            }
                        }
                        ((this.wrapping_add(s as u32).wrapping_add(MARKS)) as *mut u8)
                            .write(1);
                        break;
                    }
                    s += 1;
                    if s >= slots {
                        break;
                    }
                }
            }
            i += 1;
        }
        0
    }
});
