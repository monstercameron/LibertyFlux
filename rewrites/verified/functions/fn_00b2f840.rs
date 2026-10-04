// original: 0x00b2f840 garage_slot_init
// 0xB2F840 garage_slot_init (thiscall/0 -> eax).
//
// Zeroes the slot's timer, counter, tag and link fields, runs the shared
// slot initializer (stubbed by the checker), and returns the slot.
export!(thiscall, rw_00b2f840(rec: *mut u8) -> u32 {
    unsafe {
        *(rec.add(0x38) as *mut u32) = 0;
        *(rec.add(0x3C) as *mut u32) = 0;
        *(rec.add(0x40) as *mut u16) = 0;
        *(rec.add(0x28) as *mut u32) = 0;
        *(rec.add(0x2C) as *mut u32) = 0;
        *rec.add(0x27) = 0;
        let init: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        init(rec as u32);
        rec as u32
    }
});
