// original: 0x00b52540 EuphoriaBodyMassRescale
//! Gated entry walk with sub-iterator broadcast, triple virtual mass sample
//! and rescale branch (animation-euphoria subsystem).
//!
//! Sibling of the neighbouring bound-volume scan: after the same 32-slot
//! bound-box pass it walks a global entry table through three gates (flag
//! byte, status bit, optional blocker object). For the first slot within
//! range of each entry it clamps two entry coefficients, broadcasts to the
//! entry's sub-iterators, issues an eleven-argument setup call, then samples
//! a virtual mass hook three times. When the energy-to-sample ratio exceeds
//! its threshold the velocity vector is rescaled by the third sample;
//! otherwise it is kept as scaled. One effect record is stored; further
//! entries take the far-effect call instead.
//!
//! Like its sibling, this function reads two words of its own uninitialized
//! stack into the effect record; under the checker's defined zero fill both
//! sides agree on 0.0 (see the report). Behaviour verified bit-exact over
//! 1000 trials against checker v2 (with the greater-than-eight-argument
//! cleanup fix described in the report), including NaN/-inf float edges,
//! all three gate exits, both rescale branches, and every sub-iterator count.

const RADIUS: f32 = 2.2; // bits 0x400CCCCD
const NORM_EPS: f32 = 0.25;
const NORM_HALF: f32 = 0.5;
const FAR_SCALE: f32 = 4.0;
const PMAX: f32 = f32::from_bits(0x7F7FFFFF);
const NMAX: f32 = f32::from_bits(0xFF7FFFFF);
const VEL750: f32 = 750.0;
const RTHRESH: f32 = 625.0;
const FSCALE: f32 = 25.0;
const RADIUS2_B: f32 = 5.0;

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

// Original: 0x00B52540. Sibling of 0x00B51FA0: single-slot clear, gated
// entry walk with a sub-iterator broadcast, an 11-argument setup call, three
// virtual mass samples with a rescale branch, and one stored effect.
export!(thiscall, rw_b52540(this_ptr: *mut u8) -> u32 {
    unsafe { body2(this_ptr, RADIUS2_B) };
    0
});

