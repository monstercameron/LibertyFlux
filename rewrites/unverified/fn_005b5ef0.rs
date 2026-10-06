// original: 0x005B5EF0 alloc_table (proposed)

/// Allocate a row table and initialise every row.
///
/// Allocates `count * 16` bytes through the thread heap manager's alloc
/// entry (TLS slot 0 -> +8 -> vtable -> slot +8, taking (size, 0x10, 0) and
/// popping all three). A non-positive count (signed) allocates and returns
/// without touching the buffer. Otherwise each of the `count` 16-byte rows
/// is set to (0, 0, -1, -1) — unless the row address itself is zero. That
/// null check only guards the first row: with a null buffer the first
/// iteration stores nothing, but the second computes row address 0x10,
/// finds it non-zero and faults writing there (and likewise for any later
/// row). A failed allocation with count above 1 therefore faults, exactly
/// as here. Returns the buffer, possibly null.
/// Stdcall: count on the stack.
lf_checker_rt::export!(stdcall, rw_005B5EF0(count: u32) -> u32 {
    unsafe {
        const ROW_STRIDE: u32 = 16;
        const ALLOC_FLAG: u32 = 0x10;

        let slot = lf_checker_rt::tls_slot(0);
        let mgr = (slot.wrapping_add(8) as *const u32).read();
        let vtable = (mgr as *const u32).read();
        let entry = (vtable.wrapping_add(8) as *const u32).read();
        let alloc: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(entry as usize);
        let buf = alloc(mgr, count.wrapping_mul(ROW_STRIDE), ALLOC_FLAG, 0);
        if (count as i32) <= 0 {
            return buf;
        }
        let mut row = buf;
        let mut left = count;
        while left != 0 {
            if row != 0 {
                ((row.wrapping_add(0)) as *mut u32).write(0);
                ((row.wrapping_add(4)) as *mut u32).write(0);
                ((row.wrapping_add(8)) as *mut u32).write(0xFFFF_FFFF);
                ((row.wrapping_add(12)) as *mut u32).write(0xFFFF_FFFF);
            }
            row = row.wrapping_add(ROW_STRIDE);
            left -= 1;
        }
        buf
    }
});
