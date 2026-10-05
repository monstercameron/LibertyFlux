// original: 0x009DCCE0 pool_vec_init_0x3c (proposed)

/// Allocate and initialise the element buffer of a fixed-stride pool vector.
///
/// `this` points to a 12-byte header: element count at `+0`, a spare word at
/// `+4` (always cleared), buffer body pointer at `+8`. The buffer holds
/// `count` elements of 60 bytes each, preceded by a 4-byte copy of the
/// count. The allocation size is `count * 60 + 4` with saturation to
/// `u32::MAX` when the multiply overflows (the original tests the overflow
/// flag) or the add carries (it tests the carry flag).
///
/// On allocation failure the header keeps its count, `+4` is still cleared,
/// `+8` is set to null and the function returns 0. Otherwise every element's
/// first word is set to the element vtable pointer (an image address the loader
/// relocates, so the rewrite derives it with `relocated`), `+8` receives the
/// buffer body, and the function returns the address just past the last element
/// (the body itself when the count is zero).
///
/// Original: 0x009DCCE0 (thiscall, no stack arguments; one outgoing call to
/// the allocator).
lf_checker_rt::export!(thiscall, rw_009DCCE0(this: u32) -> u32 {
    unsafe {
        const STRIDE: u32 = 0x3C;
        /// File VA of the element vtable; relocated at runtime like the original's immediate.
        const VTABLE_FILE_VA: u32 = 0x00E9779C;
        const ALLOC: u32 = 1;

        let vtable = lf_checker_rt::relocated(VTABLE_FILE_VA);
        let count = (this as *const u32).read_unaligned();
        ((this as *mut u32).wrapping_add(1)).write_unaligned(0);
        let prod = (count as u64).wrapping_mul(STRIDE as u64);
        let size = if prod > 0xFFFF_FFFFu64 {
            0xFFFF_FFFF
        } else {
            (prod as u32).checked_add(4).unwrap_or(0xFFFF_FFFF)
        };
        let buf: u32 = lf_checker_rt::callee_cdecl!(ALLOC, u32, size);
        if buf == 0 {
            ((this as *mut u32).wrapping_add(2)).write_unaligned(0);
            return 0;
        }
        (buf as *mut u32).write_unaligned(count);
        let body = buf.wrapping_add(4);
        let mut slot = body;
        let mut left = count;
        while left != 0 {
            (slot as *mut u32).write_unaligned(vtable);
            slot = slot.wrapping_add(STRIDE);
            left -= 1;
        }
        ((this as *mut u32).wrapping_add(2)).write_unaligned(body);
        slot
    }
});
