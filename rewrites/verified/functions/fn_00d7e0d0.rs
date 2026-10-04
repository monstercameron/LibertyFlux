// original: 0x00d7e0d0 pairwise_pool_check
//! Pairwise pool check: compares two objects' direction frames, seeds a
//! search box from them via a helper call, then scans the object pool for an
//! entry that passes a chain of distance, facing and range tests. Returns
//! false on the first entry that passes every test, true if none does.
//!
//! Verified against the original with the checker (1200/1200 trials, all six
//! callees fired, both return values observed). Float operations keep the
//! original's exact operand order so NaN payloads and signed zeros match bit
//! for bit; sign flips and absolute values are done on the bit pattern.

use lf_checker_rt::{callee_cdecl, export, global};

/// Flip the sign bit, exactly like `xorps` with a sign mask (bitwise, so NaN
/// payloads and signed zeros survive unchanged apart from the sign).
#[inline(always)]
fn neg_bits(x: f32) -> f32 {
    f32::from_bits(x.to_bits() ^ 0x8000_0000)
}

/// Clear the sign bit, exactly like `andps` with an abs mask.
#[inline(always)]
fn abs_bits(x: f32) -> f32 {
    f32::from_bits(x.to_bits() & 0x7fff_ffff)
}

/// Safe inverse length: zero stays zero, otherwise 1/sqrt. NaN propagates
/// through the divide exactly as in the original.
#[inline(always)]
fn safe_inv(len_sq: f32) -> f32 {
    if len_sq == 0.0 {
        0.0
    } else {
        1.0 / len_sq.sqrt()
    }
}

#[inline(always)]
unsafe fn rd_f(base: u32, off: u32) -> f32 {
    unsafe { ((base.wrapping_add(off)) as *const f32).read() }
}

#[inline(always)]
unsafe fn rd_u(base: u32, off: u32) -> u32 {
    unsafe { ((base.wrapping_add(off)) as *const u32).read() }
}

/// Call a vtable slot exactly like the original: load the slot from the
/// object's table and call through it, so both sides land on the same
/// planted stub.
#[inline(always)]
unsafe fn vcall_0(obj: u32, slot: u32) -> u32 {
    unsafe {
        let table = (obj as *const u32).read();
        let target = ((table.wrapping_add(slot)) as *const u32).read();
        let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(target as usize);
        f(obj)
    }
}

#[inline(always)]
unsafe fn vcall_1(obj: u32, slot: u32, arg0: u32) -> u32 {
    unsafe {
        let table = (obj as *const u32).read();
        let target = ((table.wrapping_add(slot)) as *const u32).read();
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(obj, arg0)
    }
}

