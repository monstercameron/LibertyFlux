// original: 0x0089d7e0 audio_voice_probe_3slot
/// Voice probe over three sub-slots of an audio object (original 0x0089D7E0).
///
/// Resolves the voice for the first sub-slot and, when present, issues a
/// probe call for each of the three sub-slots that resolves. Returns 2 when
/// no voice can be resolved or a probe reports missing, otherwise 1 when
/// every probe answered and 0 when any probe came back empty.
///
/// Note: the original pushes its flag byte as a dword from a stack slot
/// whose upper bytes it never wrote; the contract defines that fill as
/// zero, so the rewrite passes the flag value directly.
export!(thiscall, rw_89d7e0(this: *mut u8, a1: u32, a2: u32) -> u32 {
    unsafe {
        let base: u32 = *global::<u32>(0x115d988);
        let stride: u32 = *global::<u32>(0x115d964);
        let sel = (*this.add(0x40)) as u32;
        let row_at = base
            .wrapping_add(sel.wrapping_mul(0x6f40))
            .wrapping_add(0x6f10);
        let row = *(row_at as *const u32);
        let s0 = *this.add(0x48);
        if s0 == 0xff {
            return 2;
        }
        let v0 = row.wrapping_add(stride.wrapping_mul(s0 as u32));
        if v0 == 0 {
            return 2;
        }
        let p54 = *((this.add(0x54)) as *const u32);
        let _: u32 = callee_thiscall!(1, u32, v0, p54, 0);
        let flag = (((*this.add(0x39)) >> 5) & 1) as u32;
        let mut ok: u32 = 1;
        for k in 0..3u32 {
            let s = *this.add((0x48 + k) as usize);
            if s == 0xff {
                continue;
            }
            let stride_k: u32 = *global::<u32>(0x115d964);
            let row_k = *(row_at as *const u32);
            let v = row_k.wrapping_add(stride_k.wrapping_mul(s as u32));
            if v == 0 {
                continue;
            }
            let r: u32 = callee_thiscall!(2, u32, v, a1, flag, a2);
            if r == 2 {
                return 2;
            }
            if r == 0 {
                ok = 0;
            }
        }
        ok
    }
});
