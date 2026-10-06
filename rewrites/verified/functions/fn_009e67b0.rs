// original: 0x009e67b0 ped_pose_matrix_blend (proposed)
///
/// Blend three resolved pose sources into the output record and two matrix
/// buffers, then hand the buffers to two matrix helpers.
///
/// `this` is an object with a virtual table (slot `+0xA0` resolves a live
/// source, refined through the source's slot `+0xE0`; when slot `+0xA0`
/// answers null the fallback pointer at `+0x100` is used instead), a table
/// pointer at `+0x224` and float state. `out_ptr` receives three refined
/// float triples (at `+0`, `+4`, `+8`) plus a word at `+0xC`; `aux` is only
/// passed through as the object pointer of the final helper call.
///
/// Algorithm: each of the three sources is resolved (as above) and passed
/// with a constant tag (`0x4D0`, `0x4C3`, `0x4B5`) to a classifier; the
/// answers index three frame lookups whose results are 64-byte rows with
/// float triples at `+0x30`. The output triple starts as the scaled
/// difference of two rows. A five-entry table at `[this+0x224]+0x44` is
/// scanned for its first non-zero entry (signed `jl` loop bound, values
/// 0-4) and its `+8` chain walked to the end; the end node's object at
/// `+0x10` is probed twice, and when both probes succeed and the second
/// answer exceeds the first (unordered-NaN counts as not-exceeding) a
/// quotient picks one of two vector blends, else defaults are used. Two
/// matrix products accumulate into the output triple, two 16-word buffers
/// (every fourth word unwritten) go to the first helper, a longer
/// accumulation fills the first buffer again, and the second helper takes
/// it with `aux`. All integer compares are equality or zero tests except
/// the 0..5 table scan; the one float compare treats NaN as below.
///
/// The fourth word of the output triple and every fourth buffer word read
/// stack slots this function never writes; under the contract's zero fill
/// they read 0, which the rewrite writes explicitly. The third frame
/// lookup's index is the caller's entry `esi`, which safe Rust cannot read
/// and which the stubbed lookup ignores, so the rewrite passes 0 and the
/// contract skips that one argument. (thiscall, two stack words.)
lf_checker_rt::export!(thiscall, rw_009e67b0(this_ptr: u32, out_ptr: u32, aux: u32) -> u32 {
    unsafe { fn1_body(this_ptr, out_ptr, aux) }
});

