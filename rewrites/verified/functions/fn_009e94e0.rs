// original: 0x009e94e0 ped_zero_block
/// Zeroes eighteen words at the object base, `+0x0` through
/// `+0x44`. (thiscall, no stack arguments.)
lf_checker_rt::export!(thiscall, rw_009e94e0(this_ptr: u32) -> u32 {
    unsafe {
        for i in 0..18u32 {
            (this_ptr.wrapping_add(i * 4) as *mut u32).write_unaligned(0);
        }
        0
    }
});
