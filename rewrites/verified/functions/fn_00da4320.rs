// original: 0x00da4320 lookup_record_and_ensure_init
// s10f11: look a record up by id and lazily initialise it (thiscall/1).
//
// Scans the stride-16 record array for the id. On a hit returns the record's
// payload; records whose done-flag is clear go through the init step first
// and get the flag set. A miss returns null.
export!(thiscall, rw_s10f11(this: *const u8, id: u32) -> u32 {
    unsafe {
        let begin = *(this.add(0x30) as *const u32) as *const u8;
        let end = *(this.add(0x34) as *const u32) as *const u8;
        let n = (end as u32).wrapping_sub(begin as u32) as i32 >> 4;
        let mut i = 0i32;
        while i < n {
            let rec = begin.add((i as usize) * 16) as *mut u8;
            if *(rec as *const u32) == id {
                let payload = *((rec.add(4)) as *const u32);
                if *rec.add(0xc) == 0 {
                    let init: extern "thiscall" fn(u32, u32) -> u32 =
                        core::mem::transmute(callee_addr(1) as usize);
                    init(this as u32, payload);
                }
                *rec.add(0xc) = 1;
                return payload;
            }
            i += 1;
        }
        0
    }
});
