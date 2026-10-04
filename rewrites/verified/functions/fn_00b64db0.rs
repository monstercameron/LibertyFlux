// original: 0x00b64db0 first_set_record_below
// thiscall/1. Scans downward from start-1 (wrapping a zero start to the top);
// returns the first qualifying index as in F11, or 0 when none qualifies.
export!(thiscall, rw_rs11f12(this: *mut u8, start: u32) -> u32 {
    unsafe {
        let mut i = start.wrapping_sub(1);
        if (i as i32) < 0 {
            i = 0xa;
        } else if i == 0 {
            return 0;
        }
        loop {
            let slot = (this as u32).wrapping_add(i.wrapping_mul(12)).wrapping_add(0x24);
            let value = *(slot as *const i32);
            if value > 0 {
                let info = callee_cdecl!(1, u32, value as u32);
                if *(slot.wrapping_add(4) as *const u16) != 0 {
                    return i;
                }
                if *((info + 0xc) as *const u32) == 1 {
                    return i;
                }
            }
            i = i.wrapping_sub(1);
            if (i as i32) < 0 {
                i = 0xa;
            } else if i == 0 {
                return 0;
            }
        }
    }
});
