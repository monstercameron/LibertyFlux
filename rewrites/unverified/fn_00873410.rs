// original: 0x00873410 crmt_zeroing_array_alloc

/// Allocate `count*20` bytes through the thread manager and zero two words per 20-byte record (the words at offsets 4 and 12 past each record start). Non-positive counts skip the loop and return the block as is. Returns the block.
///
/// Original: 0x00873410 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_00873410(count: u32) -> u32 {
    const MANAGER_OFF: u32 = 8;
    const FREE_SLOT: u32 = 0x0c;
    const REC_SIZE: u32 = 20;
    const ALLOC_SLOT: u32 = 8;
    unsafe {
        let n = count as i32;
        let size = (count as u32).wrapping_mul(REC_SIZE);
        let tls0 = lf_checker_rt::tls_slot(0);
        let manager = ((tls0 + MANAGER_OFF) as *const u32).read_unaligned();
        let vtable = (manager as *const u32).read_unaligned();
        let target = ((vtable as *const u8).add(ALLOC_SLOT as usize) as *const u32)
            .read_unaligned();
        let alloc: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        let buf = alloc(manager, size, 0x10, 0);
        if n > 0 {
            let mut p = buf.wrapping_add(4);
            let mut i = n;
            loop {
                if p.wrapping_sub(4) != 0 {
                    (p as *mut u32).write_unaligned(0);
                    ((p + 8) as *mut u32).write_unaligned(0);
                }
                p = p.wrapping_add(0x14);
                i -= 1;
                if i == 0 { break; }
            }
        }
        buf
    }
});
