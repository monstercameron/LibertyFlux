// original: 0x00a97a30 cook_indexed_mesh
//! Rewrite of the collision-mesh builder at file VA 0x00A97A30.
//!
//! `thiscall(this, a0, a1, a2, a3, a4) -> u32`: fetches up to 1000 vertices
//! through helpers, keeps the non-degenerate front-facing triangles, links
//! triangle adjacency, emits one output object per group id, and returns the
//! number of registered triangles. See the lane report for the full analysis.

use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

// Callee ids (match the contract).
const C_FACTORY: u32 = 1;
const C_VT14: u32 = 2;
const C_VT20: u32 = 3;
const C_NEW: u32 = 4;
const C_CTOR: u32 = 5;
const C_VT28: u32 = 6;
const C_E8P4: u32 = 7;
const C_VFETCH: u32 = 8;
const C_SETUP: u32 = 9;
const C_ATTR1: u32 = 10;
const C_ATTR2: u32 = 11;
const C_VATTR: u32 = 12;
const C_IATTR: u32 = 13;
const C_PATTR: u32 = 14;
const C_FLAGQ: u32 = 15;
const C_XATTR: u32 = 16;
const C_REGISTER: u32 = 17;
const C_ALLOC_OBJ: u32 = 18;
const C_INIT_OBJ: u32 = 19;
const C_EMIT_TRI: u32 = 20;
const C_FINISH_OBJ: u32 = 21;
const C_RELEASE: u32 = 22;
const C_DTOR: u32 = 23;

// File VAs of data the function reads.
const VA_ORIGIN: u32 = 0x0128_E340; // 3 floats subtracted from the setup translation
const VA_RANGE_LO: u32 = 0x00FE_88DC;
const VA_RANGE_HI: u32 = 0x00FE_88F4;
const VA_ONE: u32 = 0x00FE_88E8;
const VA_ABS_LIKE_MASK: u32 = 0x00FE_8F80; // 0x7FFFFFFF, used as an and-mask
const VA_EPS: u32 = 0x00FE_86EC; // adjacency epsilon
const VA_GLOBAL_OBJ: u32 = 0x0130_5D30; // singleton passed as `this` to C_ALLOC_OBJ

const MAX_VERTS: u32 = 1000;
// Bounding-box seeds, exact immediates from the original (99999.0, -99999.0).
const BBOX_INIT: f32 = f32::from_bits(0x47C3_4F80);
const BBOX_NINIT: f32 = f32::from_bits(0xC7C3_4F80);

#[inline(always)]
unsafe fn rd_u32(addr: u32) -> u32 {
    unsafe { (addr as *const u32).read() }
}

#[inline(always)]
unsafe fn rd_f32(addr: u32) -> f32 {
    unsafe { (addr as *const f32).read() }
}

#[inline(always)]
unsafe fn rd_u16(addr: u32) -> u16 {
    unsafe { (addr as *const u16).read() }
}

#[inline(always)]
unsafe fn rd_u8(addr: u32) -> u8 {
    unsafe { (addr as *const u8).read() }
}

#[inline(always)]
unsafe fn wr_u32(addr: u32, v: u32) {
    unsafe { (addr as *mut u32).write(v) }
}

#[inline(always)]
unsafe fn wr_f32(addr: u32, v: f32) {
    unsafe { (addr as *mut f32).write(v) }
}

#[inline(always)]
unsafe fn wr_u8(addr: u32, v: u8) {
    unsafe { (addr as *mut u8).write(v) }
}

/// Call a `thiscall/0` function through a vtable slot, like the original.
#[inline(always)]
unsafe fn vt_call_0(obj: u32, slot: u32) -> u32 {
    unsafe {
        let vtbl = rd_u32(obj);
        let target = rd_u32(vtbl.wrapping_add(slot));
        let f: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(target as usize);
        f(obj)
    }
}

/// Call a `thiscall/1` function through a vtable slot, like the original.
#[inline(always)]
unsafe fn vt_call_1(obj: u32, slot: u32, a: u32) -> u32 {
    unsafe {
        let vtbl = rd_u32(obj);
        let target = rd_u32(vtbl.wrapping_add(slot));
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(obj, a)
    }
}

