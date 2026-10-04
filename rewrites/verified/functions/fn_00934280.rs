// original: 0x00934280 alloc_init_slots_b
/// 0x00934280: allocate an array of `n` 0x168-byte records and initialize
/// each record's tag fields. Returns the block pointer (also for n <= 0,
/// where nothing is initialized).
// shared layout constant
const REC_B_STRIDE: u32 = 0x168;
export!(stdcall, rw_00934280(n: i32) -> u32 {
    let bytes = (n as u32).wrapping_mul(REC_B_STRIDE);
    let base = callee_cdecl!(1, u32, bytes);
    if n > 0 {
        let mut i = 0i32;
        while i < n {
            let rec = base.wrapping_add((i as u32).wrapping_mul(REC_B_STRIDE));
            // The original skips a record whose address computes to null.
            if rec != 0 {
                unsafe {
                    let p = rec as *mut u8;
                    core::ptr::write(p, 0u8);
                    core::ptr::write(p.add(0x104), 0u8);
                    core::ptr::write(p.add(0x144), 0u8);
                    core::ptr::write_unaligned(p.add(0x154) as *mut u32, 0);
                    core::ptr::write_unaligned(p.add(0x158) as *mut u32, 0);
                    core::ptr::write_unaligned(p.add(0x15c) as *mut u32, 0);
                    core::ptr::write_unaligned(p.add(0x160) as *mut u32, 0);
                    core::ptr::write_unaligned(p.add(0x164) as *mut u16, 0);
                    core::ptr::write(p.add(0x167), 0u8);
                }
            }
            i += 1;
        }
    }
    base
});
