// original: 0x00945b60 probe_slot_pair_second_kind
/// Poll the current and next slots with the second readiness predicate.
///
/// Same shape as `rw_00945b00` but dispatches to the second predicate, which
/// recognises a different ready state. Return-channel note is the same.
lf_checker_rt::export!(thiscall, rw_00945b60(this_ptr: u32) -> u32 {
    unsafe {
        let idx = *(this_ptr.wrapping_add(0x1917) as *const u8) as u32;
        let slot = this_ptr.wrapping_add(idx.wrapping_mul(0xbd0));
        let mut dl: u8 = 0;
        if *(slot.wrapping_add(0xbca) as *const u8) != 0 {
            dl = lf_checker_rt::callee_thiscall!(1, u32, slot) as u8;
        }
        let next = (idx.wrapping_add(1)) % 2;
        if dl == 0 {
            return 0;
        }
        let prod = next.wrapping_mul(0xbd0);
        let slot2 = this_ptr.wrapping_add(prod);
        if *(slot2.wrapping_add(0xbca) as *const u8) == 0 {
            return (prod & 0xffff_ff00) | (dl as u32);
        }
        lf_checker_rt::callee_thiscall!(1, u32, slot2)
    }
});
