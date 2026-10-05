// original: 0x008A2620 aud_distance_voice_blend (proposed)
/// Blend up to four audio voices by a distance factor, returning the factor.
///
/// `this` points to the blend object, `a0` selects the silent path by its
/// LOW BYTE. The function returns a float on the x87 stack: 0 when the base
/// level at `+0xB0` is negative (ordered) with null level pointers at
/// `+0xBC`/`+0xC0`, a constant when byte `+0xCD` and `a0` are both zero,
/// otherwise a computed factor `k0` in [0, 1] (NaN stays NaN).
///
/// When `+0xC0` is non-null, `k0` clamps the float it points to: at most 0
/// gives 0, below 1 keeps the value, otherwise 1 (an unordered value takes
/// the middle branch, so NaN passes through).
///
/// When `+0xC0` is null, an entry is resolved from byte `+0xCC` times the
/// second stride global plus the row cell at `table + row*0x6F40 + 0x6F14`
/// (row is byte `+0x40`). A tag byte at entry `+0xE7` (low 3 bits) indexes a
/// global table of 8 dwords; three floats at entry `+(T+1)*16` go through
/// the transform callee (with the pool global in ECX and 0), which writes
/// three floats back. `d` is their vector length, computed as
/// `sqrt((y*y + x*x) + z*z)`. The low bound is `+0xB8` dereferenced (or
/// `+0xB0` when null), the high bound `+0xBC` dereferenced (or `+0xB4`).
/// `k0` is 0 when the low bound reaches `d`, 1 when `d` reaches the high
/// bound, else `(d - lo) / (hi - lo)`; unordered comparisons fall through to
/// the fraction, and a zero span yields infinity or NaN exactly as the
/// hardware divides.
///
/// The curve callee then maps `1 - k0` to `r1` and `k0` to `r0` (float in,
/// x87 float out). Voice slots `+0x48`/`+0x49` with a present, resolving
/// selector commit `r1` through the voice callee, slots `+0x4A`/`+0x4B`
/// commit `r0`; selectors resolve through the row cell at `+0x6F10`.
///
/// Float operations run in the original's operand order through pinned
/// helpers; integer address math wraps; all integer comparisons are unsigned
/// or equality.
///
/// Original: 0x008A2620 (thiscall, one stack word, x87 float result).
export!(thiscall, rw_008a2620(this: u32, a0: u32) -> f64 {
    unsafe {
        const ROW_STRIDE: u32 = 0x6f40;
        const TABLE_BIAS: u32 = 0x6f10;
        const TABLE_BIAS2: u32 = 0x6f14;
        const ABSENT: u8 = 0xFF;
        const ONE: f32 = 1.0;

        let t = this as *const u8;
        let rd8 = |off: u32| t.add(off as usize).read();
        let rd32 = |off: u32| (t.add(off as usize) as *const u32).read_unaligned();
        let rdf = |off: u32| f32::from_bits(rd32(off));

        // jbe after comiss(0, base) skips only when base is NOT ordered-below
        // zero, i.e. base >= 0 or NaN; an ordered-negative base with two null
        // level pointers returns +0.
        let base = rdf(0xB0);
        if base < 0.0 && rd32(0xBC) == 0 && rd32(0xC0) == 0 {
            return 0.0;
        }
        if rd8(0xCD) == 0 && (a0 & 0xFF) == 0 {
            return f32::from_bits(global::<u32>(0xfe8830).read()) as f64;
        }

        let c0 = rd32(0xC0);
        let k0 = if c0 != 0 {
            let v = f32::from_bits((c0 as *const u32).read_unaligned());
            if v <= 0.0 {
                0.0
            } else if !(v >= ONE) {
                v
            } else {
                ONE
            }
        } else {
            let row = rd8(0x40);
            let idx = rd8(0xCC);
            let stride2 = global::<u32>(0x115d968).read();
            let table = global::<u32>(0x115d988).read();
            let cell2 = ((table
                .wrapping_add((row as u32).wrapping_mul(ROW_STRIDE))
                .wrapping_add(TABLE_BIAS2)) as *const u32)
                .read_unaligned();
            let entry = (idx as u32)
                .wrapping_mul(stride2)
                .wrapping_add(cell2);
            let tag =
                (((entry.wrapping_add(0xE7)) as *const u8).read() & 7) as u32;
            let tbase = global::<u32>(0x115f80c) as u32;
            let tv = ((tbase.wrapping_add(tag.wrapping_mul(8))) as *const u32)
                .read_unaligned();
            let off = tv.wrapping_add(1).wrapping_mul(2).wrapping_mul(8);
            let v0 =
                f32::from_bits((entry.wrapping_add(off) as *const u32).read_unaligned());
            let v1 = f32::from_bits(
                (entry.wrapping_add(off).wrapping_add(4) as *const u32).read_unaligned(),
            );
            let v2 = f32::from_bits(
                (entry.wrapping_add(off).wrapping_add(8) as *const u32).read_unaligned(),
            );
            let input = [v0.to_bits(), v1.to_bits(), v2.to_bits()];
            let mut output = [0u32; 3];
            let pool = global::<u32>(0x115f7f4).read();
            let _: u32 = callee_thiscall!(
                3,
                u32,
                pool,
                output.as_mut_ptr() as u32,
                input.as_ptr() as u32,
                0
            );
            let x = f32::from_bits(output[0]);
            let y = f32::from_bits(output[1]);
            let z = f32::from_bits(output[2]);
            let d = fsqrt(fadd(fadd(fmul(y, y), fmul(x, x)), fmul(z, z)));
            let pb8 = rd32(0xB8);
            let lo = if pb8 != 0 {
                f32::from_bits((pb8 as *const u32).read_unaligned())
            } else {
                base
            };
            let pbch = rd32(0xBC);
            let hi = if pbch != 0 {
                f32::from_bits((pbch as *const u32).read_unaligned())
            } else {
                rdf(0xB4)
            };
            // jb after comiss(lo, d): continue when lo < d or unordered.
            if !(lo >= d) {
                // jb after comiss(d, hi): fraction when d < hi or unordered.
                if !(d >= hi) {
                    fdiv(fsub(d, lo), fsub(hi, lo))
                } else {
                    ONE
                }
            } else {
                0.0
            }
        };

        let r1: u32 = callee_cdecl!(1, u32, fsub(ONE, k0).to_bits());
        let r0: u32 = callee_cdecl!(1, u32, k0.to_bits());

        let stride = global::<u32>(0x115d964).read();
        let table = global::<u32>(0x115d988).read();
        let row = rd8(0x40);
        let cell = ((table
            .wrapping_add((row as u32).wrapping_mul(ROW_STRIDE))
            .wrapping_add(TABLE_BIAS)) as *const u32)
            .read_unaligned();
        let mut s = 0u32;
        while s < 4 {
            let sel = rd8(0x48 + s);
            if sel != ABSENT {
                let obj = stride.wrapping_mul(sel as u32).wrapping_add(cell);
                if obj != 0 {
                    let v = if s < 2 { r1 } else { r0 };
                    let _: u32 = callee_thiscall!(2, u32, obj, v);
                }
            }
            s += 1;
        }

        (k0) as f64
    }
});
