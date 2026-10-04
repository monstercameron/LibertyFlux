// original: 0x00908f50 blip_trackable_check
/// Decide whether a blip id selects a trackable blip.
///
/// Rejects negative ids and null table entries, then runs two engine
/// validation steps (each gated on the low answer byte). When the first
/// validation's low byte is zero, compares the record's type field against 4
/// instead; otherwise, and for the remaining slots, compares engine answers
/// against 4, 8 and 5 in order. Returns 1 on the first match, else 0.
export!(cdecl, rw_00908F50(id: u32) -> u32 {
    unsafe {
        if (id as i32) < 0 {
            return 0;
        }
        let entry = blip(id);
        if entry.is_null() {
            return 0;
        }
        let mgr = *global::<u32>(0x12BD0C4);
        let validate: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let acquire: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(2) as usize);
        let entry4 = *(entry.add(4) as *const u32);
        if validate(mgr, 0x12, entry4) & 0xFF == 0 {
            let entry = blip(id);
            let tgt = if *entry.add(0x08) != 0 {
                entry
            } else {
                blip(*global::<u32>(BLIP_DEFAULT))
            };
            if *(tgt.add(0x48) as *const u32) != 4 {
                if callee_cdecl!(3, u32, id) == 8 {
                    return 1;
                }
                if callee_cdecl!(3, u32, id) == 5 {
                    return 1;
                }
                return 0;
            }
            return 1;
        }
        let entry = blip(id);
        if acquire(mgr, 0x12, *(entry.add(4) as *const u32)) & 0xFF == 0 {
            return 0;
        }
        if callee_cdecl!(3, u32, id) == 4 {
            return 1;
        }
        if callee_cdecl!(3, u32, id) == 8 {
            return 1;
        }
        if callee_cdecl!(3, u32, id) == 5 {
            return 1;
        }
        0
    }
});
