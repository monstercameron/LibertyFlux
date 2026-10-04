// original: 0x008D8210 table_accumulator

/// Table accumulator: scan, two decoded adds, replayed work list.
pub fn table_accumulate(key: u32, acc1: u32, acc2: u32, tab: u32, n: i32) -> u32 {
    unsafe {
        if n > 0 {
            let t = tab as *const u32;
            let mut i = 0i32;
            loop {
                if t.add(i as usize).read() == key {
                    return i as u32;
                }
                i += 1;
                if i >= n {
                    break;
                }
            }
        }
        let ok = callee_thiscall!(0, u32, relocated(0x0103E8D0), key);
        if (ok as u8) == 0 {
            return ok;
        }
        let mut out = [0u32; 4];
        let cnt = callee_thiscall!(1, u32, relocated(0x01305350), key, out.as_mut_ptr() as u32);
        let base = global::<u32>(0x0103E8D0).read() as *const u8;
        let row = key.wrapping_mul(3) as usize * 8;
        let raw1 = (base.add(row) as *const u32).read();
        let flag1 = (base.add(row + 0xE) as *const u16).read();
        let d1 = if ((flag1 >> 13) & 1) != 0 {
            let sh = (((raw1 >> 11) & 0xF) + 8) as u32;
            (raw1 & 0x7FF).wrapping_shl(sh)
        } else {
            raw1
        };
        let a1 = acc1 as *mut u32;
        a1.write(a1.read().wrapping_add(d1));
        let flag2 = (base.add(row + 0xE) as *const u16).read();
        let d2 = if ((flag2 >> 13) & 1) != 0 {
            let v = (base.add(row) as *const u32).read();
            let sh = (((v >> 26) & 0xF) + 8) as u32;
            ((v >> 15) & 0x7FF).wrapping_shl(sh)
        } else {
            0
        };
        let a2 = acc2 as *mut u32;
        a2.write(a2.read().wrapping_add(d2));
        let mut left = cnt;
        let mut ans = d2;
        while left != 0 {
            left -= 1;
            ans = callee_cdecl!(2, u32, out[left as usize], acc1, acc2, tab, n as u32);
        }
        ans
    }
}
