// original: 0x0088D490 audVoiceDSoundAdpcm_setScaledInt (proposed)

/// Set this ADPCM voice's scaled integer parameter from a float factor, or
/// flag the voice idle when the factor is zero.
///
/// `this` points to the voice and `arg_bits` is the factor as float bits.
/// When the factor is a non-zero float (NaN counts as non-zero), the
/// unsigned word at `+0xc` is widened exactly to double (through the
/// original's sign-split table form), narrowed to float, multiplied by the
/// factor with truncation toward zero to an integer, and stored at `+0xe8`
/// when it differs; the device at `+0x90` is then told (slot `+0x44`,
/// called with the device and the value). When the factor is zero, the
/// device is stopped instead (slot `+0x48`) unless flag bit 0 at `+0x8c`
/// is already set, and that bit is set. A trailing flag step clears bit 0
/// and sets bit 3 whenever bit 0 was found set on the convert path. Same
/// shape as the DirectSound voice's setter, one table slot further into
/// the object.
///
/// Original: 0x0088D490 (thiscall, one float stack word).
lf_checker_rt::export!(thiscall, rw_0088D490(this: u32, arg_bits: u32) -> () {
    unsafe {
        const SCALE_INT: u32 = 0x0c;
        const FLAGS: u32 = 0x8c;
        const VOICE_DEVICE: u32 = 0x90;
        const CURRENT: u32 = 0xe8;
        const SLOT_STOP: u32 = 0x48;
        const SLOT_SET: u32 = 0x44;
        const TWO_POW_32: f64 = 4294967296.0;

        #[inline(always)]
        fn fistp_chop_low(v: f32) -> u32 {
            // What an x87 `fistp qword` with the chop control leaves in the
            // low dword: truncation toward zero, out-of-range/NaN/infinite
            // giving the indefinite value whose low dword is zero.
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
            let edx = fistp_chop_low(prod);
            let cur = (this + CURRENT) as *mut u32;
            if edx != cur.read_unaligned() {
                let obj =
                    ((this + VOICE_DEVICE) as *const u32).read_unaligned();
                cur.write_unaligned(edx);
                let table = (obj as *const u32).read_unaligned();
                let set: extern "stdcall" fn(u32, u32) -> u32 =
                    core::mem::transmute(
                        ((table + SLOT_SET) as *const u32).read_unaligned()
                            as usize,
                    );
                set(obj, edx);
            }
            let al = ((this + FLAGS) as *const u8).read();
            if al & 1 != 0 {
                ((this + FLAGS) as *mut u8).write((al & 0xfe) | 8);
            }
        } else if ((this + FLAGS) as *const u8).read() & 1 == 0 {
            let obj = ((this + VOICE_DEVICE) as *const u32).read_unaligned();
            if obj != 0 {
                let table = (obj as *const u32).read_unaligned();
                let stop: extern "stdcall" fn(u32) -> u32 =
                    core::mem::transmute(
                        ((table + SLOT_STOP) as *const u32).read_unaligned()
                            as usize,
                    );
                stop(obj);
            }
            let f = (this + FLAGS) as *mut u8;
            f.write(f.read() | 1);
        }
    }
});
