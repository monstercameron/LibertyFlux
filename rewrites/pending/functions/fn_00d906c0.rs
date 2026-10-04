// original: 0x00d906c0 audio_box_query_nearest
/// Find the nearest cell overlapped by a query box.
///
/// Builds an integer bound box from the component-wise min/max of `a` and
/// `b` (scaled by 8, the z range padded by 1), then scans every cell whose
/// packed box overlaps it. Each overlapping cell has its corners resolved
/// into the shared plane slots and each plane pair plane-tested; on a pass
/// the squared distance from `c` to `a` competes for the best slot.
/// Returns the winning cell index, or 0xFFFF when no plane of any
/// overlapping cell passes. The fourth stack argument is ignored, and the
/// stack-cookie checks run unmodified on the original side.
export!(thiscall, rw_00d906c0(this: u32, a: *const f32, b: *const f32,
                              c: *const f32, _tag: u32) -> u32 {
    if unsafe { *((this + 0x50) as *const u8) } & 4 != 0 {
        callee_thiscall!(1, u32, this);
    }
    let av = unsafe { [*a, *a.add(1), *a.add(2)] };
    let bv = unsafe { [*b, *b.add(1), *b.add(2)] };
    // comiss+jbe orders NaN below everything: `!(b > a)` matches exactly.
    let mut lo = [0.0f32; 3];
    let mut hi = [0.0f32; 3];
    for i in 0..3 {
        if !(bv[i] > av[i]) {
            hi[i] = av[i];
            lo[i] = bv[i];
        } else {
            hi[i] = bv[i];
            lo[i] = av[i];
        }
    }
    // Signed 16-bit box compares (jg/jl): negative bounds sort below zero.
    let lo0 = cvtt(lo[0] * 8.0) as i16;
    let lo1 = cvtt(lo[1] * 8.0) as i16;
    let lo2 = cvtt((lo[2] - 1.0) * 8.0) as i16;
    let hi0 = cvtt(hi[0] * 8.0) as i16;
    let hi1 = cvtt(hi[1] * 8.0) as i16;
    let hi2 = cvtt((hi[2] + 1.0) * 8.0) as i16;
    let flag = global::<u32>(0x179fd30);
    if unsafe { *flag } & 1 == 0 {
        unsafe {
            *flag |= 1;
        }
    }
    let count = unsafe { *((this + 0x7c) as *const u32) };
    if count == 0 {
        return 0xFFFF;
    }
    let cv = unsafe { [*c, *c.add(1), *c.add(2)] };
    let entries = unsafe { *((this + 0x6c) as *const u32) };
    let words = unsafe { *((this + 0x60) as *const *const u16) };
    let mut best = f32::MAX;
    let mut besti = 0xFFFFu32;
    let mut i = 0u32;
    while i < count {
        let e = entries.wrapping_add(i.wrapping_mul(40));
        let e0 = unsafe { *(e as *const u32) };
        let t0 = unsafe { *((e + 0x10) as *const i16) };
        let t1 = unsafe { *((e + 0x12) as *const i16) };
        let t2 = unsafe { *((e + 0x14) as *const i16) };
        let t3 = unsafe { *((e + 0x16) as *const i16) };
        let t4 = unsafe { *((e + 0x18) as *const i16) };
        let t5 = unsafe { *((e + 0x1a) as *const i16) };
        if (e0 >> 0x12) & 1 == 0
            && lo0 <= t1 && lo1 <= t3 && lo2 <= t5
            && hi0 >= t0 && hi1 >= t2 && hi2 >= t4
        {
            let count2 = (e0 >> 0x15) & 0xf;
            let base = unsafe { *((e + 4) as *const u32) } & 0x1ffff;
            if count2 != 0 {
                for k in 0..count2 {
                    let w = unsafe { *words.add((base + k) as usize) } as u32;
                    let slot = global::<u32>(0x179fc30 + k * 0x10) as u32;
                    callee_thiscall!(2, u32, this, w, slot);
                }
            }
            if (e0 & 0x1e00000) > 0x400000 {
                let mut blk = [0.0f32; 12];
                for w in 0..4 {
                    blk[w] = unsafe { *global::<f32>(0x179fc30 + (w as u32) * 4) };
                }
                let mut j = 2u32;
                let mut s_lo = 0x10u32;
                let mut s_hi = 0x20u32;
                while j < count2 {
                    for w in 0..4 {
                        blk[4 + w] =
                            unsafe { *global::<f32>(0x179fc30 + s_hi + (w as u32) * 4) };
                        blk[8 + w] =
                            unsafe { *global::<f32>(0x179fc30 + s_lo + (w as u32) * 4) };
                    }
                    let r = callee_cdecl!(3, u32, a as u32, b as u32,
                                          blk.as_mut_ptr() as u32, c as u32);
                    if r & 0xFF != 0 {
                        let dx = cv[0] - av[0];
                        let dy = cv[1] - av[1];
                        let dz = cv[2] - av[2];
                        let dist = fadd(fadd(dy * dy, dx * dx), dz * dz);
                        if dist < best {
                            best = dist;
                            besti = i;
                        }
                    }
                    j += 1;
                    s_lo = s_hi;
                    s_hi += 0x10;
                }
            }
        }
        i += 1;
    }
    besti
});
