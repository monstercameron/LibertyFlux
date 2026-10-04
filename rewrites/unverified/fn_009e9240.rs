// original: 0x009e9240 ped_reset_weights
/// Resets the weight block: five 1.0 words, the shared float copied
/// from its global into `+0x14`, the flag words masked down and the
/// tail zeroed. The stack argument is ignored. (thiscall, 1 arg.)
lf_checker_rt::export!(thiscall, rw_009e9240(this_ptr: u32, _unused: u32) -> u32 {
    unsafe {
        const ONE_BITS: u32 = 0x3F800000;
        const SHARED_FLOAT: u32 = 0x1050E60;
        const FLAG20_MASK: u32 = 0x2000000;
        const FLAG24_MASK: u32 = 0xFF000000;
        for i in 0..5u32 {
            (this_ptr.wrapping_add(i * 4) as *mut u32).write_unaligned(ONE_BITS);
        }
        let shared = lf_checker_rt::global::<u32>(SHARED_FLOAT).read_unaligned();
        let f20 = (this_ptr.wrapping_add(0x20) as *const u32).read_unaligned();
        (this_ptr.wrapping_add(0x20) as *mut u32).write_unaligned(f20 & FLAG20_MASK);
        let f24 = (this_ptr.wrapping_add(0x24) as *const u32).read_unaligned();
        (this_ptr.wrapping_add(0x24) as *mut u32).write_unaligned(f24 & FLAG24_MASK);
        (this_ptr.wrapping_add(0x14) as *mut u32).write_unaligned(shared);
        (this_ptr.wrapping_add(0x18) as *mut u32).write_unaligned(0);
        (this_ptr.wrapping_add(0x1C) as *mut u16).write_unaligned(0);
        0
    }
});