#[allow(clippy::too_many_lines)]
unsafe fn fn1_body(this_ptr: u32, out_ptr: u32, aux: u32) -> u32 {
    unsafe {
        const SLOT_A: u32 = 0xA0;
        const SLOT_B: u32 = 0xE0;
        const SLOT_C: u32 = 0x0C;
        const FALLBACK_OFF: u32 = 0x100;
        const TABLE_OFF: u32 = 0x224;
        const ONE_BITS: u32 = 0x3F800000;
        const TAG1: u32 = 0x4D0;
        const TAG2: u32 = 0x4C3;
        const TAG3: u32 = 0x4B5;
        const SPECIAL: u32 = 0x145;
        const G_SCALE: u32 = 0xFE8830;
        const G_BASE_Y: u32 = 0xE988A4;
        const G_VEC0: u32 = 0x1B4B2A0;
        const G_VEC1: u32 = 0x1B4B2A4;
        const G_VEC2: u32 = 0x1B4B2A8;
        const G_SIGNMASK: u32 = 0xFE8FA0;
        const G_SUB1: u32 = 0xFE8D64;
        const G_SUB2: u32 = 0xFE879C;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
        unsafe fn vcall(obj: u32, slot: u32) -> u32 {
            unsafe {
                let vt = rd32(obj);
                let faddr = rd32(vt.wrapping_add(slot));
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(faddr as usize);
                f(obj)
            }
        }
        #[inline(always)]
        unsafe fn gfloat(va: u32) -> f32 {
            unsafe { f32::from_bits(rd32(lf_checker_rt::relocated(va))) }
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }

        // Frame: the original's 0xF0-byte aligned frame as words. Zeroed,
        // matching the contract's zero stack fill for the slots the
        // original reads without writing (0xEC, the buffer gaps).
        let mut frame = [0u32; 60];
        #[inline(always)]
        fn slot(frame: &mut [u32; 60], off: u32) -> *mut u32 {
            frame.as_mut_ptr().wrapping_add((off / 4) as usize)
        }
        let fr = |frame: &[u32; 60], off: u32| -> u32 { frame[(off / 4) as usize] };
        let frf = |frame: &[u32; 60], off: u32| -> f32 {
            f32::from_bits(frame[(off / 4) as usize])
        };
        let frs = |frame: &mut [u32; 60], off: u32, v: f32| {
            frame[(off / 4) as usize] = v.to_bits();
        };

        // Three resolve blocks: live source or fallback, then classify.
        let resolve = |this_ptr: u32| -> u32 {
            let a = vcall(this_ptr, SLOT_A);
            if a == 0 {
                rd32(this_ptr.wrapping_add(FALLBACK_OFF))
            } else {
                let b = vcall(this_ptr, SLOT_A);
                vcall(b, SLOT_B)
            }
        };
        let src_a = resolve(this_ptr);
        let idx1: u32 = lf_checker_rt::callee_cdecl!(3, u32, rd32(src_a.wrapping_add(4)), TAG1);
        frame[(0x38 / 4) as usize] = idx1;
        let src_b = resolve(this_ptr);
        let idx2: u32 = lf_checker_rt::callee_cdecl!(3, u32, rd32(src_b.wrapping_add(4)), TAG2);
        frame[(0x20 / 4) as usize] = idx2;
        let src_c = resolve(this_ptr);
        let idx3: u32 = lf_checker_rt::callee_cdecl!(3, u32, rd32(src_c.wrapping_add(4)), TAG3);

        // Three frame lookups. The third index is entry esi (unreadable
        // here, ignored by the stubbed lookup); the contract skips it.
        let row1: u32 = lf_checker_rt::callee_thiscall!(4, u32, this_ptr, fr(&frame, 0x38));
        frame[(0x38 / 4) as usize] = row1;
        let row2: u32 = lf_checker_rt::callee_thiscall!(5, u32, this_ptr, fr(&frame, 0x20));
        let row3: u32 =
            lf_checker_rt::callee_thiscall!(6, u32, this_ptr, 0u32);
        frame[(0x3c / 4) as usize] = row3;
        let _ = idx3;

        // Output head: scaled row difference plus the fill word.
        let scale = gfloat(G_SCALE);
        let mut dx = sub(rdf(row2.wrapping_add(0x30)), rdf(row1.wrapping_add(0x30)));
        let mut dy = sub(rdf(row2.wrapping_add(0x34)), rdf(row1.wrapping_add(0x34)));
        let mut dz = sub(rdf(row2.wrapping_add(0x38)), rdf(row1.wrapping_add(0x38)));
        dx = mul(dx, scale);
        dy = mul(dy, scale);
        dz = mul(dz, scale);
        wrf(out_ptr, dx);
        wrf(out_ptr.wrapping_add(4), dy);
        wrf(out_ptr.wrapping_add(8), dz);
        wrf(out_ptr.wrapping_add(0xC), frf(&frame, 0xEC));
        frs(&mut frame, 0x10, 0.0);

        // Table scan: first non-zero of five, then walk its +8 chain.
        let table = rd32(this_ptr.wrapping_add(TABLE_OFF));
        let mut found = 0u32;
        for k in 0..5u32 {
            let v = rd32(table.wrapping_add(k.wrapping_mul(4)).wrapping_add(0x44));
            if v != 0 {
                found = v;
                break;
            }
        }
        let mut node = found;
        if node != 0 {
            loop {
                let next = rd32(node.wrapping_add(8));
                if next == 0 {
                    break;
                }
                node = next;
            }
        }
        let probe_obj = if node == 0 { 0 } else { rd32(node.wrapping_add(0x10)) };

        // Blend vector (x6, x7) and stored factor, per the probe path.
        let base_y = gfloat(G_BASE_Y);
        let (mut vy, vz, stored): (f32, f32, f32);
        if probe_obj == 0 {
            vy = base_y;
            vz = 0.0;
        } else {
            frame[(0x1c / 4) as usize] = 0;
            let a1: u32 = lf_checker_rt::callee_thiscall!(
                7, u32, probe_obj, 0x800u32,
                slot(&mut frame, 0x1c) as u32, 0u32, ONE_BITS
            );
            let mut use_default = (a1 as u8) == 0;
            let mut o1 = 0.0f32;
            let mut o2 = 0.0f32;
            if !use_default {
                frame[(0x20 / 4) as usize] = ONE_BITS;
                let a2: u32 = lf_checker_rt::callee_thiscall!(
                    8, u32, probe_obj, 0x2000u32,
                    slot(&mut frame, 0x20) as u32, 0u32, ONE_BITS
                );
                if (a2 as u8) == 0 {
                    use_default = true;
                } else {
                    o2 = frf(&frame, 0x20);
                    o1 = frf(&frame, 0x1c);
                    if !(o2 > o1) {
                        use_default = true;
                    }
                }
            }
            if use_default {
                vy = base_y;
                vz = 0.0;
            } else {
                let t = sub(rdf(probe_obj.wrapping_add(0x4C)), o1);
                let denom = sub(o2, o1);
                let q = div(t, denom);
                frs(&mut frame, 0x10, q);
                let cans = vcall(node, SLOT_C);
                let q2 = frf(&frame, 0x10);
                if cans == SPECIAL {
                    let mask = rd32(lf_checker_rt::relocated(G_SIGNMASK));
                    let mut vx =
                        f32::from_bits(gfloat(G_VEC2).to_bits() ^ mask);
                    vy = sub(base_y, gfloat(G_VEC1));
                    let mut vz0 =
                        f32::from_bits(gfloat(G_VEC0).to_bits() ^ mask);
                    vx = mul(vx, q2);
                    vz0 = mul(vz0, q2);
                    vx = add(vx, gfloat(G_VEC2));
                    vy = mul(vy, q2);
                    vz0 = add(vz0, gfloat(G_VEC0));
                    vy = add(vy, gfloat(G_VEC1));
                    frs(&mut frame, 0x10, vx);
                    vz = vz0;
                } else {
                    vy = sub(gfloat(G_VEC1), gfloat(G_SUB1));
                    let mut vx = gfloat(G_VEC2);
                    let mut vz0 = gfloat(G_VEC0);
                    vx = mul(vx, q2);
                    vy = mul(vy, q2);
                    vz0 = mul(vz0, q2);
                    vy = sub(vy, gfloat(G_SUB2));
                    frs(&mut frame, 0x10, vx);
                    vz = vz0;
                }
            }
        }
        stored = frf(&frame, 0x10);

        // Join: accumulate two matrix products into the output triple.
        let m3 = row3;
        let m1 = row1;
        let mut x2 = stored;
        let mut x1 = rdf(m3.wrapping_add(0x18));
        let mut x5 = vz;
        x5 = mul(x5, rdf(m3));
        let mut x0 = vy;
        x0 = mul(x0, rdf(m3.wrapping_add(0x10)));
        x5 = add(x5, x0);
        x0 = frf(&frame, 0x10);
        x0 = mul(x0, rdf(m3.wrapping_add(0x20)));
        let mut x3 = rdf(out_ptr.wrapping_add(8));
        let mut x4 = vy;
        x4 = mul(x4, rdf(m3.wrapping_add(0x14)));
        x5 = add(x5, x0);
        x0 = vz;
        x0 = mul(x0, rdf(m3.wrapping_add(4)));
        x1 = mul(x1, vy);
        x4 = add(x4, x0);
        x0 = rdf(m3.wrapping_add(0x24));
        x0 = mul(x0, x2);
        x2 = mul(x2, rdf(m3.wrapping_add(0x28)));
        x5 = add(x5, rdf(out_ptr));
        x4 = add(x4, x0);
        x0 = rdf(m3.wrapping_add(8));
        x0 = mul(x0, vz);
        wrf(out_ptr, x5);
        x4 = add(x4, rdf(out_ptr.wrapping_add(4)));
        x1 = add(x1, x0);
        wrf(out_ptr.wrapping_add(4), x4);
        x1 = add(x1, x2);
        x2 = x4;
        x3 = add(x3, x1);
        x1 = x5;
        wrf(out_ptr.wrapping_add(8), x3);
        x0 = rdf(m1);
        x2 = mul(x2, rdf(m1.wrapping_add(4)));
        x1 = mul(x1, rdf(m1.wrapping_add(0x10)));
        x0 = mul(x0, x5);
        x5 = mul(x5, rdf(m1.wrapping_add(0x20)));
        x2 = add(x2, x0);
        x0 = x3;
        x0 = mul(x0, rdf(m1.wrapping_add(8)));
        x2 = add(x2, x0);
        x0 = x4;
        x0 = mul(x0, rdf(m1.wrapping_add(0x14)));
        x4 = mul(x4, rdf(m1.wrapping_add(0x24)));
        x1 = add(x1, x0);
        x0 = x3;
        x0 = mul(x0, rdf(m1.wrapping_add(0x18)));
        x3 = mul(x3, rdf(m1.wrapping_add(0x28)));
        x5 = add(x5, x4);
        x1 = add(x1, x0);
        wrf(out_ptr, x2);
        x5 = add(x5, x3);
        wrf(out_ptr.wrapping_add(4), x1);
        wrf(out_ptr.wrapping_add(8), x5);

        // Two 16-word buffers from row1 (every fourth word stays fill).
        x0 = rdf(m1);
        let mut x7 = rdf(m1.wrapping_add(0x14));
        let mut x6 = rdf(m1.wrapping_add(0x18));
        x5 = rdf(m1.wrapping_add(0x20));
        x4 = rdf(m1.wrapping_add(0x24));
        x3 = rdf(m1.wrapping_add(0x28));
        x2 = rdf(m1.wrapping_add(0x30));
        x1 = rdf(m1.wrapping_add(0x34));
        frs(&mut frame, 0x40, x0);
        x0 = rdf(m1.wrapping_add(4));
        frs(&mut frame, 0x44, x0);
        x0 = rdf(m1.wrapping_add(8));
        frs(&mut frame, 0x48, x0);
        x0 = rdf(m1.wrapping_add(0x10));
        frs(&mut frame, 0x50, x0);
        x0 = rdf(m1.wrapping_add(0x38));
        frs(&mut frame, 0x54, x7);
        frs(&mut frame, 0x58, x6);
        frs(&mut frame, 0x60, x5);
        frs(&mut frame, 0x64, x4);
        frs(&mut frame, 0x68, x3);
        frs(&mut frame, 0x70, x2);
        frs(&mut frame, 0x74, x1);
        x7 = rdf(m1);
        frs(&mut frame, 0xA0, x7);
        x7 = rdf(m1.wrapping_add(4));
        frs(&mut frame, 0xA4, x7);
        x7 = rdf(m1.wrapping_add(8));
        frs(&mut frame, 0xA8, x7);
        x7 = rdf(m1.wrapping_add(0x10));
        frs(&mut frame, 0xB0, x7);
        x7 = rdf(m1.wrapping_add(0x14));
        frs(&mut frame, 0x78, x0);
        frs(&mut frame, 0xB4, x7);
        frs(&mut frame, 0xB8, x6);
        frs(&mut frame, 0xC0, x5);
        frs(&mut frame, 0xC4, x4);
        frs(&mut frame, 0xC8, x3);
        frs(&mut frame, 0xD0, x2);
        frs(&mut frame, 0xD4, x1);
        frs(&mut frame, 0xD8, x0);
        let _: u32 = lf_checker_rt::callee_thiscall!(
            10, u32,
            slot(&mut frame, 0x40) as u32,
            slot(&mut frame, 0xA0) as u32
        );
        // Long accumulation over row3 with the first buffer, refilling it.
        x4 = rdf(m3.wrapping_add(4));
        x0 = frf(&frame, 0x54);
        x1 = frf(&frame, 0x44);
        x1 = mul(x1, rdf(m3));
        x3 = rdf(m3.wrapping_add(8));
        frs(&mut frame, 0x10, x0);
        x0 = mul(x0, x4);
        x7 = frf(&frame, 0x68);
        x6 = frf(&frame, 0x50);
        x1 = add(x1, x0);
        x0 = frf(&frame, 0x64);
        x0 = mul(x0, x3);
        x2 = rdf(m3.wrapping_add(0x18));
        x5 = frf(&frame, 0x60);
        x1 = add(x1, x0);
        x0 = frf(&frame, 0x58);
        frs(&mut frame, 0x1C, x0);
        x0 = mul(x0, x4);
        frs(&mut frame, 0x9C, x1);
        x1 = frf(&frame, 0x48);
        x1 = mul(x1, rdf(m3));
        frs(&mut frame, 0x3C, x4);
        frs(&mut frame, 0x38, x3);
        x1 = add(x1, x0);
        x0 = x7;
        x0 = mul(x0, x3);
        x3 = rdf(m3.wrapping_add(0x10));
        x4 = x6;
        x1 = add(x1, x0);
        x0 = frf(&frame, 0x40);
        frs(&mut frame, 0x20, x0);
        x0 = mul(x0, x3);
        frs(&mut frame, 0x88, x1);
        x1 = rdf(m3.wrapping_add(0x14));
        x4 = mul(x4, x1);
        x4 = add(x4, x0);
        x0 = x5;
        x0 = mul(x0, x2);
        x4 = add(x4, x0);
        x0 = frf(&frame, 0x44);
        x0 = mul(x0, x3);
        frs(&mut frame, 0x90, x4);
        x4 = frf(&frame, 0x10);
        x4 = mul(x4, x1);
        x4 = add(x4, x0);
        x0 = frf(&frame, 0x64);
        x0 = mul(x0, x2);
        x4 = add(x4, x0);
        x0 = frf(&frame, 0x48);
        x0 = mul(x0, x3);
        x3 = rdf(m3.wrapping_add(0x20));
        frs(&mut frame, 0x94, x4);
        x4 = frf(&frame, 0x1C);
        x4 = mul(x4, x1);
        x4 = add(x4, x0);
        x0 = x7;
        x0 = mul(x0, x2);
        x4 = add(x4, x0);
        frs(&mut frame, 0x98, x4);
        x1 = rdf(m3.wrapping_add(0x24));
        x0 = frf(&frame, 0x20);
        x0 = mul(x0, x3);
        x2 = rdf(m3.wrapping_add(0x28));
        x7 = x6;
        x6 = frf(&frame, 0x10);
        x7 = mul(x7, x1);
        x6 = mul(x6, x1);
        x7 = add(x7, x0);
        x0 = x5;
        x0 = mul(x0, x2);
        x5 = frf(&frame, 0x1C);
        x4 = frf(&frame, 0x50);
        x7 = add(x7, x0);
        x0 = frf(&frame, 0x44);
        x0 = mul(x0, x3);
        x5 = mul(x5, x1);
        x6 = add(x6, x0);
        x0 = frf(&frame, 0x64);
        x0 = mul(x0, x2);
        x1 = rdf(m3.wrapping_add(0x34));
        x4 = mul(x4, x1);
        x6 = add(x6, x0);
        x0 = frf(&frame, 0x48);
        x0 = mul(x0, x3);
        x3 = rdf(m3.wrapping_add(0x30));
        frs(&mut frame, 0x8C, x7);
        x5 = add(x5, x0);
        x0 = frf(&frame, 0x68);
        x0 = mul(x0, x2);
        x2 = rdf(m3.wrapping_add(0x38));
        x5 = add(x5, x0);
        x0 = frf(&frame, 0x20);
        x0 = mul(x0, x3);
        x4 = add(x4, x0);
        x0 = frf(&frame, 0x60);
        x0 = mul(x0, x2);
        x4 = add(x4, x0);
        x0 = frf(&frame, 0x10);
        x0 = mul(x0, x1);
        x4 = add(x4, frf(&frame, 0x70));
        frs(&mut frame, 0x10, x0);
        x0 = frf(&frame, 0x44);
        x7 = frf(&frame, 0x10);
        x0 = mul(x0, x3);
        x7 = add(x7, x0);
        x0 = frf(&frame, 0x64);
        x0 = mul(x0, x2);
        x7 = add(x7, x0);
        x0 = x7;
        x0 = add(x0, frf(&frame, 0x74));
        frs(&mut frame, 0x10, x7);
        frs(&mut frame, 0x10, x0);
        x0 = frf(&frame, 0x1C);
        x0 = mul(x0, x1);
        x1 = frf(&frame, 0x20);
        x1 = mul(x1, rdf(m3));
        frs(&mut frame, 0x1C, x0);
        x0 = frf(&frame, 0x48);
        x0 = mul(x0, x3);
        x3 = frf(&frame, 0x1C);
        x3 = add(x3, x0);
        x0 = frf(&frame, 0x68);
        x0 = mul(x0, x2);
        x3 = add(x3, x0);
        x0 = frf(&frame, 0x50);
        x0 = mul(x0, frf(&frame, 0x3C));
        x3 = add(x3, frf(&frame, 0x78));
        x1 = add(x1, x0);
        x0 = frf(&frame, 0x60);
        x0 = mul(x0, frf(&frame, 0x38));
        x1 = add(x1, x0);
        x0 = frf(&frame, 0x9C);
        frs(&mut frame, 0x44, x0);
        x0 = frf(&frame, 0x88);
        frs(&mut frame, 0x48, x0);
        x0 = frf(&frame, 0x90);
        frs(&mut frame, 0x50, x0);
        x0 = frf(&frame, 0x94);
        frs(&mut frame, 0x40, x1);
        frs(&mut frame, 0x54, x0);
        x0 = frf(&frame, 0x98);
        x7 = frf(&frame, 0x8C);
        frs(&mut frame, 0x58, x0);
        x0 = frf(&frame, 0x10);
        frs(&mut frame, 0x60, x7);
        frs(&mut frame, 0x64, x6);
        frs(&mut frame, 0x68, x5);
        frs(&mut frame, 0x70, x4);
        frs(&mut frame, 0x74, x0);
        frs(&mut frame, 0x78, x3);
        let ans: u32 = lf_checker_rt::callee_thiscall!(
            11, u32,
            aux,
            slot(&mut frame, 0x40) as u32
        );
        ans
    }
}
