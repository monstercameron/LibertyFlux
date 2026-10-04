// original: 0x008b02b0 audio_mixer_eval_voice
/// Surround-mixer evaluator for one voice.
///
/// Normalizes the direction vector, builds four channel gains through
/// clamped blend stages gated by block flags and the voice range, scales by
/// the voice's table, distance-attenuation and meter terms, and writes five
/// output floats to the voice struct.
export!(thiscall, rw_008b02b0(this: u32, vec: u32, voice: u32) -> () {
    unsafe {
        let one = *global::<f32>(0xFE88E8);
        let vx = rdf(vec);
        let vz = rdf(vec.wrapping_add(8));
        let mut x1 = vz * vz + vx * vx;
        if x1 != 0.0 {
            x1 = one / x1.sqrt();
        } else {
            x1 = 0.0;
        }
        let c0 = *global::<f32>(0xE7CB8C);
        let mut x0 = x1 * vx;
        let mut x3 = x1 * 0.0;
        let mut x4 = vz * x1;
        let s3c = x3;
        x3 = x3 * 0.0;
        let s20 = x0;
        x0 *= c0;
        let s18 = x4;
        let mut x5 = x3 - x0;
        x4 *= c0;
        x1 = x4 + x5;
        let mut s38 = x5;
        let mut s14 = x1;
        if x1 < 0.0 {
            x1 = 0.0;
            s14 = 0.0;
        }
        x0 = s20;
        x0 *= c0;
        let mut s40 = x1;
        x0 += x3;
        let mut x7 = x0 + x4;
        if x7 < 0.0 {
            x7 = 0.0;
        }
        x4 = s18;
        x4 *= *global::<f32>(0xE7CBAC);
        let mut s44 = x7;
        x5 = x4 + s38;
        if x5 < 0.0 {
            x5 = 0.0;
        }
        x4 += x0;
        let mut s48 = x5;
        if x4 < 0.0 {
            x4 = 0.0;
        }
        let mut s4c = x4;
        let full = ld32(this.wrapping_add(0x17f0)) == 1
            && (ld32(this.wrapping_add(0x17ec)) as i32) >= 3;
        if full {
            x0 = s20;
            let mut x6 = x0;
            x6 = x6 * 0.0;
            x6 += x3;
            x3 += x0;
            x0 = s18;
            x0 = x0 * 0.0;
            x6 += s18;
            x3 += x0;
            x0 = *global::<f32>(0xFE8878);
            if x6 > 0.0 && x0 > x6 {
                x6 = x7;
                x6 -= x0;
                x6 *= *global::<f32>(0xE7CB98);
                let mut s30 = x6;
                if x6 < 0.0 {
                    s30 = 0.0;
                }
                x6 = one;
                let mut x2 = x4;
                x2 *= *global::<f32>(0xFE8940);
                let mut s34 = x2;
                if x2 > x6 {
                    s34 = x6;
                }
                x2 = x1;
                x2 -= x0;
                x0 = x2;
                x0 *= *global::<f32>(0xE7CB98);
                s38 = x2;
                x2 = 0.0;
                let _ = x2;
                if x0 < 0.0 {
                    x0 = 0.0;
                }
                x1 = x5;
                x1 *= *global::<f32>(0xFE8940);
                s38 = x1;
                if x1 > x6 {
                    s38 = x6;
                }
                if x3 >= 0.0 {
                    x7 = s30;
                }
                s44 = x7;
                if x3 >= 0.0 {
                    x4 = s34;
                }
                x1 = s14;
                s4c = x4;
                if !(x3 >= 0.0) {
                    x1 = x0;
                }
                s14 = x1;
                s40 = x1;
                if !(x3 >= 0.0) {
                    x5 = s38;
                }
            } else {
                x1 = *global::<f32>(0xFE8878);
                let to585 = !(x1 >= x6);
                x6 = one;
                if !to585 {
                    x0 = 0.0;
                    s14 = x0;
                    s40 = x0;
                    let k = *global::<u32>(0xFE8F20);
                    x0 = f32::from_bits(x3.to_bits() ^ k);
                    x5 = x6;
                    let s38b = x0;
                    s38 = s38b;
                    x0 *= *global::<f32>(0xE7CB88);
                    x5 -= x3;
                    x7 = 0.0;
                    s44 = x7;
                    x0 += x1;
                    x5 *= x1;
                    if !(x3 >= 0.0) {
                        x4 = x6;
                        x4 -= s38b;
                        x4 *= x1;
                    } else {
                        x4 = x3;
                        x4 *= *global::<f32>(0xE7CB88);
                        x4 += x1;
                    }
                    s4c = x4;
                    if !(x3 >= 0.0) {
                        x5 = x0;
                    }
                }
            }
            s48 = x5;
        }
        // Second half: voice scaling.
        let mut y3 = rdf(vec);
        let mut y1 = rdf(vec.wrapping_add(4));
        let mut y0 = y3 * y3;
        y3 = rdf(vec.wrapping_add(8));
        y1 *= y1;
        y1 += y0;
        y0 = y3 * y3;
        y1 += y0;
        let mut rlen;
        if y1 != 0.0 {
            rlen = one / y1.sqrt();
        } else {
            rlen = 0.0;
        }
        y0 = rdf(vec.wrapping_add(4));
        let mut y6 = rdf(vec);
        y1 = rdf(vec.wrapping_add(8));
        y6 *= rlen;
        y0 *= rlen;
        y6 *= s20;
        y0 *= s3c;
        y1 *= rlen;
        y6 += y0;
        y0 = *global::<f32>(0xFE8D48);
        y1 *= s18;
        y6 += y1;
        if *global::<f32>(0xFE8D48) > y6 {
            y6 = 0.0;
        } else if y6 > *global::<f32>(0xFE88FC) {
            // Ordered greater-than: an unordered (NaN) y6 keeps its value,
            // matching jbe-taken. !(y6 <= C) would wrongly zero it.
            y6 = 0.0;
        }
        let b = |off: u32| ((this.wrapping_add(off)) as *const u8).read_unaligned();
        // Packed blend over the four slots (replicated lane by lane).
        let g4 = |va: u32| {
            let p = xbase().wrapping_add(va.wrapping_sub(0x400000));
            [rdf(p), rdf(p.wrapping_add(4)), rdf(p.wrapping_add(8)), rdf(p.wrapping_add(12))]
        };
        let mut p3: [f32; 4];
        if b(0x1795) != 0 {
            let c4a = g4(0xFE8F20);
            let c4e = g4(0xE75920);
            let mut r3 = c4a;
            let mut r0 = [s40, s44, s48, s4c];
            for i in 0..4 {
                r3[i] -= y6;
                r0[i] *= y6;
                r3[i] *= c4e[i];
                r3[i] += r0[i];
            }
            p3 = r3;
            s40 = p3[0];
            s44 = p3[1];
            s48 = p3[2];
            s4c = p3[3];
            y0 = s40;
            x4 = s4c;
            x5 = s48;
            x7 = s44;
            s14 = y0;
        } else {
            p3 = [s40, s44, s48, s4c];
        }
        y1 = rdf(vec);
        y0 = rdf(vec.wrapping_add(4));
        let esi = voice;
        let eaxt = *global::<u32>(0x115F814);
        let edi = (((esi.wrapping_add(0xd0)) as *const u16).read_unaligned() as i16) as i32;
        y1 *= y1;
        y0 *= y0;
        y1 += y0;
        y0 = rdf(vec.wrapping_add(8));
        let ecx9 = edi.wrapping_mul(9);
        y0 *= y0;
        y6 = rdf(eaxt.wrapping_add((ecx9 as u32).wrapping_mul(4)).wrapping_add(4));
        y6 *= rdf(esi.wrapping_add(0xc4));
        let eaxv = ld32(esi.wrapping_add(0xac));
        y1 += y0;
        y6 *= rdf(this.wrapping_add(0x1734));
        y6 *= rdf(this.wrapping_add(0x17dc));
        y1 = y1.sqrt();
        if eaxv != 0 {
            y0 = rdf(eaxv.wrapping_add(0x18));
            y0 *= y6;
            y6 = y0;
        }
        let lo17 = ld32(this.wrapping_add(0x17e4)) as i32;
        let hi17 = ld32(this.wrapping_add(0x17e8)) as i32;
        if !(edi < lo17 || edi > hi17) {
            y6 = *global::<f32>(0xFE879C);
        }
        y0 = rdf(this.wrapping_add(0x1738));
        s38 = y0;
        y0 = rdf(this.wrapping_add(0x173c));
        let cond_a = one > y6;
        if cond_a || b(0x1797) != 0 {
            let mut y2 = rdf(this.wrapping_add(0x1738));
            y2 *= y6;
            y0 *= y6;
            s38 = y2;
        }
        y6 = s38;
        y0 -= y6;
        y1 -= y6;
        y1 /= y0;
        y0 = one;
        if !(y1 < 1.0) {
            y1 = y0;
        } else if y1 < 0.0 {
            y1 = 0.0;
        }
        y0 = s14;
        y6 = 0.0;
        if !(y0 < 0.0) {
            y6 = y0;
        }
        if !(y6 > x7) {
            y6 = x7;
        }
        if !(y6 > x5) {
            y6 = x5;
        }
        if !(y6 > x4) {
            y6 = x4;
        }
        if b(0x1796) == 0 {
            y6 = s14;
        } else {
            let c4a = g4(0xFE8F20);
            let c4e = g4(0xE75920);
            let mut p0 = c4a;
            for i in 0..4 {
                p3[i] *= y1;
                p0[i] -= y1;
            }
            if b(0x1798) == 0 {
                for i in 0..4 {
                    p0[i] *= y6;
                }
            } else {
                for i in 0..4 {
                    p0[i] *= c4e[i];
                }
            }
            for i in 0..4 {
                p3[i] += p0[i];
            }
            s40 = p3[0];
            s44 = p3[1];
            s48 = p3[2];
            s4c = p3[3];
            x4 = s4c;
            x5 = s48;
            x7 = s44;
            y6 = s40;
        }
        let mut y2;
        if eaxv != 0 {
            y2 = rdf(eaxv.wrapping_add(0x1c));
        } else {
            y2 = 0.0;
        }
        y0 = rdf(this.wrapping_add(0x1748));
        if !(y2 > y0) {
            y2 = y0;
        }
        y0 = one;
        if !(y0 > y2) {
            y2 = y0;
        }
        if b(0x1799) != 0 {
            let c4a = g4(0xFE8F20);
            let c4e = g4(0xE75920);
            let mut p1 = c4a;
            let mut p0 = c4e;
            for i in 0..4 {
                p1[i] -= y2;
                p0[i] *= y2;
                p1[i] *= p3[i];
                p1[i] += p0[i];
            }
            s40 = p1[0];
            s44 = p1[1];
            s48 = p1[2];
            s4c = p1[3];
            x4 = s4c;
            x5 = s48;
            x7 = s44;
            y6 = s40;
        }
        if b(0x179c) != 0 {
            y2 = rdf(this.wrapping_add(0x1744));
            y0 -= y2;
            y1 = y6;
            let mut x3b = x5;
            y1 *= y2;
            let s38c = y0;
            x5 *= s38c;
            y6 *= y0;
            let mut y0b = x7;
            x7 *= s38c;
            y0b *= y2;
            x3b *= y2;
            y6 += y0b;
            let mut y0c = x4;
            x4 *= s38c;
            y0c *= y2;
            x7 += y1;
            x4 += x3b;
            x5 += y0c;
        }
        st32(esi.wrapping_add(0xc), 0);
        st32(esi.wrapping_add(8), 0);
        wrf(esi, y6);
        wrf(esi.wrapping_add(4), x7);
        wrf(esi.wrapping_add(0x10), x5);
        wrf(esi.wrapping_add(0x14), x4);
    }
});
