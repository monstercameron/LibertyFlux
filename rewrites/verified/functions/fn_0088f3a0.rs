// original: 0x0088F3A0 audVoicePcAdpcm_setRateInt (proposed)

/// Set this ADPCM voice's rate word from a float factor, or reconcile its
/// child-gate flag when the factor is zero.
///
/// `this` points to the voice and `arg_bits` is the factor as float bits.
/// When the factor is a non-zero float (NaN counts as non-zero), the
/// unsigned word at `+0xc` is widened exactly to double (through the
/// original's sign-split table form), narrowed to float, multiplied by the
/// factor with truncation toward zero to an integer, stored at `+0x13c`
/// and handed to the child at `+0x140` (callee 3); flag bit 0 is then
/// cleared, setting bit 3 first when bit 6 was set. When the factor is
/// zero and flag bit 0 is clear, flag bit 6 is set to the child gate
/// (callee 1 on the child, false when the child is null), the child's
/// apply entry (callee 2) runs when bit 6 came out set, and bit 0 is set.
/// Same shape as the software voice's setter, one table slot further into
/// the object.
///
/// Original: 0x0088F3A0 (thiscall, one float stack word).
lf_checker_rt::export!(thiscall, rw_0088F3A0(this: u32, arg_bits: u32) -> () {
    unsafe {
        const SCALE_INT: u32 = 0x0c;
        const FLAGS: u32 = 0x8c;
        const CHILD: u32 = 0x140;
        const STORE: u32 = 0x13c;
        const TWO_POW_32: f64 = 4294967296.0;
        const GATE_CHILD: u32 = 1;
        const APPLY_CHILD: u32 = 2;
        const SET_CHILD: u32 = 3;

        #[inline(always)]
        fn fistp_chop_low(v: f32) -> u32 {
            if v.is_nan() {
                return 0;
            }
            let t = v.trunc();
            if t >= 9223372036854775808.0 || t < -9223372036854775808.0 {
                0
            } else {
                (t as i64) as u32
            }
        }

        let arg = f32::from_bits(arg_bits);
        let flags = (this + FLAGS) as *mut u8;
        if arg != 0.0 {
            let e = ((this + SCALE_INT) as *const u32).read_unaligned();
            let wide = core::hint::black_box(e as i32 as f64)
                + core::hint::black_box(if e >> 31 == 0 {
                    0.0
                } else {
                    TWO_POW_32
                });
            let narrow = wide as f32;
            let prod =
                core::hint::black_box(narrow) * core::hint::black_box(arg);
            let eax = fistp_chop_low(prod);
            let child = ((this + CHILD) as *const u32).read_unaligned();
            ((this + STORE) as *mut u32).write_unaligned(eax);
            lf_checker_rt::callee_thiscall!(SET_CHILD, u32, child, eax);
            let al = flags.read();
            if al & 1 != 0 {
                if al & 0x40 != 0 {
                    flags.write(al | 8);
                }
                flags.write(flags.read() & 0xfe);
            }
        } else if flags.read() & 1 == 0 {
            let child = ((this + CHILD) as *const u32).read_unaligned();
            let gate = if child != 0 {
                lf_checker_rt::callee_thiscall!(GATE_CHILD, u32, child) as u8
                    != 0
            } else {
                false
            };
            let f = flags.read();
            flags.write((f & !0x40) | ((gate as u8) << 6));
            if flags.read() & 0x40 != 0 {
                lf_checker_rt::callee_thiscall!(APPLY_CHILD, u32, child);
            }
            flags.write(flags.read() | 1);
        }
    }
});
