// original: 0x00d88e90 proposed_chain_walk_budget
// Budgeted chain walk: follows a chain of records through the record
// table, measuring polyline length against a fixed budget. `obj` is the
// walker object; its words at +0xde0/+0xde4/+0xde8 hold packed
// (sub-record, table-index) pairs, and the bytes at +0xf19/+0xf1a/+0xf1e
// hold state flags. Each step decodes the next record's position (table
// entry plus sub-record stride, converted from fixed point) and spends
// the segment length from the budget; the walk stops when the budget is
// spent, the chain ends, or a record's kind nibble exceeds 2 (which sets
// the flag bit). Returns a status word determined by the exit path.

use lf_checker_rt::{callee_thiscall, export, global};

export!(cdecl, rw_d88e90(obj: u32) -> u32 {
    unsafe {
        let table = global::<u32>(0x1178284);
        let b = ((obj + 0xf1a) as *const u8).read();
        let cl = ((b & 1) << 1) | (b & 0xfc);
        ((obj + 0xf1a) as *mut u8).write(cl);
        if ((obj + 0xf1e) as *const u8).read() & 0x20 == 0 {
            return (b & 0xfc) as u32;
        }
        let a19 = ((obj + 0xf19) as *const u8).read();
        if a19 & 0x10 == 0 {
            return a19 as u32;
        }
        if a19 & 0x20 != 0 {
            return a19 as u32;
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
        let mut c_slot = e2.wrapping_add((p0 >> 16) << 5);
        let mut f0 = [0u32; 2];
        let mut f1b = [0u32; 2];
        let mut f2b = [0u32; 2];
        callee_thiscall!(1, u32, b_slot, f0.as_mut_ptr() as u32);
        callee_thiscall!(2, u32, a_slot, f1b.as_mut_ptr() as u32);
        let mut eax_out = callee_thiscall!(3, u32, c_slot, f2b.as_mut_ptr() as u32);
        let (mut px, mut py) = (f32::from_bits(f1b[0]), f32::from_bits(f1b[1]));
        let (mut qx, mut qy) = (f32::from_bits(f0[0]), f32::from_bits(f0[1]));
        let (mut rx, mut ry) = (f32::from_bits(f2b[0]), f32::from_bits(f2b[1]));
        let mut rem = 40.0f32;
        let mut ptr = obj.wrapping_add(0xde8);
        let mut count = 3u32;
        loop {
            let dx = px - qx;
            let dy = py - qy;
            rem = rem - (dy * dy + dx * dx).sqrt();
            if 0.0 > rem {
                return eax_out;
            }
            let av = ((a_slot + 0x1e) as *const u8).read() & 0xf;
            if av > 2 {
                ((obj + 0xf1a) as *mut u8).write(cl | 1);
                return (a_slot & 0xFFFFFF00) | (cl | 1) as u32;
            }
            ptr = ptr.wrapping_add(4);
            let v = (ptr as *const u32).read();
            a_slot = c_slot;
            qx = px;
            qy = py;
            px = rx;
            py = ry;
            count += 1;
            let vv = v & 0xffff;
            if vv == 0xffff {
                return 0xffff;
            }
            let e = table.add(vv as usize).read();
            if e == 0 {
                return 0;
            }
            let cn = e.wrapping_add((v >> 16) << 5);
            c_slot = cn;
            rx = ((cn + 0x14) as *const i16).read() as f32 * 0.125;
            let w16 = ((cn + 0x16) as *const i16).read();
            ry = w16 as f32 * 0.125;
            eax_out = w16 as i32 as u32;
            if count >= 11 {
                return eax_out;
            }
        }
    }
});
