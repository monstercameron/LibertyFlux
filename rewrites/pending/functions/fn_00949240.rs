// original: 0x00949240 TextSlot_Clear
/// Clear one 124-byte text slot: two zero words, three empty markers, one
/// zero word and three more empty markers. Indexes above 255 do nothing.
/// Returns the slot's byte offset on the taken path, the index otherwise.
export!(cdecl, rw_00949240(slot: u32) -> u32 {
    unsafe {
        if slot > 0xFF {
            return slot;
        }
        let w = global::<u8>(0x011EE608).add((slot * 0x7C) as usize) as *mut u32;
        *w = 0;
        *w.add(1) = 0;
        *w.add(2) = 0xFFFF_FFFF;
        *w.add(3) = 0xFFFF_FFFF;
        *w.add(4) = 0xFFFF_FFFF;
        *w.add(5) = 0;
        *w.add(6) = 0xFFFF_FFFF;
        *w.add(7) = 0xFFFF_FFFF;
        *w.add(8) = 0xFFFF_FFFF;
        slot * 0x7C
    }
});