/// Reciprocal length: `1/sqrt(x)` unless `x` is an ordered zero.
#[inline(always)]
fn rlen(x: f32) -> f32 {
    if x != 0.0 {
        let one = unsafe { rd_f32(global::<f32>(VA_ONE) as u32) };
        one / x.sqrt()
    } else {
        0.0
    }
}

export!(thiscall, rw_00a97a30(
    this: u32, a0: u32, _a1: u32, a2: u32, a3: u32, a4: u32,
) -> u32 {
    unsafe { body(this, a0, a2, a3, a4) }
});

unsafe fn body(this: u32, a0: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        let positive = (a4 as i32) >= 0;
        // Factory object and its three vtable queries.
        let factory = callee_thiscall!(C_FACTORY, u32, a2, a3);
        let word_table = vt_call_0(factory, 0x14);
        let vert_src = vt_call_1(factory, 0x20, 0);
        let count = rd_u16(vert_src.wrapping_add(4)) as u32;
        if count > MAX_VERTS {
            return 0;
        }
        // Helper object (heap-allocated by the callee pair).
        let mut helper: u32 = 0;
        let fresh = callee_cdecl!(C_NEW, u32, 0x10u32);
        if fresh != 0 {
            helper = callee_thiscall!(C_CTOR, u32, fresh, vert_src, 1, 1, 0);
        }
        // Index source object.
        let idx_src = vt_call_1(factory, 0x28, 0);
        let tri_aux = rd_u32(idx_src.wrapping_add(4));
        let idx_vtbl = rd_u32(idx_src);
        let idx_target = rd_u32(idx_vtbl.wrapping_add(4));
        let idx_fn: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(idx_target as usize);
        let idx_buf = idx_fn(idx_src);
        // Fetch every vertex: 3 floats + a tag dword.
        let mut verts: std::vec::Vec<[u32; 4]> =
            std::vec::Vec::with_capacity(count as usize);
        {
            let mut out = [0u32; 4];
            for i in 0..count {
                let vp = callee_thiscall!(
                    C_VFETCH, u32, helper, out.as_mut_ptr() as u32, i,
                );
                verts.push([
                    rd_u32(vp),
                    rd_u32(vp.wrapping_add(4)),
                    rd_u32(vp.wrapping_add(8)),
                    rd_u32(vp.wrapping_add(12)),
                ]);
            }
        }
        let get_pos = |idx: u32| -> (f32, f32, f32) {
            let w = verts[idx as usize];
            (
                f32::from_bits(w[0]),
                f32::from_bits(w[1]),
                f32::from_bits(w[2]),
            )
        };
        // Setup block: 3x3 rows plus a translation.
        let select = if positive {
            a4
        } else {
            rd_u8(a2.wrapping_add(0x17)) as u32
        };
        let mut setup = [0u32; 15];
        callee_cdecl!(
            C_SETUP, u32, setup.as_mut_ptr() as u32, a0, select, this,
        );
        let rf = |i: usize| f32::from_bits(setup[i]);
        let gx = rd_f32(global::<f32>(VA_ORIGIN) as u32);
        let gy = rd_f32(global::<f32>(VA_ORIGIN + 4) as u32);
        let gz = rd_f32(global::<f32>(VA_ORIGIN + 8) as u32);
        let dx = gx - rf(12);
        let dy = gy - rf(13);
        let dz = gz - rf(14);
        // Note the multiply order: row_y*dy + row_x*dx + row_z*dz.
        let c0 = rf(1) * dy + rf(0) * dx + rf(2) * dz;
        let c1 = rf(5) * dy + rf(4) * dx + rf(6) * dz;
        let c2 = rf(9) * dy + rf(8) * dx + rf(10) * dz;
        // Triangle scan.
        let tri_count = tri_aux / 3;
        // 7 words per record: 3 indices, 3 adjacency slots, group id. The
        // original reserves tri_aux*28 bytes; slots past the recorded ones
        // keep the stack fill, and the register stub's snapshot reads into
        // them, so they must exist and read zero.
        let mut tris: std::vec::Vec<[u32; 7]> =
            vec![[0u32; 7]; (tri_aux as usize).wrapping_mul(4)];
        let mut nrec = 0u32;
        let lo = rd_f32(global::<f32>(VA_RANGE_LO) as u32);
        let hi = rd_f32(global::<f32>(VA_RANGE_HI) as u32);
        let mut ip = idx_buf.wrapping_add(4);
        for _ in 0..tri_count {
            let i0 = rd_u16(ip.wrapping_sub(4)) as u32;
            let i1 = rd_u16(ip.wrapping_sub(2)) as u32;
            let i2 = rd_u16(ip) as u32;
            ip = ip.wrapping_add(6);
            if i0 == i1 || i1 == i2 || i0 == i2 {
                continue;
            }
            let pa = get_pos(i0);
            let pb = get_pos(i1);
            let pc = get_pos(i2);
            if pa.0 == pb.0 && pa.1 == pb.1 && pa.2 == pb.2 {
                continue;
            }
            if pa.0 == pc.0 && pa.1 == pc.1 && pa.2 == pc.2 {
                continue;
            }
            if positive {
                let mut aout = [0u32; 4];
                let p0 = callee_thiscall!(
                    C_ATTR1, u32, helper, aout.as_mut_ptr() as u32, i0,
                );
                let v0 = rd_f32(p0);
                let p1 = callee_thiscall!(
                    C_ATTR1, u32, helper, aout.as_mut_ptr() as u32, i1,
                );
                let v1 = rd_f32(p1);
                let p2 = callee_thiscall!(
                    C_ATTR1, u32, helper, aout.as_mut_ptr() as u32, i2,
                );
                let v2 = rd_f32(p2);
                let q = callee_thiscall!(
                    C_ATTR2, u32, helper, aout.as_mut_ptr() as u32, i0,
                );
                let qb = rd_u8(q.wrapping_add(2)) as u32;
                // Range gate with one tolerated upper miss, transcribed exactly.
                if lo > v0 {
                    continue;
                }
                if v0 > hi {
                    if lo > v1 {
                        continue;
                    }
                    if v1 > hi {
                        continue;
                    }
                } else if v1 > hi {
                    if lo > v2 {
                        continue;
                    }
                    if v2 > hi {
                        continue;
                    }
                } else if v2 > hi {
                    continue;
                }
                let tag = rd_u16(word_table.wrapping_add(qb * 2)) as u32;
                if tag != a4 {
                    continue;
                }
            }
            // Face normal = (pb - pa) x (pc - pa).
            let t1 = pc.1 - pa.1;
            let t1p = pc.2 - pa.2;
            let t2 = pb.2 - pa.2;
            let t4 = pb.1 - pa.1;
            let t5 = pb.0 - pa.0;
            let t3 = pc.0 - pa.0;
            let mut nx = t1p * t4 - t1 * t2;
            let mut ny = t3 * t2 - t1p * t5;
            let mut nz = t1 * t5 - t3 * t4;
            let len2 = ny * ny + nx * nx + nz * nz;
            let inv = rlen(len2);
            nx *= inv;
            ny *= inv;
            nz *= inv;
            // Reference direction: negated pa, or pa minus the setup center.
            let (ddx, ddy, ddz) = if positive {
                (-pa.0, -pa.1, -pa.2)
            } else {
                (pa.0 - c0, pa.1 - c1, pa.2 - c2)
            };
            let dlen2 = ddy * ddy + ddx * ddx + ddz * ddz;
            let dinv = rlen(dlen2);
            let dxn = ddx * dinv;
            let dyn_ = ddy * dinv;
            let dzn = ddz * dinv;
            let dot = dyn_ * ny + dxn * nx + dzn * nz;
            // comiss+jb: skip when ordered-above zero, or unordered.
            if dot.is_nan() || dot > 0.0 {
                continue;
            }
            tris[nrec as usize] =
                [i0, i1, i2, 0xFFFF_FFFF, 0xFFFF_FFFF, 0xFFFF_FFFF, 0];
            nrec += 1;
        }
        if nrec == 0 {
            finish(idx_src, helper, 0);
            return 0;
        }
        adjacency_and_emit(
            this, a3, helper, idx_src, vert_src, &verts, &mut tris, nrec,
            select,
        )
    }
}