unsafe fn body2(this_ptr: *mut u8, radius2: f32) {
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
        let pool = ru32(this, 0x430);
        let table = *global::<u32>(0x12e22a4);
        let cslot = this.add(0x434) as *mut u32;
        if *cslot != 0 {
            callee_thiscall!(1, u32, *cslot, cslot as u32);
            *cslot = 0;
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
                if *(entry.add(0x24) as *const u8) & 1 == 0 {
                    if idx == 0 {
                        break;
                    }
                    continue;
                }
                let gate = ru32(entry, 0x6c);
                if gate != 0 && *((gate as *const u8).add(0xe) as *const u8) != 0 {
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
                        if radius2 > d2 {
                            process2(this, entry, pool, si, ex, ey, ez, &mut stored);
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
}

#[inline(never)]
unsafe fn process2(
    this: *mut u8, entry: *mut u8, pool: u32, si: usize,
    ex: f32, ey: f32, ez: f32, stored: &mut u32,
) {
    unsafe {
        let c0 = rf(entry, 0x10d8);
        if !(c0 > 1.0) {
            wf(entry, 0x10d8, 1.0);
        }
        let c1 = rf(entry, 0x10ac);
        if !(c1 > 1.0) {
            wf(entry, 0x10ac, 1.0);
        }
        let sub_n = ru32(entry, 0xf84) as i32;
        if sub_n > 0 {
            let sub_base = ru32(entry, 0xf80);
            let mut j = 0i32;
            while j < sub_n {
                let tgt = (sub_base as usize).wrapping_add((j as usize) * 0x170);
                callee_thiscall!(2, u32, tgt as u32);
                j += 1;
            }
        }
        let vp = this.add((si + 0x23) * 0x10);
        let a = rf(vp, 0);
        let b = rf(vp, 4);
        let c = rf(vp, 8);
        let d2 = b * b + a * a + c * c;
        let k = if d2 == 0.0 { 0.0 } else { 1.0 / d2.sqrt() };
        let kx = a * k;
        let kz = k * b;
        let ky = k * c;
        let alt = *((pool as *const u8).add(0xf50) as *const u32);
        let actor = if alt != 0 { alt } else { pool };
        let slotpos = this.add(0x30 + si * 0x10);
        let mut outbuf = [0u32; 4];
        let kvec = [kx.to_bits(), kz.to_bits(), ky.to_bits()];
        callee_thiscall!(3, u32, (entry as usize).wrapping_add(0x10d0) as u32,
            actor, 8u32, 0x37u32, 0x3dcccccdu32, slotpos as u32,
            outbuf.as_mut_ptr() as u32, kvec.as_ptr() as u32,
            0u32, 0u32, 0xffffffffu32, 0u32);
        let vx = rf(vp, 0) * VEL750;
        let vy = rf(vp, 4) * VEL750;
        let vz = rf(vp, 8) * VEL750;
        let h = callee_thiscall!(4, u32, entry as u32);
        let sample1 = vhook(h);
        let h = callee_thiscall!(4, u32, entry as u32);
        let sample2 = vhook(h);
        let d2v = vy * vy + vx * vx + vz * vz;
        let r = d2v / (sample2 * sample1);
        let (mut fx, mut fy, mut fz);
        if r > RTHRESH {
            let h = callee_thiscall!(4, u32, entry as u32);
            let sample3 = vhook(h);
            let f = (sample3 * FSCALE) / d2v.sqrt();
            fx = vx * f;
            fy = vy * f;
            fz = vz * f;
        } else {
            fx = vx;
            fy = vy;
            fz = vz;
        }
        let dx = rf(this, 0x30 + si * 0x10) - ex;
        let dy = rf(this, 0x34 + si * 0x10) - ey;
        let dz = rf(this, 0x38 + si * 0x10) - ez;
        let (mut nx, mut ny, mut nz) = (dx, dy, dz);
        let d2b = dy * dy + dx * dx + dz * dz;
        if d2b > NORM_EPS {
            let k2 = if d2b == 0.0 { 0.0 } else { 1.0 / d2b.sqrt() };
            nx = nx * k2;
            ny = ny * k2;
            nz = nz * k2;
            nx = nx * NORM_HALF;
            ny = ny * NORM_HALF;
            nz = nz * NORM_HALF;
        }
        if *stored >= 1 {
            fx = fx * FAR_SCALE;
            fy = fy * FAR_SCALE;
            fz = fz * FAR_SCALE;
            let arg0 = [nx.to_bits(), ny.to_bits(), nz.to_bits()];
            let arg1 = [fx.to_bits(), fy.to_bits(), fz.to_bits()];
            callee_thiscall!(7, u32, entry as u32, arg1.as_ptr() as u32, arg0.as_ptr() as u32, 0u32);
        } else {
            let buf = this.add(0x448 + (*stored as usize) * 0x10);
            // As in the sibling: two words the original reads from its own
            // uninitialized stack (verified: no writer on any path). Zero
            // under the checker's defined fill on both sides.
            wf(buf, 4, 0.0);
            wf(buf.wrapping_sub(8), 0, fx);
            wf(buf.wrapping_sub(4), 0, fy);
            wf(buf, 0, fz);
            wf(buf, 8, nx);
            wf(buf, 0xc, ny);
            wf(buf, 0x10, nz);
            wf(buf, 0x14, 0.0);
            let slot = this.add(0x434 + (*stored as usize) * 4) as *mut u32;
            *slot = entry as u32;
            callee_thiscall!(6, u32, entry as u32, slot as u32);
            *stored += 1;
        }
    }
}

#[inline(always)]
unsafe fn vhook(h: u32) -> f32 {
    unsafe {
        let vtable = *(h as *const u32);
        let target = *((vtable as *const u8).add(0x24) as *const u32);
        let sample: extern "thiscall" fn(u32) -> f32 =
            core::mem::transmute(target as usize);
        sample(h)
    }
}
