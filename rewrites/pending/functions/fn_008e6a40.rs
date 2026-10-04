// original: 0x008e6a40 insert_entry_sorted
/// Insert one entry into the sorted run ending at `hole`: shift down every
/// entry whose float key is strictly below the new key, then store the new
/// value. Returns the key word.
export!(cdecl, rw_008e6a40(hole: *mut u8, val0: u32, val1: u32) -> u32 {
    unsafe {
        let key = f32::from_bits(val1);
        let mut slot = hole;
        loop {
            let prev = slot.sub(8);
            let p = entry_at(prev, 0);
            if !(key > f32::from_bits(p.1)) {
                break;
            }
            set_entry(slot, 0, p);
            slot = prev;
        }
        set_entry(slot, 0, (val0, val1));
        val1
    }
});

