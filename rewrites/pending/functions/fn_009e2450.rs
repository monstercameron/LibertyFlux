// original: 0x009e2450 portal_tracker_refresh_snapshot
/// When fresh input of a handled kind arrives, re-voice it and
/// snapshot its position block, then run the virtual refresh; otherwise just
/// run the virtual refresh and clear the pending flag. Returns the refresh
/// answer. (thiscall/2; the virtual call goes through the object's own table,
/// exactly like the original, landing on the checker's planted stub.)
export!(thiscall, rw_009e2450(this: *mut u8, a: u32, b: u32) -> u32 {
    unsafe {
        if *this.add(0x76) == 0 && a != 0 {
            let ty = (*((a.wrapping_add(0x28)) as *const u32) >> 6) & 0xF;
            if ty == 4 || ty == 3 || ty == 2 {
                let w0 = *((a.wrapping_add(0xB0)) as *const u32);
                let w1 = *((a.wrapping_add(0xB8)) as *const u32);
                callee_thiscall!(1, u32, this as u32, w0, w1);
                let v0 = *((a.wrapping_add(0xA0)) as *const u32);
                let v1 = *((a.wrapping_add(0xA4)) as *const u32);
                let v2 = *((a.wrapping_add(0xA8)) as *const u32);
                let v3 = *((a.wrapping_add(0xAC)) as *const u32);
                *(this.add(0x20) as *mut u32) = v0;
                *(this.add(0x24) as *mut u32) = v1;
                *(this.add(0x28) as *mut u32) = v2;
                *(this.add(0x2c) as *mut u32) = v3;
                *(this.add(0x10) as *mut u32) = v0;
                *(this.add(0x14) as *mut u32) = v1;
                *(this.add(0x18) as *mut u32) = v2;
                *(this.add(0x1c) as *mut u32) = v3;
                let vt = *(this as *const u32);
                let tgt = *((vt.wrapping_add(4)) as *const u32);
                let f: extern "thiscall" fn(u32, u32, u32) -> u32 =
                    core::mem::transmute(tgt as usize);
                return f(this as u32, b, 0);
            }
        }
        let vt = *(this as *const u32);
        let tgt = *((vt.wrapping_add(4)) as *const u32);
        let f: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(tgt as usize);
        let r = f(this as u32, b, 0);
        *this.add(0x76) = 0;
        r
    }
});
