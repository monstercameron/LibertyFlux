// original: 0x0089e6e0 scan_audio_slots_accumulate
/// Scan the slot row, resolving and accumulating each live slot.
///
/// Returns -1 at once when the object is flagged off or its count is not
/// positive. Otherwise walks the slot bytes: skips 0xFF slots, resolves
/// each other slot through the table word picked by the object's key byte
/// combined with the global multiplier, skips zero resolutions, and asks
/// the tester about the rest with a frame flag byte. Adds every tester
/// answer that is not -1 into the total; a -1 answer abandons the scan
/// with -1.
export!(thiscall, rw_0089e6e0(this_: *const u8, _arg: u32) -> u32 {
    unsafe {
        if *this_.add(0xd2) != 0 {
            return 0xffffffff;
        }
        let count = *(this_.add(0xc8) as *const i32);
        if count <= 0 {
            return 0xffffffff;
        }
        let mult = *(global::<u32>(0x0115d964) as *const u32);
        let table_base = *(global::<u32>(0x0115d988) as *const u32);
        let key = *this_.add(0x40) as u32;
        let table_word = *((table_base + key.wrapping_mul(0x6f40) + 0x6f10) as *const u32);
        let mut total: u32 = 0xffffffff;
        let mut slot = this_.add(0x48);
        let base = 0xffffffb8u32.wrapping_sub(this_ as u32);
        // The bound below mirrors the original's running comparison.
        for _ in 0..1000000u32 {
            let bound = base.wrapping_add(slot as u32);
            if (bound as i32) >= count {
                break;
            }
            let byte = *slot as u32;
            if byte != 0xff {
                let key_val = mult.wrapping_mul(byte).wrapping_add(table_word);
                if key_val != 0 {
                    let mut flag: u32 = 0;
                    let answer = callee_thiscall!(1, u32, key_val, &mut flag as *mut u32 as u32);
                    if answer == 0xffffffff {
                        return 0xffffffff;
                    }
                    total = total.wrapping_add(answer);
                }
            }
            slot = slot.add(1);
        }
        total
    }
});
