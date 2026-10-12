// original: 0x00A33B90 entity_link_unlink

/// Detach this node from its doubly linked neighbours. The previous and next
/// node pointers are stored at byte offsets `0x44` and `0x48`; each neighbour
/// has the reciprocal pointer at the opposite offset. Loads and stores follow
/// the original order so self-links preserve the same aliasing behavior. The
/// game layout uses 32-bit pointers and the function has a thiscall receiver.
lf_checker_rt::export!(thiscall, rw_00a33b90(this: u32) -> () {
    const PREVIOUS_NODE: u32 = 0x44;
    const NEXT_NODE: u32 = 0x48;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    unsafe {
        let next = read_u32(this.wrapping_add(NEXT_NODE));
        let previous = read_u32(this.wrapping_add(PREVIOUS_NODE));
        write_u32(next.wrapping_add(PREVIOUS_NODE), previous);

        let previous_after_first_write = read_u32(this.wrapping_add(PREVIOUS_NODE));
        let next_after_first_write = read_u32(this.wrapping_add(NEXT_NODE));
        write_u32(
            previous_after_first_write.wrapping_add(NEXT_NODE),
            next_after_first_write,
        );
    }
});
