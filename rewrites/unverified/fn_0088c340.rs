// original: 0x0088c340 rage::audVoiceDSound::vf3
/// Recompute one voice's loop length and notify its channel.
///
/// `this` points to the voice object. When flag bit 2 at `+0x8c` is set and
/// the dword at `+0xa0` is non-zero, a helper resolves the new length from
/// the incoming position `arg0` and the limit at `+0xb0`: positions at or
/// past the limit resolve directly and double, earlier positions resolve
/// and combine as `limit-word + 2 * answer - base` with the words at
/// `+0xc0` and `+0xa0`. Otherwise the length is the low dword of
/// `floor(rate * arg0 * 0.001)` (zero when not finite or out of 64-bit
/// range, as the original's SSE round and `fistp` produce), doubled, with
/// the rate at `+0xc`.
///
/// When flag bit `0x10` is set, a second helper runs first (thiscall, single
/// argument `1`). The length is then delivered to the channel object at
/// `+0x90` through its slot at `+0x34` (two stack arguments, callee
/// cleanup), a third helper runs with the float at `[[+0x4]]` (thiscall),
/// flag bit 8 is set at `+0x8c`, and the third helper's answer is returned.
///
/// Original: 0x0088c340 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_0088c340(this: u32, arg0: u32) -> u32 {
    unsafe {
        const FLAG_BYTE: u32 = 0x8c;
        const USE_HELPER: u8 = 2;
        const NOTIFY_FIRST: u8 = 0x10;
        const MARK_DONE: u8 = 8;
        const BASE_WORD: u32 = 0xa0;
        const LIMIT_WORD: u32 = 0xb0;
        const RATE_WORD: u32 = 0xc;
        const COMBINE_WORD: u32 = 0xc0;
        const CHANNEL_OBJ: u32 = 0x90;
        const CHANNEL_SLOT: u32 = 0x34;
        const PARAM_PTR: u32 = 0x4;
        const CAL_RESOLVE: u32 = 1;
        const CAL_NOTIFY: u32 = 2;
        // The channel slot (+0x34) is callee id 3 in the contract, reached
        // through the planted vtable exactly like the original.
        const CAL_FINISH: u32 = 4;
        const MILLI: f32 = f32::from_bits(0x3a83_126f); // 0.001
        const TWO63_F: f32 = 9223372036854775808.0; // 2^63

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
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

        let flags = (this.wrapping_add(FLAG_BYTE) as *const u8).read();
        let base = rd32(this.wrapping_add(BASE_WORD));
        let edi = if flags & USE_HELPER != 0 && base != 0 {
            let lim = rd32(this.wrapping_add(LIMIT_WORD));
            let rate = rd32(this.wrapping_add(RATE_WORD));
            if arg0 >= lim {
                let v = lf_checker_rt::callee_cdecl!(CAL_RESOLVE, u32, arg0.wrapping_sub(lim), rate);
                v.wrapping_add(v)
            } else {
                let v = lf_checker_rt::callee_cdecl!(CAL_RESOLVE, u32, arg0, rate);
                rd32(this.wrapping_add(COMBINE_WORD))
                    .wrapping_add(v.wrapping_mul(2))
                    .wrapping_sub(base)
            }
        } else {
            let rate = rd32(this.wrapping_add(RATE_WORD));
            let h = mul(mul(rate as f32, arg0 as f32), MILLI);
            let e = fistp_low(h.floor());
            e.wrapping_add(e)
        };
        if flags & NOTIFY_FIRST != 0 {
            lf_checker_rt::callee_thiscall!(CAL_NOTIFY, u32, this, 1u32);
        }
        let obj = rd32(this.wrapping_add(CHANNEL_OBJ));
        let vt = rd32(obj);
        let slot = rd32(vt.wrapping_add(CHANNEL_SLOT));
        let channel: extern "stdcall" fn(u32, u32) -> u32 = core::mem::transmute(slot as usize);
        channel(obj, edi);
        let param_at = rd32(this.wrapping_add(PARAM_PTR));
        let fval = rd32(param_at);
        let r = lf_checker_rt::callee_thiscall!(CAL_FINISH, u32, this, fval);
        let fb = this.wrapping_add(FLAG_BYTE) as *mut u8;
        fb.write(fb.read() | MARK_DONE);
        r
    }
});
