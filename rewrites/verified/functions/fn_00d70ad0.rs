// original: 0x00d70ad0 replay_bar_layout
/// Lay out the replay bar's markers for a two-tick window.
///
/// Clamps two scaled configuration values into unit bounds, then, when the
/// object's position sits strictly inside the resulting window on all four
/// comparisons, samples the text backend and arms the object. After that it
/// runs an unconditional pipeline: two tick counts scaled to milliseconds
/// offset sampled backend values into the object's cells, two record fields
/// scaled the same way are subtracted back out, a zero-snapping guard keeps
/// the head cell, two out-param blocks fill scratch words that are published
/// through the formatting calls, and a final ratio scaled like the draw
/// routine's feeds three measure calls whose combination lands in the tail
/// cells. Every floating-point comparison keeps the original's NaN path
/// (unordered inputs skip the gated block). Returns the last measure's bits.
export!(thiscall, rw_d70ad0(this: u32, arg0: u32, arg1: u32) -> u32 {
    const CLAMP_MAX: f32 = f32::from_bits(0x3F80_0000);
    const MS_PER_TICK: f32 = f32::from_bits(0x3A83_126F);
    const SNAP_LIM: f32 = f32::from_bits(0x3D4C_CCCD);
    const BAR_SCALE: f32 = f32::from_bits(0x3EE6_6666);
    // Tail subtrahend from the file's read-only constants (verified by
    // raw section math and the mapped image alike, sitting in a coherent
    // ascending table; the r-b145 transcription ended in A7D7).
    const TAIL_SUB: f32 = f32::from_bits(0x3BA3_D70A);
    const U32_FIXUP: [f64; 2] = [0.0, 4294967296.0];
    const NUM_WIDE: u32 = 0x0105_C87C;
    const NUM_NARROW: u32 = 0x0105_C880;
    const DEN_WIDE: u32 = 0x0105_C888;
    const DEN_NARROW: u32 = 0x0105_C884;
    unsafe {
        (this as *mut u8).byte_add(0x21).write(0);
        let t1 = (global::<u32>(0x018B_7A8C).read() as i32) as f32
            * global::<f32>(0x017A_CCF0).read();
        let mut v1 = t1;
        if v1 < 0.0 {
            v1 = 0.0;
        } else if v1 > CLAMP_MAX {
            v1 = CLAMP_MAX;
        }
        let t0 = (global::<u32>(0x018B_7A80).read() as i32) as f32
            * global::<f32>(0x017A_CCE8).read();
        let w2 = if t0 < 0.0 {
            0.0
        } else if t0 > CLAMP_MAX {
            CLAMP_MAX
        } else {
            t0
        };
        let pos18 = (this as *const f32).byte_add(0x18).read();
        if w2 > pos18 {
            let t = (this as *const f32).byte_add(0x28).read() + pos18;
            if t > w2 {
                let pos1c = (this as *const f32).byte_add(0x1C).read();
                if v1 > pos1c {
                    let f = callee_cdecl!(1, f32,);
                    if f + pos1c > v1 {
                        let m = global::<u32>(0x018B_7A84).read();
                        let c = (global::<u32>(0x018B_7A88).read() ^ m) & m;
                        if c & 1 != 0 {
                            let b = (this as *const u8).byte_add(0x20).read();
                            (this as *mut u8).byte_add(0x20).write(u8::from(b == 0));
                        }
                        (this as *mut u8).byte_add(0x21).write(1);
                    }
                }
            }
        }
        let g1 = callee_cdecl!(2, f32,);
        let t3 = g1 + (arg1 as i32) as f32 * MS_PER_TICK;
        (this as *mut f32).byte_add(4).write(t3);
        let t4 = (arg0 as i32) as f32 * MS_PER_TICK;
        let g2 = callee_cdecl!(3, f32,);
        (this as *mut f32).write(g2 + t4);
        let ebx = global::<u32>(0x011F_6954).read();
        let g3 = callee_thiscall!(4, f32, ebx);
        (this as *mut f32).byte_add(0x14).write(g3);
        let edi = global::<u32>(0x011F_6F34).read();
        let s1 = callee_thiscall!(5, u32, ebx, edi);
        let esi1 = (s1 as *const u32).byte_add(0x3C).read();
        let g4 = callee_cdecl!(3, f32,);
        let t6 = g4 + t4;
        let d1 = (((esi1 as i32) as f64) + U32_FIXUP[(esi1 >> 31) as usize]) as f32
            * MS_PER_TICK;
        let t7 = t6 - d1;
        let g5 = callee_thiscall!(6, f32, ebx, edi);
        (this as *mut f32).byte_add(0x10).write(g5 + t7);
        let s2 = callee_thiscall!(5, u32, ebx, edi);
        (this as *mut u32).byte_add(0x0C).write((s2 as *const u32).byte_add(0x44).read());
        let s3 = callee_thiscall!(5, u32, ebx, edi);
        let esi2 = (s3 as *const u32).byte_add(0x3C).read();
        let g6 = callee_cdecl!(3, f32,);
        let t9 = g6 + t4;
        let d2 = (((esi2 as i32) as f64) + U32_FIXUP[(esi2 >> 31) as usize]) as f32
            * MS_PER_TICK;
        (this as *mut f32).byte_add(8).write(t9 - d2);
        let m0 = (this as *const f32).read();
        if SNAP_LIM >= m0 {
            (this as *mut f32).write(0.0);
        }
        let bl = u32::from((this as *const u8).byte_add(0x20).read() == 0);
        let mut s10 = 0u32;
        let mut s14 = 0u32;
        let mut s18b = 0u32;
        let mut s1c = 0u32;
        let mut s20 = 0u32;
        let mut s24 = 0u32;
        callee_thiscall!(
            7, u32, this,
            &mut s20 as *mut u32 as u32,
            &mut s24 as *mut u32 as u32,
            &mut s14 as *mut u32 as u32,
            1, bl
        );
        callee_thiscall!(
            8, u32, this,
            &mut s10 as *mut u32 as u32,
            &mut s18b as *mut u32 as u32,
            &mut s1c as *mut u32 as u32,
            0, bl
        );
        callee_cdecl!(9, u32, this + 0x2C, 0x10, relocated(0x00EE_AEA8), s20, s24, s14);
        callee_cdecl!(10, u32, this + 0x4C, 0x10, relocated(0x00EE_AEC8), s10, s18b, s1c);
        let f_a = global::<f32>(0x0118_EFF4).read();
        let f_b = global::<f32>(0x0118_EFFC).read();
        let a1 = callee_cdecl!(11, u32,);
        let num = if a1 & 0xFF != 0 {
            global::<u32>(NUM_WIDE).read()
        } else {
            global::<u32>(NUM_NARROW).read()
        };
        let a2 = callee_cdecl!(12, u32,);
        let den = if a2 & 0xFF != 0 {
            global::<u32>(DEN_WIDE).read()
        } else {
            global::<u32>(DEN_NARROW).read()
        };
        let ratio = (num as i32) as f32 / (den as i32) as f32;
        let mut anchor = 0u32;
        callee_cdecl!(13, u32, relocated(0x00EE_AEE8), &mut anchor as *mut u32 as u32);
        let scaled = ratio * BAR_SCALE;
        callee_cdecl!(14, u32, scaled.to_bits(), BAR_SCALE.to_bits());
        callee_cdecl!(15, u32, 0);
        let m1 = callee_cdecl!(16, f32, this + 0x2C, 0);
        (this as *mut f32).byte_add(0x24).write(m1);
        let mut f2tmp = 0u32;
        let m2 = callee_cdecl!(17, f32, &mut f2tmp as *mut u32 as u32, 0);
        let t11 = m2 + m1;
        let m3 = callee_cdecl!(16, f32, this + 0x4C, 0);
        let t12 = t11 + m3;
        let t13 = f_b + f_a;
        (this as *mut f32).byte_add(0x28).write(t12);
        (this as *mut f32).byte_add(0x18).write(t13 - t12 - TAIL_SUB);
        m3.to_bits()
    }
});
