// original: 0x00c6e1e0 anim_assoc_create
/// Create an animation association record: pick the slot (indexed into the
/// global table for ids below 0x8f, allocated through the table helper
/// otherwise), copy the two name strings, store the parameter fields,
/// optionally hash the second name, allocate the count*8 data block with
/// overflow saturation, and notify through the finish helper, whose answer
/// is returned.
export!(stdcall, rw_00c6e1e0(idx: u32, src1: u32, src2: u32, count: u32, f1: u32, f2: u32, f3: u32, f4: u32, f5: u32, flag: u32) -> u32 {
    unsafe {
        let rec: u32;
        let fin: u32;
        if (idx as i32) < 0x8f {
            let base = *global::<u32>(0x016D_D640);
            rec = base.wrapping_add(idx.wrapping_mul(0x58));
            fin = idx;
        } else {
            let idx2 = *global::<u16>(0x016D_D644) as u32;
            rec = callee_thiscall!(1, u32, relocated(0x016D_D640), 0x10);
            fin = idx2;
        }
        let mut s = src1;
        let mut d = rec;
        loop {
            let b = *(s as *const u8);
            *(d as *mut u8) = b;
            s = s.wrapping_add(1);
            d = d.wrapping_add(1);
            if b == 0 {
                break;
            }
        }
        s = src2;
        d = rec.wrapping_add(0x20);
        loop {
            let b = *(s as *const u8);
            *(d as *mut u8) = b;
            s = s.wrapping_add(1);
            d = d.wrapping_add(1);
            if b == 0 {
                break;
            }
        }
        *((rec + 0x3c) as *mut u32) = f1;
        *((rec + 0x44) as *mut u32) = f2;
        *((rec + 0x4c) as *mut u32) = f3;
        *((rec + 0x50) as *mut u32) = f4;
        *((rec + 0x38) as *mut u32) = count;
        *((rec + 0x48) as *mut u32) = 0xFFFF_FFFF;
        *((rec + 0x54) as *mut u32) = f5;
        if (flag & 0xFF) != 0 {
            let h = callee_cdecl!(2, u32, rec.wrapping_add(0x20));
            *((rec + 0x48) as *mut u32) = ((h as u16 as i16) as i32) as u32;
        }
        let prod = (count as u64).wrapping_mul(8);
        let alloc = if prod > 0xFFFF_FFFF { 0xFFFF_FFFF } else { prod as u32 };
        let p = callee_cdecl!(3, u32, alloc);
        *((rec + 0x40) as *mut u32) = p;
        callee_cdecl!(4, u32, fin, alloc)
    }
});
