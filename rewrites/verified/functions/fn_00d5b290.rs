// original: 0x00D5B290 loader_find_active_request_by_type (proposed)

/// Find the first active request whose zero-extended 16-bit kind equals the
/// full 32-bit argument. The eight entry pointers begin at `this + 0x3114`;
/// search starts just after the head at `+0x3138` and visits the signed
/// number of pending entries in `+0x313c`. A matching record counts only
/// when its state word at offset zero is nonzero. Returns its zero-based
/// search position or `u32::MAX` when none is found.
lf_checker_rt::export!(thiscall, rw_00D5B290(this_ptr: u32, requested_kind: u32) -> u32 {
    unsafe {
        const REQUEST_RING: u32 = 0x3114;
        const RING_HEAD: u32 = 0x3138;
        const RING_LENGTH: u32 = 0x313c;
        const ENTRY_CAPACITY_MASK: u32 = 7;
        const ENTRY_STATE: u32 = 0;
        const ENTRY_KIND: u32 = 8;
        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_u16(address: u32) -> u16 {
            unsafe { (address as *const u16).read_unaligned() }
        }

        let length = read_u32(this_ptr.wrapping_add(RING_LENGTH));
        if (length as i32) <= 0 {
            return u32::MAX;
        }
        let ring = this_ptr.wrapping_add(REQUEST_RING);
        let head = read_u32(this_ptr.wrapping_add(RING_HEAD));
        let mut position = 0u32;
        let mut raw_slot = head.wrapping_add(1);
        while (position as i32) < (length as i32) {
            let slot = raw_slot & ENTRY_CAPACITY_MASK;
            let entry = read_u32(ring.wrapping_add(slot.wrapping_mul(4)));
            let kind = u32::from(read_u16(entry.wrapping_add(ENTRY_KIND)));
            if kind == requested_kind && read_u32(entry.wrapping_add(ENTRY_STATE)) != 0 {
                return position;
            }
            raw_slot = raw_slot.wrapping_add(1);
            position = position.wrapping_add(1);
        }
        u32::MAX
    }
});

