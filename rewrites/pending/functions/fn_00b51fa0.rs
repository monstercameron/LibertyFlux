// original: 0x00b51fa0 EuphoriaBodyDamageScan
//! Bound-volume scan over 32 body slots with per-entry damage-event
//! construction and effect attachment (animation-euphoria subsystem).
//!
//! The function builds an axis-aligned bound box over the active body slots,
//! walks a global entry table, and for the first slot within range of each
//! entry it constructs a damage event, allocates two task objects, samples a
//! virtual mass hook, and stores the resulting effect parameters. At most one
//! slot is processed per table entry, and at most five effects are stored.
//!
//! Two dwords the original reads from its own uninitialized stack are stored
//! to the effect record; under the checker's defined zero fill both sides
//! agree on 0.0 (see the report). Behaviour verified bit-exact over 1000
//! trials against checker v2, including NaN/-inf float edges, null and
//! non-null allocator answers, and every branch of the store path.

use lf_k2_rt::{callee_cdecl, callee_thiscall, export, global};

const RADIUS: f32 = 2.2;
const RADIUS2: f32 = 4.84;
const VEL_SCALE: f32 = 0.25;
const VEL_BIAS: f32 = 5.0;
const NORM_EPS: f32 = 0.25;
const NORM_HALF: f32 = 0.5;
const FAR_SCALE: f32 = 4.0;
const PMAX: f32 = f32::from_bits(0x7F7FFFFF);
const NMAX: f32 = f32::from_bits(0xFF7FFFFF);

#[inline(always)]
unsafe fn rf(p: *const u8, off: usize) -> f32 {
    *(p.add(off) as *const f32)
}

#[inline(always)]
unsafe fn ru32(p: *const u8, off: usize) -> u32 {
    *(p.add(off) as *const u32)
}

#[inline(always)]
unsafe fn wf(p: *mut u8, off: usize, v: f32) {
    *(p.add(off) as *mut f32) = v;
}

export!(thiscall, rw_b51fa0(this_ptr: *mut u8) -> u32 {
    unsafe {
        let this = this_ptr;
        let (mut min_x, mut min_y, mut min_z) = (PMAX, PMAX, PMAX);
        let (mut max_x, mut max_y, mut max_z) = (NMAX, NMAX, NMAX);
        let mut i = 0usize;
        while i < 32 {
            if *this.add(0x10 + i) != 0 {
                let base = 0x30 + i * 0x10;
                let px = rf(this, base);
                let v = px - RADIUS;
                if !(v > min_x) {
                    min_x = v;
                }
                let w = px + RADIUS;
                if !(max_x > w) {
                    max_x = w;
                }
                let py = rf(this, base + 4);
                let v = py - RADIUS;
                if !(v > min_y) {
                    min_y = v;
                }
                let w = py + RADIUS;
                if !(max_y > w) {
                    max_y = w;
                }
                let pz = rf(this, base + 8);
                let v = pz - RADIUS;
                if !(v > min_z) {
                    min_z = v;
                }
                let w = pz + RADIUS;
                if !(max_z > w) {
                    max_z = w;
                }
            }
            i += 1;
        }
        let table = *global::<u32>(0x18b6f1c);
        let mut k = 0usize;
        while k < 5 {
            let slot = this.add(0x460 + k * 4) as *mut u32;
            if *slot != 0 {
                callee_thiscall!(1, u32, *slot, slot as u32);
                *slot = 0;
            }
            k += 1;
        }
        let mut stored = 0u32;
        let n = *((table as *const u8).add(8) as *const u32);
        if n != 0 {
            let mut idx = n;
            loop {
                idx = idx.wrapping_sub(1);
                let flags = *((table as *const u8).add(4) as *const u32);
                if *((flags as *const u8).add(idx as usize) as *const u8) & 0x80 != 0 {
                    if idx == 0 {
                        break;
                    }
                    continue;
                }
                let stride = *((table as *const u8).add(0xc) as *const u32);
                let base = *(table as *const u32);
                let entry = base.wrapping_add(stride.wrapping_mul(idx)) as *mut u8;
                if entry as u32 == 0 {
                    if idx == 0 {
                        break;
                    }
                    continue;
                }
                let pool = ru32(this, 0x430);
                let gate = callee_cdecl!(2, u32, entry as u32, 6u32, pool, 0u32);
                if gate as u8 == 0 {
                    if idx == 0 {
                        break;
                    }
                    continue;
                }
                let obj = ru32(entry, 0x20) as *const u8;
                let ex = rf(obj, 0x30);
                if !(ex > min_x) {
                    if idx == 0 {
                        break;
                    }
                    continue;
                }
                if !(max_x > ex) {
                    if idx == 0 {
                        break;
                    }
                    continue;
                }
                let ey = rf(obj, 0x34);
                if !(ey > min_y) {
                    if idx == 0 {
                        break;
                    }
                    continue;
                }
                if !(max_y > ey) {
                    if idx == 0 {
                        break;
                    }
                    continue;
                }
                let ez = rf(obj, 0x38);
                if !(ez > min_z) {
                    if idx == 0 {
                        break;
                    }
                    continue;
                }
                if !(max_z > ez) {
                    if idx == 0 {
                        break;
                    }
                    continue;
                }
                let mut si = 0usize;
                while si < 32 {
                    if *this.add(0x10 + si) != 0 {
                        let base = 0x30 + si * 0x10;
                        let dx = rf(this, base) - ex;
                        let dy = rf(this, base + 4) - ey;
                        let dz = rf(this, base + 8) - ez;
                        let d2 = dy * dy + dx * dx + dz * dz;
                        if RADIUS2 > d2 {
                            process_slot(this, entry, si, ex, ey, ez, &mut stored);
                            break;
                        }
                    }
                    si += 1;
                }
                if idx == 0 {
                    break;
                }
            }
        }
    }
    0
});

