// original: 0x00b64c30 topmost_matching_record
// thiscall/1. Scans the ten stride-12 records from the top, skipping empty
// and excluded ones, and stops at the first record whose info triple matches
// (or at the requested kind mismatch). Returns the record word at the stop.
export!(thiscall, rw_rs11f6(this: *mut u8, kind: u32) -> u32 {
    unsafe {
        let mut left = 10u32;
        let mut slot = (this as u32).wrapping_add(0x9c);
        loop {
            let value = *(slot as *const i32);
            if value > 0 && value != 0x2e {
                let info = callee_cdecl!(1, u32, value as u32);
                if *((info + 8) as *const u32) != 3 {
                    break;
                }
                if *((info + 0xc) as *const u32) == 0 {
                    break;
                }
                if kind != 8 {
                    break;
                }
            }
            left = left.wrapping_sub(1);
            slot = slot.wrapping_sub(0xc);
            if left == 0 {
                break;
            }
        }
        *((this as u32).wrapping_add(left.wrapping_mul(12)).wrapping_add(0x24) as *const u32)
    }
});