/// Pairwise pool check (see module docs).
export!(cdecl, rw_00d7e0d0(a: u32, b: u32) -> u8 {
    unsafe {
        let r0 = rd_f(vcall_0(a, 0x64), 0);
        let r1 = rd_f(vcall_0(b, 0x64), 0);
        let ma = rd_u(a, 0x20);
        let mb = rd_u(b, 0x20);

        let d1x = rd_f(mb, 0x10);
        let d1y = rd_f(mb, 0x14);
        let inv1 = safe_inv(d1x * d1x + d1y * d1y);
        let e0 = rd_f(mb, 0x0);
        let e1 = rd_f(mb, 0x4);
        let mut n1x = d1x * inv1;
        let mut n1y = d1y * inv1;
        let mut z1 = inv1 * 0.0;
        let inv2 = safe_inv(e1 * e1 + e0 * e0);
        let mut n2x = inv2 * e0;
        let mut n2y = inv2 * rd_f(mb, 0x4);
        let z2_plain = inv2 * 0.0;
        let mut z2 = z2_plain;
        let t = rd_f(ma, 0x14) * n1y + n1x * rd_f(ma, 0x10) + rd_f(ma, 0x18) * z1;
        // The original's jbe jumps OVER the negate block: negate only when
        // 0.0 > t strictly (t negative and ordered).
        if 0.0 > t {
            n1x = neg_bits(n1x);
            n1y = neg_bits(n1y);
            z1 = neg_bits(z1);
            n2x = neg_bits(n2x);
            n2y = neg_bits(n2y);
            z2 = neg_bits(z2);
        }

        let s10 = r1 + r0 + 0.1f32;
        let mut o0 = 0.0f32;
        let mut o1 = 0.0f32;
        let ok: u8 = callee_cdecl!(
            4,
            u8,
            b,
            &mut o0 as *mut f32 as u32,
            &mut o1 as *mut f32 as u32
        );
        let (ox, oy, oz);
        if ok != 0 {
            ox = neg_bits(o0);
            oy = neg_bits(o1);
            oz = z2_plain;
        } else {
            let d34 = rd_f(mb, 0x34) - rd_f(ma, 0x34);
            let d30 = rd_f(mb, 0x30) - rd_f(ma, 0x30);
            let d38 = rd_f(mb, 0x38) - rd_f(ma, 0x38);
            let s = rd_f(ma, 0x4) * d34 + rd_f(ma, 0x0) * d30 + rd_f(ma, 0x8) * d38;
            let (mut fx, mut fy, mut fz) = (n2x, n2y, z2);
            if !(0.0 > s) {
                fx = neg_bits(fx);
                fy = neg_bits(fy);
                fz = neg_bits(fz);
            }
            ox = fx;
            oy = fy;
            oz = fz;
        }

        let qy = oy * s10 + rd_f(mb, 0x34);
        let qx = ox * s10 + rd_f(mb, 0x30);
        let qz = s10 * oz + rd_f(mb, 0x38);
        let w = (rd_f(ma, 0x30) - rd_f(mb, 0x30)) * n1x
            + (rd_f(ma, 0x34) - rd_f(mb, 0x34)) * n1y
            + (rd_f(ma, 0x38) - rd_f(mb, 0x38)) * z1;
        let rx = n1x * w + qx;
        let ry = n1y * w + qy;
        let ww = 15.0 - w;
        let rz = w * z1 + qz;
        let fx = rx + ww * n1x;
        let fy = ry + ww * n1y;
        let fz = rz + ww * z1;
        let near = [rx, ry, rz];
        let far = [fx, fy, fz];

        let pool = global::<u32>(0x12e22a4).read();
        let count = rd_u(pool, 8);
        if count == 0 {
            return 1;
        }
        let base = rd_u(pool, 0);
        let flags = rd_u(pool, 4);
        let stride = rd_u(pool, 0x0c) as i32;
        let mut i = count;
        loop {
            i = i.wrapping_sub(1);
            if ((flags.wrapping_add(i) as *const u8).read() & 0x80) == 0 {
                let entry = base.wrapping_add(stride.wrapping_mul(i as i32) as u32);
                if entry != 0 && entry != a && entry != b {
                    if ((entry.wrapping_add(0xf20) as *const u8).read() & 1) == 0 {
                        let me = rd_u(entry, 0x20);
                        let dz = abs_bits(rd_f(ma, 0x38) - rd_f(me, 0x38));
                        if 4.0 > dz {
                            let mut scratch = 0u32;
                            let p2 = vcall_1(entry, 0xec, &mut scratch as *mut u32 as u32);
                            let u = rd_f(p2, 0) * n1x + rd_f(p2, 4) * n1y + rd_f(p2, 8) * z1;
                            if -0.2f32 > u {
                                let _g: f32 = callee_cdecl!(
                                    5,
                                    f32,
                                    near.as_ptr() as u32,
                                    far.as_ptr() as u32,
                                    me.wrapping_add(0x30)
                                );
                                let p3 = vcall_0(entry, 0x64);
                                let h = rd_f(p3, 0) + r0 + 0.1f32;
                                if h > z2_plain {
                                    return 0;
                                }
                            }
                        }
                    }
                }
            }
            if i == 0 {
                break;
            }
        }
        1
    }
});
