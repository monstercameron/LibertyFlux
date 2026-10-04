// original: 0x00949590 obj_clear_slot_word
/// Clear the 16-bit state word of slot `idx` in a 200-byte-stride array.
/// Returns the slot's byte offset.
export!(thiscall, rw_00949590(obj: *mut u8, idx: u32) -> u32 {
    unsafe {
        let off = idx.wrapping_mul(0xC8);
        *(obj.add(off as usize) as *mut u16) = 0;
        off
    }
});
