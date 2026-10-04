// original: 0x00b0abb0 ring_push_handle
/// Push a tagged handle and its value onto the module ring.
///
/// Tags the slot with the table word for its lane, stores the handle and
/// the value at the next ring position, and advances the counter with
/// wrap at twenty. Returns the new counter.
export!(cdecl, rw_00b0abb0(slot: u32, value: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 0x0161_5574;
        const HANDLES: u32 = 0x0161_5574;
        const VALUES: u32 = 0x0161_55C4;
        const TAGS: u32 = 0x0161_56A2;
        const LIMIT: u32 = 0x14;
        const LANES: usize = 5;
        const LANE_BYTES: usize = 16;
        let next = (*global::<u32>(COUNT)).wrapping_add(1);
        let at = (relocated(TAGS) as usize)
            .wrapping_add((slot as usize).wrapping_mul(LANES).wrapping_mul(LANE_BYTES));
        let tag = *(at as *const u16);
        let handle = ((tag as u32) << 16) | slot;
        *((relocated(HANDLES) as usize).wrapping_add((next as usize).wrapping_mul(4))
            as *mut u32) = handle;
        *((relocated(VALUES) as usize).wrapping_add((next as usize).wrapping_mul(4))
            as *mut u32) = value;
        let wrapped = if next < LIMIT { next } else { 0 };
        *global::<u32>(COUNT) = wrapped;
        wrapped
    }
});
