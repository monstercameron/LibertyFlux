// original: 0x008A6310 aud_param_expression_eval (proposed)
/// Evaluate a small per-voice parameter expression program.
///
/// `this` points to the voice object, `arg0` (stored at `+0xB0`) is an
/// integer operand some operators read back. Byte `+0xB4` is the record
/// count, byte `+0xB5` the row index, byte `+0x40` the table row. Records are
/// 24 bytes at `stride*index + cell`, where the cell is read at
/// `table + row*0x6F40 + 0x6F10`. Each record holds three operand words, a
/// store pointer (a null store skips the record), an opcode dword and a flag
/// byte. Operands are floats or pointers to floats per flag bits 0/1/2.
/// Each record's result is stored as one float through its store pointer.
/// Only heap writes and outgoing calls are observable; the function keeps no
/// meaningful return value.
///
/// Opcodes (selected by a jump table) cover arithmetic, truncation-based
/// float remainder, min/max, bit masking, sign tests, fraction extraction,
/// integer rounding, selection, interpolation, three-way clamp, calls to
/// double and float math helpers, and reading the integer argument back as a
/// float. An opcode above 0x1F stores 0.0: the loop prologue zeroes the
/// running value every iteration, so entry vector state never survives to
/// any arm (the contract still varies entry xmm4 to prove it is ignored).
///
/// Float operations run in the original's operand order through pinned
/// helpers; integer conversions truncate exactly like the hardware
/// instructions; all integer comparisons are unsigned or equality.
///
/// Original: 0x008A6310 (thiscall, one stack word, no meaningful return).
export!(thiscall, rw_008a6310(this: u32, arg0: u32) -> u32 {
    unsafe {
        const ROW_STRIDE: u32 = 0x6f40;
        const TABLE_BIAS: u32 = 0x6f10;
        const REC_LEN: u32 = 0x18;

        // Low lane of compare-less-than (ordered only).
        let cmplt = |a: f32, b: f32| -> u32 {
            if a < b {
                0xFFFF_FFFF
            } else {
                0
            }
        };
        // Low lane of compare-not-less-or-equal (imm8 6: ordered
        // greater-than; false when unordered).
        let cmpnle = |a: f32, b: f32| -> u32 {
            if a > b {
                0xFFFF_FFFF
            } else {
                0
            }
        };
        // Truncate a float toward zero to 64 bits like x87 fistp with the
        // chop control, keeping the low 32 bits (out of range or NaN gives
        // the indefinite value, whose low half is 0).
        let trunc_low32 = |x: f32| -> u32 {
            let d = x as f64;
            if d.is_nan() || d >= 9223372036854775808.0 || d < -9223372036854775808.0
            {
                0
            } else {
                d.trunc() as i64 as u32
            }
        };
        // Truncate a float to int32 like cvttss2si (invalid gives 0x80000000).
        let cvtt = |x: f32| -> u32 {
            if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
                0x8000_0000
            } else {
                (x as i32) as u32
            }
        };
        // The shared fraction-extraction sequence.
        let frac_seq = |x: f32, ca: u32, cb: f32| -> f32 {
            let m2 = x.to_bits() & ca;
            let t0 = f32::from_bits(x.to_bits() ^ m2);
            let mask = cmplt(t0, cb);
            let m1f = f32::from_bits(cb.to_bits() & mask | m2);
            let mut r = fadd(x, m1f);
            r = fsub(r, m1f);
            let o = fsub(r, x);
            let nm = cmpnle(o, f32::from_bits(m2));
            fsub(r, f32::from_bits(1.0f32.to_bits() & nm))
        };

        let t = this as *const u8;
        let rd8 = |off: u32| t.add(off as usize).read();
        let rd32 = |off: u32| (t.add(off as usize) as *const u32).read_unaligned();
        (this as *mut u32).add(0xB0 / 4).write_unaligned(arg0);

        let count = rd8(0xB4);
        if count == 0 {
            return 0;
        }
        let stride = global::<u32>(0x115d964).read();
        let table = global::<u32>(0x115d988).read();
        let row = rd8(0x40);
        let idx = rd8(0xB5);
        let cell = ((table
            .wrapping_add((row as u32).wrapping_mul(ROW_STRIDE))
            .wrapping_add(TABLE_BIAS)) as *const u32)
            .read_unaligned();
        let base = stride.wrapping_mul(idx as u32).wrapping_add(cell);

        let ca_and = global::<u32>(0xfe8f80).read();
        let cb = f32::from_bits(global::<u32>(0xfe8cf8).read());
        let c_half = f32::from_bits(global::<u32>(0xfe8830).read());
        let c_one = f32::from_bits(global::<u32>(0xfe88e8).read());
        let c_sign = f32::from_bits(global::<u32>(0xfe8d94).read());
        let ca_frac = global::<u32>(0xfe8d1c).read();

        // The loop prologue zeroes the running value every iteration, so the
        // default arm stores 0.0 (entry xmm4 never survives to any arm).
        let mut i = 0u32;
        while i < count as u32 {
            let rec = base.wrapping_add(i.wrapping_mul(REC_LEN));
            let op_a = (rec as *const u32).read_unaligned();
            let op_b = ((rec.wrapping_add(4)) as *const u32).read_unaligned();
            let op_c = ((rec.wrapping_add(8)) as *const u32).read_unaligned();
            let store = ((rec.wrapping_add(0x0C)) as *const u32).read_unaligned();
            if store == 0 {
                i += 1;
                continue;
            }
            let opc = ((rec.wrapping_add(0x10)) as *const u32).read_unaligned();
            let flag = ((rec.wrapping_add(0x14)) as *const u8).read();
            let fetch = |w: u32, bit: u8| -> f32 {
                if (flag & bit) != 0 {
                    f32::from_bits(w)
                } else {
                    f32::from_bits((w as *const u32).read_unaligned())
                }
            };
            let x5 = fetch(op_a, 1);
            let x1 = fetch(op_b, 2);
            let x0 = fetch(op_c, 4);
            let x4 = match opc {
                0 => fadd(x1, x5),
                1 => fsub(x5, x1),
                2 => fmul(x1, x5),
                3 => fdiv(x5, x1),
                4 => x5,
                5 => {
                    let b = trunc_low32(x1);
                    let a = trunc_low32(x5);
                    // b is nonzero on every trial: the contract clears flag
                    // bit 1 wherever the opcode is 5 (a pointer-bits operand
                    // would truncate to 0) and steers float operands to
                    // values truncating nonzero.
                    (a % b) as f32
                }
                6 => {
                    if !(x1 > x5) {
                        x1
                    } else {
                        x5
                    }
                }
                7 => {
                    if !(x5 > x1) {
                        x1
                    } else {
                        x5
                    }
                }
                8 | 15 => f32::from_bits(x5.to_bits() & ca_and),
                9 => {
                    // lahf/test/jp idiom: negative gives -1.0, zero 0.0,
                    // positive or NaN 1.0.
                    if x5 < 0.0 {
                        c_sign
                    } else if x5 == 0.0 {
                        0.0
                    } else {
                        c_one
                    }
                }
                10 => frac_seq(x5, ca_frac, cb),
                11 => {
                    let d = (x5 as f64).to_bits();
                    let ans: f64 =
                        callee_cdecl!(1, f64, d as u32, (d >> 32) as u32);
                    ans as f32
                }
                12 => {
                    let ans: u32 =
                        callee_cdecl!(2, u32, x5.to_bits(), x1.to_bits());
                    f32::from_bits(ans)
                }
                13 => {
                    let ans: u32 = callee_cdecl!(3, u32, x5.to_bits());
                    f32::from_bits(ans)
                }
                14 => {
                    let ans: u32 = callee_cdecl!(4, u32, x5.to_bits());
                    f32::from_bits(ans)
                }
                16 => {
                    let t = fsub(x5, f32::from_bits(global::<u32>(0xfe8df8).read()));
                    if !(t >= 0.0) {
                        0.0
                    } else {
                        let x5b = fmul(
                            x5,
                            f32::from_bits(global::<u32>(0xfe876c).read()),
                        );
                        let d0 = global::<u64>(0xfe8a78).read();
                        let d1 = (x5b as f64).to_bits();
                        let ans: u64 = callee_cdecl!(
                            5,
                            u64,
                            d0 as u32,
                            (d0 >> 32) as u32,
                            d1 as u32,
                            (d1 >> 32) as u32
                        );
                        f64::from_bits(ans) as f32
                    }
                }
                17 => {
                    let ans: u32 = callee_cdecl!(6, u32, x5.to_bits());
                    f32::from_bits(ans)
                }
                18 => {
                    let d0 = global::<u64>(0xfe8a28).read();
                    let iv = cvtt(x5);
                    let f = fmul(
                        (iv as i32) as f32,
                        f32::from_bits(global::<u32>(0xe78574).read()),
                    );
                    let d1 = (f as f64).to_bits();
                    let ans: u64 = callee_cdecl!(
                        5,
                        u64,
                        d0 as u32,
                        (d0 >> 32) as u32,
                        d1 as u32,
                        (d1 >> 32) as u32
                    );
                    f64::from_bits(ans) as f32
                }
                19 => {
                    let x = f32::from_bits(callee_cdecl!(7, u32, x5.to_bits()));
                    let x = fmul(x, f32::from_bits(global::<u32>(0xe78588).read()));
                    let x = fmul(x, f32::from_bits(global::<u32>(0xe7858c).read()));
                    (cvtt(x) as i32) as f32
                }
                20 => fmul(
                    arg0 as f32,
                    f32::from_bits(global::<u32>(0xfe86b4).read()),
                ),
                21 => {
                    if !(x5 >= 0.0) {
                        x0
                    } else {
                        x1
                    }
                }
                22 => fadd(fmul(fsub(x0, x1), x5), x1),
                23 => {
                    if x1 > x5 {
                        x1
                    } else if x5 > x0 {
                        x0
                    } else {
                        x5
                    }
                }
                24 => {
                    let ans: u32 = callee_cdecl!(8, u32, x5.to_bits());
                    f32::from_bits(ans)
                }
                25 => {
                    let v = if x5 < 0.0 {
                        fsub(x5, c_half)
                    } else {
                        fadd(x5, c_half)
                    };
                    (cvtt(v) as i32) as f32
                }
                26 => {
                    let x = fmul(x5, f32::from_bits(global::<u32>(0xfe8a24).read()));
                    let x = fmul(x, f32::from_bits(global::<u32>(0xfe8aa0).read()));
                    let x = f32::from_bits(callee_cdecl!(3, u32, x.to_bits()));
                    fmul(fadd(x, c_one), c_half)
                }
                27 => {
                    let v2 = frac_seq(x5, ca_frac, cb);
                    let r = fsub(x5, v2);
                    let t = fsub(r, c_half);
                    if !(t >= 0.0) {
                        fmul(r, f32::from_bits(global::<u32>(0xfe8a24).read()))
                    } else {
                        let t = fmul(t, f32::from_bits(global::<u32>(0xfe8a24).read()));
                        fsub(c_one, t)
                    }
                }
                28 => {
                    let v2 = frac_seq(x5, ca_frac, cb);
                    fsub(x5, v2)
                }
                29 => {
                    let v2 = frac_seq(x5, ca_frac, cb);
                    let t = fsub(fsub(x5, v2), c_half);
                    // Taken branch takes xmm1, which this arm set to 1.0.
                    if !(t >= 0.0) {
                        c_one
                    } else {
                        0.0
                    }
                }
                30 => {
                    let x2 = fsub(x1, x5);
                    let s6 = if x2 < 0.0 {
                        c_sign
                    } else if x2 == 0.0 {
                        0.0
                    } else {
                        c_one
                    };
                    let x2m = f32::from_bits(x2.to_bits() & ca_and);
                    let sel = if x0 > x2m { x2m } else { x0 };
                    fsub(x1, fmul(sel, s6))
                }
                31 => {
                    let v = fdiv(
                        fmul(
                            arg0 as f32,
                            f32::from_bits(global::<u32>(0xfe86b4).read()),
                        ),
                        x5,
                    );
                    let v2 = frac_seq(v, ca_frac, cb);
                    fsub(v, v2)
                }
                _ => 0.0,
            };
            (store as *mut u32).write_unaligned(x4.to_bits());
            i += 1;
        }
        0
    }
});
