// original: 0x00ca6ae0 CEventHandler::vf66
/// Event reaction 66: scale the owner's speed bucket and commit a timed
/// behaviour, or take a fixed fallback when the owner is not ready.
///
/// Readiness needs the flag set, a zero poll answer and a zero check
/// answer. On the main path a virtual gate decides a flag bit (cleared
/// when the kind word reads 2); that bit travels in the low byte of a
/// word holding the owner's own address. The speed bucket (a 16-bit
/// word scaled by a fixed factor, always in integer range) becomes the
/// duration argument. The committed handle (or zero) lands at owner+0xc.
export!(thiscall, rw_rb02_vf66(this_ptr: u32, _a1: u32, _a2: u32, _a3: u32) -> u32 {
    unsafe {
        let p1 = *((this_ptr.wrapping_add(4)) as *const u32);
        let a1: u32 = callee_thiscall!(1, u32, p1);
        if a1 == 0 {
            return 0;
        }
        let ans2: u32 = callee_thiscall!(2, u32, a1.wrapping_add(8));
        if ans2 == 0 {
            return 0;
        }
        if ans2 == p1 {
            return p1;
        }
        let p2 = *((p1.wrapping_add(0x224)) as *const u32);
        if *((p2.wrapping_add(0x50)) as *const u32) != 0 {
            return p2;
        }
        // Alternate form unless every readiness check clears.
        let mut alt = false;
        if (*((p1.wrapping_add(0x26c)) as *const u8) & 4) == 0 {
            alt = true;
        } else {
            let c3: u32 = callee_thiscall!(3, u32, p2.wrapping_add(0x44), 0x2cau32);
            if c3 != 0 {
                alt = true;
            } else {
                let c4: u32 = callee_thiscall!(4, u32, p2.wrapping_add(0x2e0), 0x2e2u32, 0u32);
                if (c4 & 0xff) != 0 {
                    alt = true;
                }
            }
        }
        if !alt {
            let vt = *(ans2 as *const u32);
            let tgt = *((vt.wrapping_add(0x128)) as *const u32);
            let gate: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(tgt as usize);
            let mut bit = 1u32;
            if (gate(ans2) & 0xff) == 0 {
                let o = *((p1.wrapping_add(0x21c)) as *const u32);
                bit = if *((o.wrapping_add(0x12c)) as *const u32) == 2 {
                    0
                } else {
                    1
                };
            }
            let bus = *global::<u32>(0x167e2a0);
            let mgr: u32 = callee_thiscall!(5, u32, bus);
            if mgr == 0 {
                *((this_ptr.wrapping_add(0xc)) as *mut u32) = 0;
                return 0;
            }
            // Low byte is the gate bit; the rest echoes the owner word,
            // exactly as the original's byte-store-then-word-push does.
            let w = (this_ptr & 0xffffff00) | bit;
            let bucket = *((p1.wrapping_add(0x2c)) as *const u16) as f32;
            let scaled = bucket * *global::<f32>(0xed7ca4);
            // Non-negative factor times a u16: always in i32 range, so a
            // plain truncation matches the original's convert instruction.
            let n = (scaled as i32) as u32;
            let ans: u32 = callee_thiscall!(6, u32, mgr, n, 0u32, 0u32, w);
            *((this_ptr.wrapping_add(0xc)) as *mut u32) = ans;
            return ans;
        }
        let c7: u32 = callee_thiscall!(7, u32, p2.wrapping_add(0x44), 0x2deu32);
        if c7 == 0 {
            return 0;
        }
        let bus2 = *global::<u32>(0x167e2a0);
        let mgr2: u32 = callee_thiscall!(5, u32, bus2);
        if mgr2 == 0 {
            *((this_ptr.wrapping_add(0xc)) as *mut u32) = 0;
            return 0;
        }
        let ans: u32 = callee_thiscall!(8, u32, mgr2, 1u32, 0u32);
        *((this_ptr.wrapping_add(0xc)) as *mut u32) = ans;
        ans
    }
});
