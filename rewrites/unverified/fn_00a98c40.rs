// original: 0x00a98c40 attach_and_dispatch_state (stage 1)
/// Attaches a state object to a parameter block, resolves it through a
/// dispatch table, and runs the tail configuration.
///
/// `this` points to the state being attached, `a0` to the parameter block
/// and `a1` is a slot index. The function links the two, evaluates a
/// floating-point probe through a helper (whose out-structure yields both a
/// status flag and the working object pointer), sets a bit in the block's
/// occupancy table, resolves the working object through two virtual
/// dispatches, runs the fallback scan when the region flag is clear, and
/// finishes with constant stamps plus a masked mode value, which it also
/// returns.
///
/// Stage 1 covers the direct-tag dispatch path, the fallback scan and the
/// tail. The counted-child loop and the flagged region are stage 2 (marked
/// below); the stage-1 contract never reaches them.
export!(thiscall, rw_a98c40(this_: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        ((this_.wrapping_add(0x68)) as *mut u32).write_unaligned(a0);
        lf_checker_rt::callee_thiscall!(1, u32, a0, this_.wrapping_add(0x68));
        let back = (this_.wrapping_add(0x68) as *const u32).read_unaligned();
        let sx = (back.wrapping_add(0x2e) as *const i16).read_unaligned() as i32;
        (this_.wrapping_add(0x64) as *mut u32).write_unaligned(sx as u32);
        let vp = (a0.wrapping_add(0x20) as *const u32).read_unaligned();
        let (vx, vy, vz) = if vp != 0 {
            let b = vp.wrapping_add(0x30);
            ((b as *const f32).read_unaligned(),
             ((b.wrapping_add(4)) as *const f32).read_unaligned(),
             ((b.wrapping_add(8)) as *const f32).read_unaligned())
        } else {
            (((a0.wrapping_add(0x10)) as *const f32).read_unaligned(),
             ((a0.wrapping_add(0x14)) as *const f32).read_unaligned(),
             ((a0.wrapping_add(0x18)) as *const f32).read_unaligned())
        };
        let mut out = [0u32; 6];
        let fr = lf_checker_rt::callee_cdecl!(2, f32, vx.to_bits(), vy.to_bits(), vz.to_bits(),
            out.as_mut_ptr() as u32, 0, 4);
        (this_.wrapping_add(0x6c) as *mut f32).write_unaligned(fr);
        let edi = (out[3] >> 8) | (out[4] << 24);
        if out[0] & 0xFF == 0 {
            let f = f32::from_bits((out[4] >> 8) | (out[5] << 24)) - 10.0;
            (this_.wrapping_add(0x6c) as *mut f32).write_unaligned(f);
        }
        let wi = (a1 as i32).wrapping_shr(5);
        let bit = 1u32 << (a1 & 0x1F);
        let ba = (a0 as i32).wrapping_add(wi.wrapping_mul(4)).wrapping_add(0x1ac) as u32;
        let old = (ba as *const u32).read_unaligned();
        (ba as *mut u32).write_unaligned(old | bit);
        (edi.wrapping_add(0x70) as *mut u32).write_unaligned(0);
        (edi.wrapping_add(8) as *mut u32).write_unaligned(a1);
        lf_checker_rt::callee_thiscall!(3, u32, lf_checker_rt::relocated(0x1305d30), a0);
        let v1 = (edi.wrapping_add(0x68) as *const u32).read_unaligned();
        let vt = (v1 as *const u32).read_unaligned();
        let f1: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute((((vt.wrapping_add(0xa0)) as *const u32).read_unaligned()));
        let r1 = f1(v1);
        let e2 = if r1 != 0 {
            r1
        } else {
            (v1.wrapping_add(0x38) as *const u32).read_unaligned()
        };
        let e3 = (e2.wrapping_add(4) as *const u32).read_unaligned();
        let cy = (e3.wrapping_add(0xc) as *const u32).read_unaligned();
        if ((cy.wrapping_add(4)) as *const u8).read_unaligned() == 0x0c {
            let t1 = (cy.wrapping_add(0x80) as *const u32).read_unaligned();
            let row = (t1.wrapping_add(a1.wrapping_mul(4)) as *const u32).read_unaligned();
            let u = (row as *const u32).read_unaligned();
            let f2: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(((u.wrapping_add(0x5c)) as *const u32).read_unaligned());
            let r2 = f2(row, 0);
            (edi.wrapping_add(0x60) as *mut u32).write_unaligned(r2 & 0xFF);
        } else {
            unreachable!("stage 2: counted-child loop");
        }
        (edi.wrapping_add(0x87) as *mut u8).write_unaligned(0);
        if ((a0.wrapping_add(0x24)) as *const u32).read_unaligned() & 0x04000000 != 0 {
            unreachable!("stage 2: flagged region");
        }
        let r2obj = (a0.wrapping_add(0x34) as *const u32).read_unaligned();
        if r2obj != 0 {
            let r2b = (r2obj as *const u32).read_unaligned();
            if r2b != 0 {
                let r2c = ((r2b.wrapping_add(0x40)) as *const u32).read_unaligned();
                let r2d = (r2c as *const u32).read_unaligned();
                let r2e = (r2d as *const u32).read_unaligned();
                let count = ((r2e.wrapping_add(0x1a)) as *const u16).read_unaligned() as i32;
                let mut i: i32 = 0;
                while i < count {
                    let r2f = ((r2e.wrapping_add(0x10)) as *const u32).read_unaligned();
                    let sx2 = (((r2f.wrapping_add((i as u32).wrapping_mul(2))) as *const u16)
                        .read_unaligned()) as u32;
                    let r2g = ((r2b.wrapping_add(8)) as *const u32).read_unaligned();
                    let r2h = ((r2g.wrapping_add(8)) as *const u32).read_unaligned();
                    let w = ((r2h.wrapping_add(sx2.wrapping_mul(4))) as *const u32).read_unaligned();
                    let g = lf_checker_rt::callee_thiscall!(4, u32, lf_checker_rt::relocated(0x1305d30), w);
                    if (g as u8) != 0 {
                        lf_checker_rt::callee_thiscall!(5, u32, edi, a0, r2b, r2e, i as u32, 0xFFFFFFFFu32);
                    }
                    i += 1;
                }
            }
        }
        (edi.wrapping_add(0x78) as *mut u32).write_unaligned(0);
        (edi.wrapping_add(0x84) as *mut u16).write_unaligned(0);
        (edi.wrapping_add(0x86) as *mut u8).write_unaligned(0);
        (edi.wrapping_add(0x8c) as *mut u32).write_unaligned(0x3D99999A);
        (edi.wrapping_add(0x88) as *mut u32).write_unaligned(0x3CA3D70A);
        let d8 = lf_checker_rt::callee_thiscall!(6, u32, a0);
        if (d8 as u8) == 0 {
            (edi.wrapping_add(0x85) as *mut u8).write_unaligned(1);
        } else if ((edi.wrapping_add(0x87)) as *const u8).read_unaligned() != 0 {
            (edi.wrapping_add(0x85) as *mut u8).write_unaligned(1);
        }
        let m = ((a0.wrapping_add(0x28)) as *const u32).read_unaligned() & 0x3C0;
        (edi.wrapping_add(0x90) as *mut u32).write_unaligned(0);
        if m == 0x80 {
            (edi.wrapping_add(0x85) as *mut u8).write_unaligned(1);
        }
        m
    }
});