/// Closeness test used by the adjacency pass: every component difference,
/// masked with the image constant, must be strictly below epsilon.
fn close3(
    a: (f32, f32, f32), b: (f32, f32, f32), eps: f32, mask: u32,
) -> bool {
    let ca = [a.0 - b.0, a.1 - b.1, a.2 - b.2];
    for d in ca {
        let m = f32::from_bits(d.to_bits() & mask);
        if !(eps > m) {
            return false;
        }
    }
    true
}

unsafe fn adjacency_and_emit(
    this: u32, a3: u32, helper: u32, idx_src: u32, vert_src: u32,
    verts: &[[u32; 4]], tris: &mut [[u32; 7]], nrec: u32, select: u32,
) -> u32 {
    unsafe {
        let pos = |idx: u32| -> (f32, f32, f32) {
            let w = verts[idx as usize];
            (
                f32::from_bits(w[0]),
                f32::from_bits(w[1]),
                f32::from_bits(w[2]),
            )
        };
        let eps = rd_f32(global::<f32>(VA_EPS) as u32);
        let mask = rd_u32(global::<u32>(VA_ABS_LIKE_MASK) as u32);
        // Adjacency: pairs sharing exactly two close vertices are linked.
        for o in 0..nrec as usize {
            for i in (o + 1)..nrec as usize {
                let mut matches = 0u32;
                for k in 0..3 {
                    let op = pos(tris[o][k]);
                    for w in 0..3 {
                        if close3(op, pos(tris[i][w]), eps, mask) {
                            matches += 1;
                        }
                    }
                }
                if matches != 2 {
                    continue;
                }
                for k in 0..3 {
                    if tris[o][3 + k] == 0xFFFF_FFFF {
                        tris[o][3 + k] = i as u32;
                        break;
                    }
                }
                for k in 0..3 {
                    if tris[i][3 + k] == 0xFFFF_FFFF {
                        tris[i][3 + k] = o as u32;
                        break;
                    }
                }
            }
        }
        for t in tris.iter_mut() {
            t[6] = 0xFFFF_FFFF;
        }
        // Register every triangle; the callee assigns group ids in w6.
        let buf = tris.as_mut_ptr() as u32;
        let mut nreg = 0u32;
        for t in 0..nrec as usize {
            if tris[t][6] == 0xFFFF_FFFF {
                callee_thiscall!(
                    C_REGISTER, u32, this, buf, nrec, t as u32, nreg,
                );
                nreg += 1;
            }
        }
        if nreg == 0 {
            finish(idx_src, helper, 0);
            return 0;
        }
        let one = rd_f32(global::<f32>(VA_ONE) as u32);
        let flag_this = rd_u32(vert_src.wrapping_add(0x10));
        let global_obj = relocated(VA_GLOBAL_OBJ);
        let mut w124 = 0.0f32;
        for g in 0..nreg {
            let obj =
                callee_thiscall!(C_ALLOC_OBJ, u32, global_obj);
            if obj == 0 {
                continue;
            }
            callee_thiscall!(C_INIT_OBJ, u32, obj, a3, select);
            for t in 0..nrec as usize {
                if tris[t][6] != g {
                    continue;
                }
                let ia = tris[t][0];
                let ib = tris[t][1];
                let ic = tris[t][2];
                let mut tmp = [0u32; 4];
                let pa = callee_thiscall!(
                    C_VATTR, u32, helper, tmp.as_mut_ptr() as u32, ia,
                );
                let fa = [
                    rd_u32(pa),
                    rd_u32(pa.wrapping_add(4)),
                    rd_u32(pa.wrapping_add(8)),
                    rd_u32(pa.wrapping_add(12)),
                ];
                let pb = callee_thiscall!(
                    C_VATTR, u32, helper, tmp.as_mut_ptr() as u32, ib,
                );
                let fb = [
                    rd_u32(pb),
                    rd_u32(pb.wrapping_add(4)),
                    rd_u32(pb.wrapping_add(8)),
                    rd_u32(pb.wrapping_add(12)),
                ];
                let pc = callee_thiscall!(
                    C_VATTR, u32, helper, tmp.as_mut_ptr() as u32, ic,
                );
                let fc = [
                    rd_u32(pc),
                    rd_u32(pc.wrapping_add(4)),
                    rd_u32(pc.wrapping_add(8)),
                    rd_u32(pc.wrapping_add(12)),
                ];
                let qa = callee_thiscall!(
                    C_IATTR, u32, helper, tmp.as_mut_ptr() as u32, ia,
                );
                let da = rd_u32(qa);
                let qb = callee_thiscall!(
                    C_IATTR, u32, helper, tmp.as_mut_ptr() as u32, ib,
                );
                let db = rd_u32(qb);
                let qc = callee_thiscall!(
                    C_IATTR, u32, helper, tmp.as_mut_ptr() as u32, ic,
                );
                let dc = rd_u32(qc);
                let ra = callee_thiscall!(
                    C_PATTR, u32, helper, tmp.as_mut_ptr() as u32, ia, 0,
                );
                let pa0 = rd_u32(ra);
                let pa1 = one - rd_f32(ra.wrapping_add(4));
                let rb = callee_thiscall!(
                    C_PATTR, u32, helper, tmp.as_mut_ptr() as u32, ib, 0,
                );
                let pb0 = rd_u32(rb);
                let pb1 = one - rd_f32(rb.wrapping_add(4));
                let rc = callee_thiscall!(
                    C_PATTR, u32, helper, tmp.as_mut_ptr() as u32, ic, 0,
                );
                let pc0 = rd_u32(rc);
                let pc1 = one - rd_f32(rc.wrapping_add(4));
                let mut xa = [0u32; 4];
                let mut xb = [0u32; 4];
                let mut xc = [0u32; 4];
                let h = callee_thiscall!(C_FLAGQ, u32, flag_this, 0);
                if h as u8 != 0 {
                    // Separate out-params per call, like the original; the
                    // bbox weight aliases the third one's last word.
                    let mut out_a = [0u32; 4];
                    let mut out_b = [0u32; 4];
                    let mut out_c = [0u32; 4];
                    let sa = callee_thiscall!(
                        C_XATTR, u32, helper, out_a.as_mut_ptr() as u32, ia, 0,
                    );
                    xa = [
                        rd_u32(sa),
                        rd_u32(sa.wrapping_add(4)),
                        rd_u32(sa.wrapping_add(8)),
                        rd_u32(sa.wrapping_add(12)),
                    ];
                    let sb = callee_thiscall!(
                        C_XATTR, u32, helper, out_b.as_mut_ptr() as u32, ib, 0,
                    );
                    xb = [
                        rd_u32(sb),
                        rd_u32(sb.wrapping_add(4)),
                        rd_u32(sb.wrapping_add(8)),
                        rd_u32(sb.wrapping_add(12)),
                    ];
                    let sc = callee_thiscall!(
                        C_XATTR, u32, helper, out_c.as_mut_ptr() as u32, ic, 0,
                    );
                    xc = [
                        rd_u32(sc),
                        rd_u32(sc.wrapping_add(4)),
                        rd_u32(sc.wrapping_add(8)),
                        rd_u32(sc.wrapping_add(12)),
                    ];
                    w124 = f32::from_bits(out_c[3]);
                }
                let mut blk_a = [pa0, pa1.to_bits()];
                let mut blk_b = [pb0, pb1.to_bits()];
                let mut blk_c = [pc0, pc1.to_bits()];
                let vptr = verts.as_ptr() as u32;
                callee_thiscall!(
                    C_EMIT_TRI, u32, obj, vptr.wrapping_add(ia * 16),
                    vptr.wrapping_add(ib * 16),
                    vptr.wrapping_add(ic * 16), fa.as_ptr() as u32,
                    fb.as_ptr() as u32, fc.as_ptr() as u32,
                    blk_a.as_mut_ptr() as u32, blk_b.as_mut_ptr() as u32,
                    blk_c.as_mut_ptr() as u32, &da as *const u32 as u32,
                    &db as *const u32 as u32, &dc as *const u32 as u32,
                    xa.as_ptr() as u32, xb.as_ptr() as u32,
                    xc.as_ptr() as u32,
                );
                wr_u8(obj.wrapping_add(0x14), 0);
                wr_u32(obj.wrapping_add(0x18), 0);
            }
            // Bounding box over the object's vertex chain.
            let mut bmin = [BBOX_INIT, BBOX_INIT, BBOX_INIT];
            let mut bmax = [BBOX_NINIT, BBOX_NINIT, BBOX_NINIT];
            let mut node = rd_u32(obj.wrapping_add(0x0C));
            while node != 0 {
                let mut vp = node.wrapping_add(0x18);
                for _ in 0..3 {
                    let x = rd_f32(vp.wrapping_sub(8));
                    let y = rd_f32(vp.wrapping_sub(4));
                    let z = rd_f32(vp);
                    if !(x > bmin[0]) {
                        bmin[0] = x;
                    }
                    if !(y > bmin[1]) {
                        bmin[1] = y;
                    }
                    if !(z > bmin[2]) {
                        bmin[2] = z;
                    }
                    if !(bmax[0] > x) {
                        bmax[0] = x;
                    }
                    if !(bmax[1] > y) {
                        bmax[1] = y;
                    }
                    if !(bmax[2] > z) {
                        bmax[2] = z;
                    }
                    vp = vp.wrapping_add(0x10);
                }
                node = rd_u32(node);
            }
            // Transform both corners by the owner's matrices.
            let t1 = rd_u32(this.wrapping_add(0x68));
            let m1 = rd_u32(t1.wrapping_add(0x20));
            let m = |o: u32| rd_f32(m1.wrapping_add(o));
            let ox = bmin[1] * m(0x10) + bmin[0] * m(0) + bmin[2] * m(0x20)
                + m(0x30);
            let oy = bmin[1] * m(0x14) + bmin[0] * m(4) + bmin[2] * m(0x24)
                + m(0x34);
            let oz = bmin[1] * m(0x18) + bmin[0] * m(8) + bmin[2] * m(0x28)
                + m(0x38);
            wr_f32(obj.wrapping_add(0x90), ox);
            wr_f32(obj.wrapping_add(0x9C), w124);
            wr_f32(obj.wrapping_add(0x94), oy);
            wr_f32(obj.wrapping_add(0x98), oz);
            // Note: ecx already holds this+0x0c here, so this reads
            // [this+0x68] again, not [this+0x5c].
            let t2 = rd_u32(this.wrapping_add(0x68));
            let m2 = rd_u32(t2.wrapping_add(0x20));
            let n = |o: u32| rd_f32(m2.wrapping_add(o));
            let qx = bmax[1] * n(0x10) + bmax[0] * n(0) + bmax[2] * n(0x20)
                + n(0x30);
            let qy = bmax[1] * n(0x14) + bmax[0] * n(4) + bmax[2] * n(0x24)
                + n(0x34);
            let qz = bmax[1] * n(0x18) + bmax[0] * n(8) + bmax[2] * n(0x28)
                + n(0x38);
            wr_f32(obj.wrapping_add(0xA0), qx);
            wr_f32(obj.wrapping_add(0xA4), qy);
            wr_f32(obj.wrapping_add(0xAC), w124);
            wr_f32(obj.wrapping_add(0xA8), qz);
            callee_thiscall!(C_FINISH_OBJ, u32, this.wrapping_add(0x0C), obj);
        }
        finish(idx_src, helper, nreg);
        nreg
    }
}

/// Release the index source and destroy the helper, then return `rv`.
unsafe fn finish(idx_src: u32, helper: u32, rv: u32) -> u32 {
    unsafe {
        vt_call_0(idx_src, 0x0C);
        if helper != 0 {
            vt_call_1(helper, 0x00, 1);
        }
        rv
    }
}
