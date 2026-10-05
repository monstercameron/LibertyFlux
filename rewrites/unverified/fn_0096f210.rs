// original: 0x0096f210 audio_update_large (proposed)

/// Run the large audio update: rescale two voices, seed a filter bank from
/// thread-local listener data and one x87 entry value, then configure the
/// output chain.
///
/// `this` points to the voice bank. One x87 entry value (ST0, a double)
/// selects the bank rows: it is truncated toward zero to an integer
/// (NaN or out of int64 range gives the indefinite value, whose low word
/// is 0) and split by an unsigned magic division into `rem` (`%120`) and
/// `quo` (`/120`). A 9×9 loop then fills `[this+0x2788]` (81 floats) from
/// the table helper (stdcall, `(R-4+s, Q-d+4)`), while two int→double→
/// float conversions (with the unsigned-adjust table) form signed filter
/// seeds `f14b`/`f1cb` and two flag words from their signs (strictly-above
/// wins; unordered takes the not-above side).
///
/// Four scalar-helper calls (thiscall, int args `0x28`, `0x27/0x29`,
/// flag, `flag±1`), three 5-word combiner calls (cdecl) chained through
/// two reused slots, and the filter helper (thiscall, ST0 float answers)
/// feed four mix-helper calls whose answers drive a 4-iteration output
/// loop. Two global flag bytes then select one of three gain paths,
/// including a thread-table lookup answered by a range callback (cdecl,
/// al answer); a state query (thiscall, al answer plus one out-byte
/// through a frame slot) selects the final mix constant. The tail pushes
/// the results through four fixed-object setters and returns the last
/// setter's answer. A standard stack-cookie round-trip (`G^esp` stored,
/// `stored^esp == G` checked) guards the frame; the check preserves all
/// registers. Float operation order is the original's (SSE scalar, x87
/// truncation, one exact int→double→float chain per seed).
///
/// Original: 0x0096f210 (thiscall, no stack arguments, plain return).
lf_checker_rt::export!(thiscall, rw_0096f210(this: u32) -> u32 {
    unsafe {
        const G_V0: u32 = 0x01037a30;
        const G_V1A: u32 = 0x01037a3c;
        const G_V1B: u32 = 0x01037a38;
        const G_MILLI: u32 = 0x00fe86b4;
        const G_TLSIDX: u32 = 0x017aba14;
        const TLS_TABLE: u32 = 0x0115e3f0;
        const TLS_TABLE2: u32 = 0x0115df20;
        const G_DBLTAB: u32 = 0x00fe8f50;
        const G_D60: u32 = 0x00fe8b80;
        const G_D05: u32 = 0x00fe8830;
        const G_D50: u32 = 0x00fe8b68;
        const G_D002: u32 = 0x00fe8734;
        const G_ABS: u32 = 0x00fe8f80;
        const G_K34: u32 = 0x01037a34;
        const G_GLOB: u32 = 0x011618fc;
        const G_FLAG1: u32 = 0x010379d2;
        const G_FLAG2: u32 = 0x01218490;
        const G_HALF: u32 = 0x010379a8;
        const G_ONE: u32 = 0x00fe88e8;
        const G_K40: u32 = 0x01037a40;
        const G_COOKIE: u32 = 0x01057fb4;
        const OUT_OBJ: u32 = 0x0115def0;
        const MAGIC120: u64 = 0x88888889;
        const C_VOICE: u32 = 1;
        const C_PREFILTER: u32 = 2;
        const C_TABLE: u32 = 3;
        const C_SCALAR: u32 = 4;
        const C_COMBINE: u32 = 5;
        const C_FILT: u32 = 6;
        const C_MIXIN: u32 = 7;
        const C_MIX: u32 = 8;
        const C_RANGE: u32 = 9;
        const C_STATE: u32 = 10;
        const C_SET_A: u32 = 11;
        const C_SET_B: u32 = 12;
        const C_SET_C: u32 = 13;
        const C_SET_D: u32 = 14;
        const C_COOKIE: u32 = 15;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
        }
        #[inline(always)]
        unsafe fn rd64(a: u32) -> u64 {
            unsafe { (a as *const u64).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn g32(va: u32) -> f32 {
            unsafe { f32::from_bits((lf_checker_rt::global::<u32>(va) as *const u32).read()) }
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn filt(obj: u32, x: f32) -> f32 {
            unsafe { lf_checker_rt::callee_thiscall!(C_FILT, f32, obj, x.to_bits()) }
        }
        #[inline(always)]
        unsafe fn voice(obj: u32, a0: f32, a1: f32) -> u32 {
            unsafe { lf_checker_rt::callee_thiscall!(C_VOICE, u32, obj, a0.to_bits(), a1.to_bits()) }
        }
        #[inline(always)]
        unsafe fn scalar(obj: u32, a: u32) -> f32 {
            unsafe { lf_checker_rt::callee_thiscall!(C_SCALAR, f32, obj, a) }
        }

        let milli = g32(G_MILLI);
        // Two voice rescales.
        let v0 = g32(G_V0);
        voice(this.wrapping_add(0x2744), v0, v0);
        let t1 = mul(g32(G_V1A), milli);
        let t0 = mul(g32(G_V1B), milli);
        voice(this.wrapping_add(0x29bc), t0, t1);
        // Thread-local listener row.
        let idx = rd32(lf_checker_rt::relocated(G_TLSIDX));
        let blk = lf_checker_rt::tls_slot(idx as usize);
        let row = rd32(blk.wrapping_add(0x70));
        let tbase = lf_checker_rt::relocated(TLS_TABLE).wrapping_add(row.wrapping_shl(6));
        let f14 = rdf(tbase.wrapping_add(0x30));
        let f1c = rdf(tbase.wrapping_add(0x34));
        let f38 = rdf(tbase.wrapping_add(0x38));
        // Prefilter over the 3-float frame block (pointer skipped in the
        // contract; contents snapshotted).
        let block = [f14.to_bits(), f1c.to_bits(), f38.to_bits()];
        lf_checker_rt::callee_thiscall!(C_PREFILTER, u32, this, &block as *const u32 as u32);
        // x87 entry value, truncated toward zero to an integer.
        let st = lf_checker_rt::x87_f64(0);
        let lo: u32 = if st.is_nan() || st >= 9223372036854775808.0 || st < -9223372036854775808.0 {
            0 // low word of the 0x8000000000000000 indefinite
        } else {
            (st as i64) as u32
        };
        let q120 = (((MAGIC120.wrapping_mul(lo as u64)) >> 32) as u32) >> 6;
        let rem = lo.wrapping_sub(q120.wrapping_mul(120));
        let base = lo.wrapping_sub(rem);
        let quo = (((MAGIC120.wrapping_mul(base as u64)) >> 32) as u32) >> 6;
        // 9x9 table fill.
        let mut d: u32 = 0;
        while d < 9 {
            let mut s: u32 = 0;
            while s < 9 {
                let v: f32 = lf_checker_rt::callee_stdcall!(
                    C_TABLE,
                    f32,
                    rem.wrapping_sub(4).wrapping_add(s),
                    quo.wrapping_sub(d).wrapping_add(4)
                );
                wrf(
                    this.wrapping_add(0x2788).wrapping_add(d.wrapping_mul(9).wrapping_add(s).wrapping_mul(4)),
                    v,
                );
                s = s.wrapping_add(1);
            }
            d = d.wrapping_add(1);
        }
        // Double-precision seed conversion, twice: first rem, then quo
        // (eax is reloaded from each slot before its block).
        let dbl0 = f64::from_bits(rd64(
            lf_checker_rt::relocated(G_DBLTAB).wrapping_add((rem >> 31).wrapping_mul(8)),
        ));
        let dd = core::hint::black_box(rem as i32 as f64) + core::hint::black_box(dbl0);
        let mut h = sub(dd as f32, g32(G_D60));
        h = add(h, g32(G_D05));
        h = mul(h, g32(G_D50));
        let mut f14b = sub(f14, h);
        f14b = mul(f14b, g32(G_D002));
        let dbl1 = f64::from_bits(rd64(
            lf_checker_rt::relocated(G_DBLTAB).wrapping_add((quo >> 31).wrapping_mul(8)),
        ));
        let dd2 = core::hint::black_box(quo as i32 as f64) + core::hint::black_box(dbl1);
        let mut h2 = sub(dd2 as f32, g32(G_D60));
        h2 = add(h2, g32(G_D05));
        h2 = mul(h2, g32(G_D50));
        let mut f1cb = sub(f1c, h2);
        f1cb = mul(f1cb, g32(G_D002));
        // Sign selects.
        let mut edi: u32 = 0x31;
        if f1cb > 0.0 {
            edi = 0x1f;
        }
        let (sel18, eo): (u32, u32);
        // jbe taken (f14b<=0 or NaN) keeps the earlier 0x27 and takes edi-1;
        // not taken stores 0x29 and takes edi+1.
        if f14b > 0.0 {
            sel18 = 0x29;
            eo = edi.wrapping_add(1);
        } else {
            sel18 = 0x27;
            eo = edi.wrapping_sub(1);
        }
        let af14 = f32::from_bits(
            f14b.to_bits() & rd32(lf_checker_rt::relocated(G_ABS)),
        );
        // Four scalar calls.
        let a1 = scalar(this, 0x28);
        let a2 = scalar(this, sel18);
        let a3 = scalar(this, edi);
        let a4 = scalar(this, eo);
        // Three combiner calls chained through two reused slots.
        let one = g32(G_ONE);
        let b1: f32 = lf_checker_rt::callee_cdecl!(
            C_COMBINE, f32, a1.to_bits(), a2.to_bits(), 0, one.to_bits(), af14.to_bits()
        );
        let b2: f32 = lf_checker_rt::callee_cdecl!(
            C_COMBINE, f32, a3.to_bits(), a4.to_bits(), 0, one.to_bits(), af14.to_bits()
        );
        let af1c = f32::from_bits(
            f1cb.to_bits() & rd32(lf_checker_rt::relocated(G_ABS)),
        );
        let b3: f32 = lf_checker_rt::callee_cdecl!(
            C_COMBINE, f32, b1.to_bits(), b2.to_bits(), 0, one.to_bits(), af1c.to_bits()
        );
        // Filter + mix-in to [this+0x2740].
        let c1 = filt(this.wrapping_add(0x2760), b3);
        let gg = rd32(lf_checker_rt::relocated(G_GLOB));
        let d1: f32 =
            lf_checker_rt::callee_thiscall!(C_MIXIN, f32, this.wrapping_add(0x2744), c1.to_bits(), gg);
        wrf(this.wrapping_add(0x2740), d1);
        // Four mix calls.
        let k34 = g32(G_K34);
        let e1: f32 = lf_checker_rt::callee_thiscall!(
            C_MIX, f32, this, f14b.to_bits(), add(k34, f1cb).to_bits()
        );
        let e2: f32 = lf_checker_rt::callee_thiscall!(
            C_MIX, f32, this, add(k34, f14b).to_bits(), f1cb.to_bits()
        );
        let e3: f32 = lf_checker_rt::callee_thiscall!(
            C_MIX, f32, this, f14b.to_bits(), sub(f1cb, k34).to_bits()
        );
        let e4: f32 = lf_checker_rt::callee_thiscall!(
            C_MIX, f32, this, sub(f14b, k34).to_bits(), f1cb.to_bits()
        );
        // Four-iteration output loop.
        let mut p14 = this.wrapping_add(0x28cc);
        let mut sp = this.wrapping_add(0x293c);
        let mut k: u32 = 0;
        while k < 4 {
            let ek = if k == 0 {
                e1
            } else if k == 1 {
                e2
            } else if k == 2 {
                e3
            } else {
                e4
            };
            let f6 = filt(this.wrapping_add(0x2760), ek);
            let f7: f32 = lf_checker_rt::callee_thiscall!(C_MIXIN, f32, p14, f6.to_bits(), gg);
            wrf(sp, f7);
            p14 = p14.wrapping_add(0x1c);
            sp = sp.wrapping_add(4);
            k = k.wrapping_add(1);
        }
        // Flag-selected gain path.
        if rd8(lf_checker_rt::relocated(G_FLAG1)) == 0 {
            if rd8(lf_checker_rt::relocated(G_FLAG2)) != 0 {
                wrf(this.wrapping_add(0x2740), g32(G_HALF));
            } else {
                let row2 = rd32(blk.wrapping_add(0x70));
                let t2 = lf_checker_rt::relocated(TLS_TABLE2).wrapping_add(row2.wrapping_shl(6));
                let alr: u32 = lf_checker_rt::callee_cdecl!(C_RANGE, u32, t2);
                wrf(
                    this.wrapping_add(0x2740),
                    if alr & 0xff != 0 {
                        f32::from_bits(0x3f800000)
                    } else {
                        f32::from_bits(0x3f000000)
                    },
                );
            }
        }
        // State query with out-byte. The original zeroes only the low byte
        // of a slot still holding |f14b| above it, and the call-time
        // snapshot observes the whole word: reproduce it exactly.
        let mut outbyte: u32 = af14.to_bits() & 0xffffff00;
        let al10: u32 = lf_checker_rt::callee_thiscall!(
            C_STATE, u32, this, &outbyte as *const u32 as u32, 0
        );
        // Zero constant iff the query answered nonzero AND the out-byte stayed
        // zero (je falls to the load on al==0; jne falls through on byte==0).
        let x0: f32 = if al10 & 0xff != 0 && outbyte & 0xff == 0 {
            f32::from_bits(0)
        } else {
            one
        };
        let f73: f32 =
            lf_checker_rt::callee_thiscall!(C_MIXIN, f32, this.wrapping_add(0x29bc), x0.to_bits(), gg);
        let m2740 = rdf(this.wrapping_add(0x2740));
        let outobj = lf_checker_rt::relocated(OUT_OBJ);
        lf_checker_rt::callee_thiscall!(C_SET_A, u32, outobj, m2740.to_bits());
        lf_checker_rt::callee_thiscall!(C_SET_B, u32, outobj, this.wrapping_add(0x293c));
        let f63 = filt(this.wrapping_add(0x1240), m2740);
        let qq = div(one, f63);
        lf_checker_rt::callee_thiscall!(C_SET_C, u32, outobj, qq.to_bits());
        let rr = mul(f73, g32(G_K40));
        let fin: u32 = lf_checker_rt::callee_thiscall!(C_SET_D, u32, outobj, rr.to_bits());
        // Cookie round-trip: ecx at the check always equals the cookie.
        lf_checker_rt::callee_fastcall!(C_COOKIE, u32, rd32(lf_checker_rt::relocated(G_COOKIE)), 0);
        fin
    }
});
