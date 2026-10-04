// original: 0x00952de0 alloc_slot
/// Allocate a slot from the 0x5dc-entry table, scanning forward from the
/// stored cursor and wrapping round to the start. The taken slot is marked
/// used, the cursor moves past it, and the second counter is bumped; the
/// result packs the slot index with the counter's low word. Returns all-ones
/// when the table is full.
export!(cdecl, rw_00952de0() -> u32 {
    unsafe {
        let cursor = *global::<u16>(0x011f7104) as u32;
        let mut found: u32 = 0xffff_ffff;
        let mut done = false;
        if cursor < 0x5dc {
            let mut c = cursor;
            while c < 0x5dc {
                if *global::<u8>(0x011f6958 + c) == 0 {
                    found = c;
                    done = true;
                    break;
                }
                c += 1;
            }
        }
        if !done {
            let mut c: u32 = 0;
            while c < cursor {
                if *global::<u8>(0x011f6958 + c) == 0 {
                    found = c;
                    done = true;
                    break;
                }
                c += 1;
            }
        }
        if !done {
            return 0xffff_ffff;
        }
        *global::<u8>(0x011f6958 + found) = 1;
        *global::<u16>(0x011f7104) = (found + 1) as u16;
        let counter: u32 = callee_cdecl!(1, u32,);
        (found & 0xffff) | ((counter & 0xffff) << 16)
    }
});
