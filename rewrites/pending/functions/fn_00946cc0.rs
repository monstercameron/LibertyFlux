// original: 0x00946cc0 ensure_slot_capacity
/// Top up slot capacity toward the requested need.
///
/// Compares the request (unsigned) against the current slot's free space.
/// When the request exceeds it and the next slot is live, a bounded top-up
/// amount is computed (saturating lower bound, clamped upper bound),
/// converted by a helper, and applied to the next slot. The request itself
/// is then always applied to the current slot. The lower amount subtracts a
/// floor value only when affordable (small values pass through unchanged);
/// the upper amount is clamped to a ceiling.
lf_checker_rt::export!(thiscall, rw_00946cc0(this_ptr: u32, need: u32) -> () {
    unsafe {
        let idx = *(this_ptr.wrapping_add(0x1917) as *const u8) as u32;
        let slot = this_ptr.wrapping_add(idx.wrapping_mul(0xbd0));
        let mut avail = *(slot.wrapping_add(0x994) as *const u32);
        if *(slot.wrapping_add(0xbcc) as *const u8) != 0 {
            avail = avail.wrapping_sub(*(slot.wrapping_add(0xbbc) as *const u32));
        }
        avail = avail.wrapping_sub(*(slot.wrapping_add(0x990) as *const u32));
        if need > avail {
            let next = (idx.wrapping_add(1)) % 2;
            let s2 = this_ptr.wrapping_add(next.wrapping_mul(0xbd0));
            if *(s2.wrapping_add(0xbca) as *const u8) != 0 {
                let mut ecx = *(s2.wrapping_add(0x994) as *const u32);
                if *(s2.wrapping_add(0xbcc) as *const u8) != 0 {
                    ecx = ecx.wrapping_sub(*(s2.wrapping_add(0xbbc) as *const u32));
                }
                let a = if ecx >= 0x4e20 {
                    ecx.wrapping_sub(0x4e20)
                } else {
                    ecx
                };
                let b = ecx.min(0x2710);
                let r = lf_checker_rt::callee_cdecl!(1, u32, b, a);
                lf_checker_rt::callee_thiscall!(2, u32, s2, r);
            }
        }
        let idx2 = *(this_ptr.wrapping_add(0x1917) as *const u8) as u32;
        let slot2 = this_ptr.wrapping_add(idx2.wrapping_mul(0xbd0));
        lf_checker_rt::callee_thiscall!(2, u32, slot2, need);
    }
});
