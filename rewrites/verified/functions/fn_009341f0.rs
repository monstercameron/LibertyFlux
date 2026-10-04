// original: 0x009341f0 alloc_init_slots_a
/// 0x009341F0: allocate an array of `n` 0x22c-byte records and initialize
/// each record's tag fields. Returns the block pointer (also for n <= 0,
/// where nothing is initialized).
// shared layout constant
const REC_A_STRIDE: u32 = 0x22c;
export!(stdcall, rw_009341f0(n: i32) -> u32 {
    let bytes = (n as u32).wrapping_mul(REC_A_STRIDE);
    let base = callee_cdecl!(1, u32, bytes);
    if n > 0 {
        let mut i = 0i32;
        while i < n {
            let rec = base.wrapping_add((i as u32).wrapping_mul(REC_A_STRIDE));
            // The original skips a record whose address computes to null.
            if rec != 0 {
                unsafe {
                    let p = rec as *mut u8;
                    core::ptr::write_unaligned(p as *mut u32, 0xffff_ffff);
                    for off in [4u32, 0x44, 0x64, 0xa4, 0xe4, 0x124, 0x164, 0x1a4, 0x1e4] {
                        core::ptr::write(p.add(off as usize), 0u8);
                    }
                    core::ptr::write_unaligned(p.add(0x224) as *mut u32, 0xff00_0000);
                    core::ptr::write_unaligned(p.add(0x228) as *mut u16, 0);
                }
            }
            i += 1;
        }
    }
    base
});