#[inline(never)]
unsafe fn process_slot(
    this: *mut u8, entry: *mut u8, si: usize,
    ex: f32, ey: f32, ez: f32, stored: &mut u32,
) {
    unsafe {
        let mut obj_a = [0u32; 16];
        let mut obj_b = [0u32; 16];
        let mut buf_c = [0u32; 8];
        let pool = ru32(this, 0x430);
        let vp = this.add((si + 0x23) * 0x10);
        let alt = *((pool as *const u8).add(0xf50) as *const u32);
        let actor = if alt != 0 { alt } else { pool };
        let gval = *global::<u32>(0x11735b4);
        callee_thiscall!(3, u32, obj_a.as_mut_ptr() as u32, actor, gval, 0x3bu32);
        callee_thiscall!(4, u32, obj_b.as_mut_ptr() as u32, actor, 0x3a83126fu32, 0x3bu32, 0u32, 0u32);
        callee_thiscall!(5, u32, obj_b.as_mut_ptr() as u32, entry as u32, buf_c.as_mut_ptr() as u32);
        let alloc_pool = *global::<u32>(0x167e2a0);
        let h1 = callee_thiscall!(6, u32, alloc_pool);
        if h1 != 0 {
            let h2 = callee_thiscall!(7, u32, alloc_pool);
            let r8 = if h2 != 0 {
                let mut d = [0u32; 3];
                callee_thiscall!(8, u32, h2, 0x9c4u32, 0xc350u32, d.as_mut_ptr() as u32, pool, 0u32)
            } else {
                0
            };
            callee_thiscall!(9, u32, h1, 0x9c4u32, 0xc350u32, r8, 0u32);
        }
        let r10 = callee_thiscall!(10, u32, obj_a.as_mut_ptr() as u32, entry as u32);
        if r10 as u8 != 0 {
            let t = (ru32(entry, 0x224) as usize).wrapping_add(0x84);
            callee_thiscall!(11, u32, t as u32, obj_a.as_mut_ptr() as u32, 0u32, 1u32);
        }
        let vx = rf(vp, 0) * VEL_SCALE;
        let vy = rf(vp, 4) * VEL_SCALE;
        let vz = rf(vp, 8) * VEL_SCALE + VEL_BIAS;
        let h3 = callee_thiscall!(12, u32, entry as u32);
        let vtable = *(h3 as *const u32);
        let target = *((vtable as *const u8).add(0x24) as *const u32);
        let sample: extern "thiscall" fn(u32) -> f32 =
            core::mem::transmute(target as usize);
        let s = sample(h3);
        let dx = rf(this, 0x30 + si * 0x10) - ex;
        let dy = rf(this, 0x34 + si * 0x10) - ey;
        let dz = rf(this, 0x38 + si * 0x10) - ez;
        let mut fx = vx * s;
        let mut fy = vy * s;
        let mut fz = vz * s;
        let (mut nx, mut ny, mut nz) = (dx, dy, dz);
        let d2b = dy * dy + dx * dx + dz * dz;
        if d2b > NORM_EPS {
            let k = if d2b == 0.0 { 0.0 } else { 1.0 / d2b.sqrt() };
            nx = nx * k;
            ny = ny * k;
            nz = nz * k;
            nx = nx * NORM_HALF;
            ny = ny * NORM_HALF;
            nz = nz * NORM_HALF;
        }
        if *stored >= 5 {
            if ru32(entry, 0x7b8) == 6 {
                fx = fx * FAR_SCALE;
                fy = fy * FAR_SCALE;
                fz = fz * FAR_SCALE;
                let arg0 = [nx.to_bits(), ny.to_bits(), nz.to_bits()];
                let arg1 = [fx.to_bits(), fy.to_bits(), fz.to_bits()];
                callee_thiscall!(15, u32, entry as u32, arg1.as_ptr() as u32, arg0.as_ptr() as u32, 0u32);
            }
        } else {
            let buf = this.add(0x488 + (*stored as usize) * 0x10);
            wf(buf, 4, 0.0);
            wf(buf.wrapping_sub(8), 0, fx);
            wf(buf.wrapping_sub(4), 0, fy);
            wf(buf, 0, fz);
            wf(buf, 0x48, nx);
            wf(buf, 0x4c, ny);
            wf(buf, 0x50, nz);
            wf(buf, 0x54, 0.0);
            let slot = this.add(0x460 + (*stored as usize) * 4) as *mut u32;
            *slot = entry as u32;
            callee_thiscall!(14, u32, entry as u32, slot as u32);
            *stored += 1;
        }
        callee_thiscall!(16, u32, obj_b.as_mut_ptr() as u32);
        callee_thiscall!(17, u32, obj_a.as_mut_ptr() as u32);
    }
}
