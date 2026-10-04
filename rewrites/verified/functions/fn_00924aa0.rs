// original: 0x00924AA0 input_item_init
/// Initialise an item object: tag it, store two arguments, copy 16 bytes.
///
/// Sets a provisional table pointer, xors the old tag word with a counter
/// derived from global `0x010327A0` (masked to 14 bits) and bumps that
/// counter, stores `a0`/`a1` at +8/+12, installs the final table pointer,
/// then copies four words from `src` to +0x10. Returns `this`.
export!(thiscall, rw_00924AA0(this_ptr: u32, a0: u32, a1: u32, src: u32) -> u32 {
    unsafe {
        let o = this_ptr as *mut u32;
        let counter = global::<u32>(0x10327A0);
        let tag = *o.add(1) ^ *counter;
        *o.add(0) = relocated(0xE7E048);
        *o.add(1) ^= tag & 0x3FFF;
        *counter = (*counter).wrapping_add(1);
        *o.add(2) = a0;
        *o.add(3) = a1;
        *o.add(0) = relocated(0xE86314);
        let s = src as *const u32;
        *o.add(4) = *s;
        *o.add(5) = *s.add(1);
        *o.add(6) = *s.add(2);
        *o.add(7) = *s.add(3);
        this_ptr
    }
});
