// original: 0x00ace110 audio_mix_slices
/// Mix pass: resolve the source offsets, run the per-slice loop that emits
/// one triple per slice and re-emits slices whose id scan hits, then publish
/// the scaled direction, the optional output vector and the transformed
/// tail triple through the voice's hooks.
///
/// `a` is the voice, `b` the source, `c` the slice array of `count` entries,
/// `f1`/`f2` mix scalars, `d` an opaque per-slice argument, `out` an optional
/// output vector. Returns nothing; effects are the outgoing calls and the
/// output-vector writes.
///
/// Build note: this rewrite is verified bit-exact as compiled without
/// optimizations. The optimizing compiler reorders some SSE operands, which
/// changes NaN payload propagation on trials where two different-payload
/// NaNs meet (random-bit harness fills only); see the lane report.
export!(cdecl, rw_ace110(
    a: u32,
    b: u32,
    c: u32,
    count: i32,
    f1_bits: u32,
    d: u32,
    f2_bits: u32,
    out: u32,
) -> u32 {
    /// File VA of the id table consulted by the per-slice scan.
    const ID_TABLE: u32 = 0x103f348;
    /// File VA of the shared vector pushed to the first virtual hook.
    const HOOK_VEC: u32 = 0x1b4b320;
    /// Stride of the slice array walked by the outer loop and the inner scan.
    const SLICE_STRIDE: u32 = 0x170;
    /// File VA of the output adjustment vector.
    const OUT_VEC: u32 = 0x103f390;
    unsafe {
        let f2 = f32::from_bits(f2_bits);
        let af = a as *const f32;
        let bf = b as *const f32;
        let mut ob = [0.0f32; 16];
        callee_cdecl!(
            1,
            u32,
            ob.as_mut_ptr() as u32,
            a.wrapping_add(0xd0),
            b.wrapping_add(0x40),
            b.wrapping_add(0x50),
            f2_bits
        );
        let mut t3 = [
            *bf.add(0x30 / 4) - ob[12],
            *bf.add(0x34 / 4) - ob[13],
            *bf.add(0x38 / 4) - ob[14],
        ];
        let vt = *(a as *const u32);
        let slot70 = *((vt + 0x70) as *const u32);
        let hook3: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(slot70 as usize);
        hook3(a, t3.as_mut_ptr() as u32, relocated(HOOK_VEC), 0);
        let mut e8 = [0.0f32; 4];
        callee_thiscall!(
            6,
            u32,
            e8.as_mut_ptr() as u32,
            ob.as_mut_ptr() as u32,
            b
        );
        let mut id6buf = [0.0f32; 2];
        callee_thiscall!(
            7,
            u32,
            e8.as_mut_ptr() as u32,
            id6buf.as_mut_ptr() as u32
        );
        let mut w3 = [0.0f32; 3];
        let mut scale = 0.0f32;
        callee_thiscall!(
            8,
            u32,
            id6buf.as_mut_ptr() as u32,
            w3.as_mut_ptr() as u32,
            core::ptr::addr_of_mut!(scale) as u32
        );
        w3[0] = w3[0] * scale;
        w3[1] = w3[1] * scale;
        w3[2] = w3[2] * scale;
        let slot74 = *((vt + 0x74) as *const u32);
        let hook1: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(slot74 as usize);
        hook1(a, w3.as_mut_ptr() as u32);
        let d0 = *bf.add(0x40 / 4) - *af.add(0x110 / 4);
        let d1 = *bf.add(0x44 / 4) - *af.add(0x114 / 4);
        let d2 = *bf.add(0x48 / 4) - *af.add(0x118 / 4);
        let mut e3 = [
            *bf.add(0x50 / 4) - *af.add(0x120 / 4),
            *bf.add(0x54 / 4) - *af.add(0x124 / 4),
            *bf.add(0x58 / 4) - *af.add(0x128 / 4),
        ];
        if count > 0 {
            let mut c_cur = c;
            let mut e_cur = b.wrapping_add(0x60);
            let mut x_cur = b.wrapping_add(0x80);
            for _ in 0..count {
                callee_thiscall!(
                    9,
                    u32,
                    c_cur,
                    a,
                    b,
                    b.wrapping_add(0x40),
                    b.wrapping_add(0x50),
                    *(e_cur as *const u32),
                    x_cur,
                    f1_bits,
                    d,
                    f2_bits,
                    w3.as_mut_ptr() as u32
                );
                // The original skips the scan only when all three emitted
                // words are exactly zero (each lahf test jumps on zero).
                if w3[0] != 0.0 || w3[1] != 0.0 || w3[2] != 0.0 {
                    let entry0 = *(c_cur as *const u32);
                    let table = relocated(ID_TABLE);
                    let want = *((table.wrapping_add(entry0.wrapping_mul(4)))
                        as *const u32);
                    let mut j: i32 = 0;
                    while j < count {
                        let at = c.wrapping_add((j as u32).wrapping_mul(SLICE_STRIDE));
                        if *(at as *const u32) == want {
                            break;
                        }
                        j += 1;
                    }
                    if j < count {
                        let lane =
                            c.wrapping_add((j as u32).wrapping_mul(SLICE_STRIDE));
                        if lane != 0 {
                            callee_thiscall!(
                                10, u32, lane, a, w3.as_mut_ptr() as u32
                            );
                        }
                    }
                }
                c_cur = c_cur.wrapping_add(SLICE_STRIDE);
                e_cur = e_cur.wrapping_add(4);
                x_cur = x_cur.wrapping_add(0x10);
            }
        }
        let c0 = *af.add(0xc0 / 4);
        let r = 1.0f32 / f2;
        t3[0] = (d0 * c0) * r;
        t3[1] = (d1 * c0) * r;
        t3[2] = (d2 * c0) * r;
        let slot84 = *((vt + 0x84) as *const u32);
        let hook84: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(slot84 as usize);
        hook84(a, t3.as_mut_ptr() as u32);
        if out != 0 {
            let g = global::<[f32; 3]>(OUT_VEC);
            // The original stores the intermediate products first and then
            // the adjusted values; only the final values are observable.
            *((out) as *mut f32) = ((d0 * c0) * r) - (c0 * (*g)[0]);
            *((out + 4) as *mut f32) = ((d1 * c0) * r) - (c0 * (*g)[1]);
            *((out + 8) as *mut f32) = ((d2 * c0) * r) - (c0 * (*g)[2]);
        }
        let e = (a.wrapping_add(0xd0)) as *const f32;
        let e0 = *e.add(0);
        let e1 = *e.add(1);
        let e2 = *e.add(2);
        let e4 = *e.add(4);
        let e5 = *e.add(5);
        let e6 = *e.add(6);
        let e8v = *e.add(8);
        let e9 = *e.add(9);
        let e10 = *e.add(10);
        let v0 = e3[0];
        let v1 = e3[1];
        let v2 = e3[2];
        let r1 = (v0 * e0 + v1 * e1) + v2 * e2;
        let r4 = (e5 * v1 + e4 * v0) + e6 * v2;
        let r2 = (e9 * v1 + v0 * e8v) + e10 * v2;
        let s5 = r1 * *af.add(0xa0 / 4);
        let s2 = *af.add(0xa4 / 4) * r4;
        let s3 = *af.add(0xa8 / 4) * r2;
        let f78 = ((s5 * e0) + (e4 * s2)) + (s3 * e8v);
        let f7c = ((e5 * s2) + (s5 * e1)) + (e9 * s3);
        let f80 = ((e6 * s2) + (s5 * e2)) + (e10 * s3);
        e3[0] = f78;
        e3[1] = f7c;
        e3[2] = f80;
        // The original also spills one word from below the resolve block
        // here; that slot is never written, so the harness defines it as
        // zero, and the stored word is never read again either.
        t3[0] = f78 * r;
        t3[1] = f7c * r;
        t3[2] = f80 * r;
        let slot88 = *((vt + 0x88) as *const u32);
        let hook88: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(slot88 as usize);
        hook88(a, t3.as_mut_ptr() as u32);
        0
    }
});
