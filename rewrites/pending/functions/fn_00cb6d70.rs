// original: 0x00cb6d70 interaction_guard
/// True when the interaction survives every veto (see above).
export!(thiscall, rw_cb6d70(this_ptr: u32, arg0: u32) -> u8 {
    unsafe {
        let esi: u32 = lf_checker_rt::callee_thiscall!(1, u32, this_ptr);
        let ebx = arg0;
        if ((ebx.wrapping_add(0x29C)) as *const u8).read() & 4 != 0 {
            return 0;
        }
        if esi != 0 && ((esi.wrapping_add(0x29C)) as *const u8).read() & 4 != 0 {
            return 0;
        }
        if ((ebx.wrapping_add(0xB0)) as *const u32).read() != 0
            && ((ebx.wrapping_add(0xB8)) as *const u32).read() as i32 > 0
        {
            return 0;
        }
        if esi != 0 {
            let e = ((esi.wrapping_add(0x224)) as *const u32).read();
            let mut node = ((e.wrapping_add(0x2E0)) as *const u32).read();
            if node != 0 {
                loop {
                    // The (bits>>1)&7 self-compare is dead (same value twice);
                    // only the 0x11F blocking-kind test can fire.
                    if ((node.wrapping_add(4)) as *const u32).read() == 0x11F {
                        return 0;
                    }
                    node = ((node.wrapping_add(0xC)) as *const u32).read();
                    if node == 0 {
                        break;
                    }
                }
            }
        }
        // A null linked ped faults here on both sides (the [0x20] read).
        let pa = ((esi.wrapping_add(0x20)) as *const u32).read();
        let pb = ((ebx.wrapping_add(0x20)) as *const u32).read();
        let fa = ((pa.wrapping_add(0x38)) as *const f32).read();
        let fb = ((pb.wrapping_add(0x38)) as *const f32).read();
        if fa - fb > 2.0 {
            return 0;
        }
        if ((this_ptr.wrapping_add(0xB4)) as *const u8).read() & 1 != 0 {
            let ans: u32 =
                lf_checker_rt::callee_thiscall!(2, u32, esi.wrapping_add(0x80));
            if ans as u8 != 0 {
                return 0;
            }
            let q0 = ((this_ptr.wrapping_add(0x20)) as *const u32).read();
            let q1 = ((q0.wrapping_add(0x50)) as *const u32).read();
            let q2 = ((q1.wrapping_add(0x14)) as *const u32).read();
            return ((q2 >> 4) & 1) as u8;
        }
        let t = ((this_ptr.wrapping_add(0x24)) as *const u32).read();
        if t != 0 {
            let ans: u32 =
                lf_checker_rt::callee_thiscall!(2, u32, t.wrapping_add(0x80));
            if ans as u8 != 0 {
                return 0;
            }
        }
        1
    }
});
