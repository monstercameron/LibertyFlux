// original: 0x0088ae30 audVoice_update_88AE30 (proposed)
/// Sweep all 192 voice records of one bank, refreshing finished voices.
///
/// `this` points to the bank object; the dword at `+0xe8` is the record
/// area base. `arg0` selects the bank (`* 0x6f40`). Each of the 192 records
/// has a flag byte (at `P + edx + index`); a 0x70-byte descriptor reached
/// through the running offset (a word at `+0xd4`, a byte at `+0xd8`, a
/// dword at `+0xc8`); and a 0x20-byte counter block reached through a
/// running pointer (a gate byte at `+0x10`, dwords at `+0`, `+4` and a flag
/// word at `+0x8`).
///
/// Per record: if flag bit 4 is set, the record is marked done (`0x80` at
/// `+0xd8`); unless the gate byte's low two bits are set, a helper resolves
/// the descriptor word (when its flag bit 2 is clear), the flag byte is
/// cleared and a shared done-counter byte at `P + edx + 0x6f0f` is bumped.
/// If flag bit 4 is clear but bit 2 is set, the record is re-armed: the bit
/// is cleared, two helpers resolve and register the descriptor word, a
/// fourth helper returns a rate block (or null), and on success the
/// descriptor word at `+0xd6` is refreshed from the block, the counter at
/// `+0` becomes the low dword of `floor(rate / (divisor * 0.001))` (zero
/// when not finite or out of 64-bit range, as the original's SSE round and
/// `fistp` produce), the counter at `+4` becomes a fifth helper's answer
/// (or all-ones when the block's secondary count is `-1`), and `+0xd8`
/// gains bit 2 when the resolved voice's byte at `+0x48` is non-zero. A
/// null rate block sets flag bits `0x1fffffff` at `+0x8` instead.
///
/// All five helpers are intercepted by the checker: three cdecl
/// (one, two and two arguments) and two thiscall with no stack arguments.
/// The value left in `eax` is the last iteration's leftover (entry garbage
/// when that iteration takes the no-op path), so the return channel is not
/// compared; see the contract.
///
/// Original: 0x0088ae30 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_0088ae30(this: u32, arg0: u32) -> u32 {
    unsafe {
        const AREA_BASE: u32 = 0xe8;
        const BANK_STRIDE: u32 = 0x6f40;
        const RECORDS: u32 = 192;
        const DESC_STRIDE: u32 = 0x70;
        const COUNT_STRIDE: u32 = 0x20;
        const COUNT_BASE_OFF: u32 = 0x54c0;
        const FLAG_DONE: u8 = 4;
        const FLAG_REARM: u8 = 2;
        const DESC_WORD: u32 = 0xd4;
        const DESC_BYTE: u32 = 0xd8;
        const DESC_PTR: u32 = 0xc8;
        const DESC_RATE: u32 = 0xd6;
        const MARK_DONE: u8 = 0x80;
        const MARK_REARMED: u8 = 4;
        const GATE_OFF: u32 = 0x10;
        const GATE_MASK: u8 = 3;
        const COUNT_A: u32 = 0x0;
        const COUNT_B: u32 = 0x4;
        const COUNT_FLAGS: u32 = 0x8;
        const NULL_FLAGS: u32 = 0x1fff_ffff;
        const DONE_TALLY: u32 = 0x6f0f;
        const RATE_NUM: u32 = 0x10;
        const RATE_AUX: u32 = 0x14;
        const RATE_DIV: u32 = 0x18;
        const RATE_SHORT: u32 = 0x1a;
        const VOICE_LIVE: u32 = 0x48;
        const CAL_RESOLVE: u32 = 1;
        const CAL_DROP: u32 = 2;
        const CAL_REGISTER: u32 = 3;
        const CAL_RATE: u32 = 4;
        const CAL_SCALE: u32 = 5;
        const MILLI: f32 = f32::from_bits(0x3a83_126f); // 0.001
        const TWO63_F: f32 = 9223372036854775808.0; // 2^63

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        /// Low dword of the original's `fistp qword` of an integral float.
        #[inline(always)]
        fn fistp_low(x: f32) -> u32 {
            if x.is_finite() && x < TWO63_F && x >= -TWO63_F {
                (x as i64) as u32
            } else {
                0
            }
        }

        let bank = arg0.wrapping_mul(BANK_STRIDE);
        let mut cnt = 0u32;
        while cnt < RECORDS {
            let base = rd32(this.wrapping_add(AREA_BASE));
            let row = bank.wrapping_add(base);
            let desc = row.wrapping_add(cnt.wrapping_mul(DESC_STRIDE));
            let blk = bank
                .wrapping_add(COUNT_BASE_OFF)
                .wrapping_add(base)
                .wrapping_add(cnt.wrapping_mul(COUNT_STRIDE));
            let flag_at = row.wrapping_add(cnt);
            let flag = rd8(flag_at);
            if flag & FLAG_DONE != 0 {
                let db = desc.wrapping_add(DESC_BYTE);
                wr8(db, rd8(db) | MARK_DONE);
                if rd8(blk.wrapping_add(GATE_OFF)) & GATE_MASK != 0 {
                    cnt = cnt.wrapping_add(1);
                    continue;
                }
                if rd8(flag_at) & FLAG_REARM == 0 {
                    let key = rd16(desc.wrapping_add(DESC_WORD)) as i16 as i32 as u32;
                    let v = lf_checker_rt::callee_cdecl!(CAL_RESOLVE, u32, key);
                    lf_checker_rt::callee_thiscall!(CAL_DROP, u32, v);
                }
                wr8(flag_at, 0);
                let tally = row.wrapping_add(DONE_TALLY);
                wr8(tally, rd8(tally).wrapping_add(1));
            } else if flag & FLAG_REARM != 0 {
                wr8(flag_at, flag & !FLAG_REARM);
                let key = rd16(desc.wrapping_add(DESC_WORD)) as i16 as i32 as u32;
                let v = lf_checker_rt::callee_cdecl!(CAL_RESOLVE, u32, key);
                lf_checker_rt::callee_thiscall!(CAL_REGISTER, u32, v);
                let mem = rd32(desc.wrapping_add(DESC_PTR));
                let rate = lf_checker_rt::callee_cdecl!(CAL_RATE, u32, v, mem);
                if rate == 0 {
                    let cf = blk.wrapping_add(COUNT_FLAGS);
                    wr32(cf, rd32(cf) | NULL_FLAGS);
                } else {
                    wr16(desc.wrapping_add(DESC_RATE), rd16(rate.wrapping_add(RATE_SHORT)));
                    let num = rd32(rate.wrapping_add(RATE_NUM));
                    let den = rd16(rate.wrapping_add(RATE_DIV));
                    let f1 = div(num as f32, mul(den as f32, MILLI));
                    wr32(blk.wrapping_add(COUNT_A), fistp_low(f1.floor()));
                    let aux = rd32(rate.wrapping_add(RATE_AUX));
                    let scaled = if aux == 0xffff_ffff {
                        0xffff_ffff
                    } else {
                        lf_checker_rt::callee_cdecl!(CAL_SCALE, u32, aux, den as u32)
                    };
                    wr32(blk.wrapping_add(COUNT_B), scaled);
                    if rd8(v.wrapping_add(VOICE_LIVE)) != 0 {
                        let db = desc.wrapping_add(DESC_BYTE);
                        wr8(db, rd8(db) | MARK_REARMED);
                    }
                }
            }
            cnt = cnt.wrapping_add(1);
        }
        0
    }
});
