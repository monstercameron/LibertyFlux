// original: 0x00d890d0 proposed_gated_chain_walk
// Gated chain walk: the budgeted chain walk of the sibling function,
// with an entry gate and a resolved-record tail. `obj` carries a gate
// word at +0x2c (summed with a global, bit 8 must be set), a mode byte
// at +0xe6e (anything but 1 takes the resolver path through a helper
// call) and a sub-mode at +0xe70 (only 0/1 continue). The walk itself
// matches the sibling's, except each step's position comes from the
// decode helper and a record whose kind nibble exceeds 2 resolves
// through two normalize calls: a strong alignment exits quietly, otherwise
// the turn direction sets one of two flag bits.

use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global};

export!(cdecl, rw_d890d0(obj: u32) -> u32 {
    unsafe {
        let table = global::<u32>(0x1178284);
        callee_cdecl!(1, u32, obj);
        let g = global::<u32>(0x11735b4).read();
        let w = ((obj + 0x2c) as *const u16).read() as u32;
        let sum = w.wrapping_add(g);
        if sum & 0x200 == 0 {
            return sum;
        }
        if ((obj + 0xe6e) as *const u8).read() != 1 {
            let ans = callee_cdecl!(8, u32, obj);
            if ans == 0 {
                return 0;
            }
            if ((ans + 0x26) as *const u8).read() != 0x15 {
                return ans;
            }
            let ab = ((ans + 0x2b) as *const u8).read();
            if ab & 8 == 0 {
                let p = (obj + 0xf1c) as *mut u8;
                p.write(p.read() | 1);
                return (ans & 0xFFFFFF00) | ab as u32;
            }
            if ab & 0x10 == 0 {
                let p = (obj + 0xf1b) as *mut u8;
                p.write(p.read() | 0x80);
                return (ans & 0xFFFFFF00) | ab as u32;
            }
            let p = (obj + 0xf1c) as *mut u8;
            p.write(p.read() | 1);
            return (ans & 0xFFFFFF00) | ab as u32;
        }
        let e70 = ((obj + 0xe70) as *const u8).read();
        if e70 != 0 && e70 != 1 {
            return (sum & 0xFFFFFF00) | e70 as u32;
        }
        let d0 = ((obj + 0xde0) as *const u32).read();
        let d1 = ((obj + 0xde4) as *const u32).read();
        let p0 = ((obj + 0xde8) as *const u32).read();
        if d0 & 0xffff == 0xffff {
            return d1;
        }
        if d1 & 0xffff == 0xffff {
            return d1;
        }
        if p0 & 0xffff == 0xffff {
            return d1;
        }
        let e0 = table.add((d0 & 0xffff) as usize).read();
        if e0 == 0 {
            return d1;
        }
        let e1 = table.add((d1 & 0xffff) as usize).read();
        if e1 == 0 {
            return d1;
        }
        let e2 = table.add((p0 & 0xffff) as usize).read();
        if e2 == 0 {
            return d1;
        }
        let mut a_slot = e1.wrapping_add((d1 >> 16) << 5);
        let b_slot = e0.wrapping_add((d0 >> 16) << 5);
        let mut c_cur = e2.wrapping_add((p0 >> 16) << 5);
        let mut fb = [0u32; 2];
        let mut fa = [0u32; 2];
        let mut fc = [0u32; 2];
        callee_thiscall!(2, u32, b_slot, fb.as_mut_ptr() as u32);
        callee_thiscall!(3, u32, a_slot, fa.as_mut_ptr() as u32);
        callee_thiscall!(4, u32, c_cur, fc.as_mut_ptr() as u32);
        let (mut px, mut py) = (f32::from_bits(fa[0]), f32::from_bits(fa[1]));
        let (mut qx, mut qy) = (f32::from_bits(fb[0]), f32::from_bits(fb[1]));
        let mut rem = 40.0f32;
        let mut ptr = obj.wrapping_add(0xde8);
        let mut count = 3u32;
        let (mut last_dx, mut last_dy) = (0.0f32, 0.0f32);
        loop {
            let dx = px - qx;
            let dy = py - qy;
            last_dx = dx;
            last_dy = dy;
            rem = rem - (dy * dy + dx * dx).sqrt();
            if 0.0 > rem {
                return 0;
            }
            let av = ((a_slot + 0x1e) as *const u8).read() & 0xf;
            if av > 2 {
                break;
            }
            ptr = ptr.wrapping_add(4);
            let v = (ptr as *const u32).read();
            qx = px;
            qy = py;
            px = f32::from_bits(fc[0]);
            py = f32::from_bits(fc[1]);
            a_slot = c_cur;
            count += 1;
            let vv = v & 0xffff;
            if vv == 0xffff {
                return 0xffff;
            }
            let e = table.add(vv as usize).read();
            if e == 0 {
                return 0;
            }
            c_cur = e.wrapping_add((v >> 16) << 5);
            callee_thiscall!(5, u32, c_cur, fc.as_mut_ptr() as u32);
            if count >= 11 {
                return 0;
            }
        }
        // resolve tail: dx/dy of the last step and C-minus-P, normalized.
        let dxp = f32::from_bits(fc[0]) - px;
        let dyp = f32::from_bits(fc[1]) - py;
        let mut buf1 = [last_dx, last_dy, 0.0f32];
        let mut buf2 = [dxp, dyp, 0.0f32];
        callee_thiscall!(6, u32, buf1.as_mut_ptr() as u32);
        callee_thiscall!(7, u32, buf2.as_mut_ptr() as u32);
        let (nx, ny) = (buf1[0], buf1[1]);
        let (nxp, nyp) = (buf2[0], buf2[1]);
        let dot = (nyp * ny + nxp * nx) + buf2[2] * buf1[2];
        if dot < 0.25f32 {
            let cross = nyp * nx - nxp * ny;
            if 0.0 > cross {
                let p = (obj + 0xf1c) as *mut u8;
                p.write(p.read() | 1);
            } else {
                let p = (obj + 0xf1b) as *mut u8;
                p.write(p.read() | 0x80);
            }
        }
        0
    }
});
