// original: 0x00b64d60 first_set_record_above
// thiscall/1. Scans stride-12 records upward from start+1; returns the first
// index whose record is present and either locally flagged or tagged 1,
// or 0 when none qualifies.
export!(thiscall, rw_rs11f11(this: *mut u8, start: u32) -> u32 {
    unsafe {
        let mut i = start.wrapping_add(1);
        if (i as i32) >= 0xb {
            return 0;
        }
        loop {
            let slot = (this as u32).wrapping_add(i.wrapping_add(3).wrapping_mul(12));
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
            i = i.wrapping_add(1);
            if (i as i32) >= 0xb {
                return 0;
            }
        }
    }
});
