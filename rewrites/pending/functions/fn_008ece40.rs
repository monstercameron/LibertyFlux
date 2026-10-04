// original: 0x008ece40 id_set_contains_32
/// Test whether `id` is one of 32 tracked ids.
///
/// Linear scan over the table at `this`+0x1AC4. Returns the low byte
/// set to 1 or 0 with the incoming `id`'s high bytes preserved, exactly
/// as the original leaves EAX.
export!(thiscall, rw_008ece40(this: *const u8, id: u32) -> u32 {
    unsafe {
        let mut i = 0u32;
        while i < 32 {
            if *(this.add((0x1AC4 + i * 4) as usize) as *const u32) == id {
                return (id & !0xFF) | 1;
            }
            i += 1;
        }
        id & !0xFF
    }
});
