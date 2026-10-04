// original: 0x0089e500 audio_queue_advance
/// Queue advance on an audio object (original 0x0089E500).
///
/// Retires the pending slot recorded in the object, then advances the
/// play cursor: the current entry is validated through the resolver
/// callee, and on success the cursor moves on and the entered and left
/// entries are announced. Returns 1 on success, 0 when the cursor cannot
/// advance. Only the low return byte is defined; the upper bytes carry
/// whatever the last call (or the incoming registers) left behind.
///
/// Note: the original pushes its flag byte as a dword over the register
/// it saved on entry, so the upper bytes of that call argument always
/// equal the incoming object pointer with a zero low byte; the rewrite
/// computes the same word directly.
export!(thiscall, rw_89e500(this: *mut u8, a1: u32) -> u32 {
    unsafe {
        let base: u32 = *global::<u32>(0x115d988);
        let stride: u32 = *global::<u32>(0x115d964);
        let sel = (*this.add(0x40)) as u32;
        let row_at = base
            .wrapping_add(sel.wrapping_mul(0x6f40))
            .wrapping_add(0x6f10);
        let idx0 = *this.add(0xd0);
        if idx0 != 0xff {
            let flag = (((*this.add(0x39)) >> 5) & 1) as u32;
            let w = *((this.add(0x3c)) as *const i16) as i32 as u32;
            let r: u32 = callee_cdecl!(1, u32, w);
            let sv = *this.add((0x48u32 + idx0 as u32) as usize);
            let vout: u32 = if sv == 0xff {
                0
            } else {
                let row = *(row_at as *const u32);
                row.wrapping_add(stride.wrapping_mul(sv as u32))
            };
            let flag_word = ((this as u32) & 0xffffff00) | flag;
            let r2: u32 = callee_thiscall!(2, u32, vout, r, flag_word, 0);
            if r2 != 1 {
                let _: u32 = callee_thiscall!(3, u32, this as u32, idx0 as u32);
            }
            *this.add(0xd0) = 0xff;
        }
        let edi = *((this.add(0xcc)) as *const i32);
        if edi < 0 {
            return 0;
        }
        let c8 = *((this.add(0xc8)) as *const i32);
        if edi >= c8 {
            return 0;
        }
        // Note: the original folds the parity reduction back into the
        // cursor register, so every later use below sees the reduced value.
        let mut e = edi;
        if *this.add(0xd2) != 0 {
            e = edi % 2;
        }
        let cur0 = edi;
        let edi = e;
        let v = *this.add((0x48 + e) as usize);
        if v != 0xff {
            let row = *(row_at as *const u32);
            let w2 = row.wrapping_add(stride.wrapping_mul(v as u32));
            if w2 != 0 {
                let r4: u32 = callee_thiscall!(4, u32, this as u32, edi as u32, a1);
                let r5: u32 = callee_thiscall!(5, u32, r4);
                if (r5 & 0xFF) != 0 {
                    return 1;
                }
            }
        }
        if (*this.add(0x39) & 8) != 0 {
            return 0;
        }
        let ccnew = cur0.wrapping_add(1);
        *((this.add(0xcc)) as *mut i32) = ccnew;
        let mut ebx = ccnew;
        if *this.add(0xd2) != 0 {
            ebx = ccnew % 2;
        }
        if c8 <= ccnew {
            return 0;
        }
        let r6: u32 = callee_thiscall!(6, u32, this as u32, ebx as u32);
        if r6 == 0 {
            return 0;
        }
        let r7: u32 = callee_thiscall!(9, u32, this as u32, ebx as u32);
        if r7 != 0 {
            let r8: u32 = callee_thiscall!(4, u32, this as u32, ebx as u32, a1);
            let _: u32 = callee_thiscall!(7, u32, r8);
        }
        let _: u32 = callee_thiscall!(6, u32, this as u32, edi as u32);
        let _: u32 = callee_thiscall!(3, u32, this as u32, edi as u32);
        let _: u32 = callee_thiscall!(8, u32, this as u32, edi as u32);
        1
    }
});
