// original: 0x009e89a0 first_live_slot_tail
/// First non-null of the five slot words at `+0x44`, followed down
/// its `+8` chain to the tail; null when every slot is empty.
/// (thiscall.)
lf_checker_rt::export!(thiscall, rw_009e89a0(this_ptr: u32) -> u32 {
    unsafe {
        const SLOTS_OFF: u32 = 0x44;
        const SLOT_COUNT: u32 = 5;
        const NEXT_OFF: u32 = 8;
        let mut found = 0u32;
        for i in 0..SLOT_COUNT {
            let v = (this_ptr.wrapping_add(SLOTS_OFF + i * 4) as *const u32).read_unaligned();
            if v != 0 {
                found = v;
                break;
            }
        }
        if found == 0 {
            return 0;
        }
        loop {
            let next = (found.wrapping_add(NEXT_OFF) as *const u32).read_unaligned();
            if next == 0 {
                return found;
            }
            found = next;
        }
    }
});
