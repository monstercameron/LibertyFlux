// original: 0x00ab6ea0 guarded_pair_authorize
/// Resolves (arg0,arg1) through the pair table, checks the entry flags
/// against the global mask/deny words, then scans the global authorization
/// rows for a matching record. Returns 1 when authorized, 0 when denied.

export!(thiscall, rw_00ab6ea0(this: *const u8, arg0: u32, arg1: u32) -> u32 {
    unsafe {
        let entry: u32 = callee_thiscall!(1, u32, this as u32, arg0, arg1);
        let flags = *((entry + 8) as *const u32);
        let mask = *(global::<u32>(0x150E0EC));
        if flags & mask != mask {
            return 0;
        }
        if *(global::<u32>(0x150E0F0)) & flags != 0 {
            return 0;
        }
        if *(this.add(4) as *const u16) == 0 {
            return 1;
        }
        let count = *(global::<i32>(0x150E128));
        if count <= 0 {
            return 1;
        }
        let base = *(this as *const u32);
        let table = global::<u32>(0x150E100);
        let mut i = 0i32;
        while i < count {
            let t = *table.add(i as usize);
            if t != 0xFFFFFFFF {
                let rec = base.wrapping_add(t.wrapping_shl(5));
                if arg0 == *((rec + 0xC) as *const u32) {
                    let v10 = *((rec + 0x10) as *const u32);
                    if arg1 == v10 {
                        return 0;
                    }
                    if v10 == 0x3E7 {
                        return 0;
                    }
                }
                if arg0 == *((rec + 0x14) as *const u32) {
                    let v18 = *((rec + 0x18) as *const u32);
                    if arg1 == v18 {
                        return 0;
                    }
                    if v18 == 0x3E7 {
                        return 0;
                    }
                }
            }
            i += 1;
        }
        1
    }
});
