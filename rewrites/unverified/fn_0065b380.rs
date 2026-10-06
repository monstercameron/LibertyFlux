// original: 0x0065B380 shader_block_zero (proposed)

/// Zero 27 consecutive dwords (0x8c bytes) at `this`.
///
/// Writes 0 to every word from `+0x00` through `+0x88` in groups of three.
/// No arguments, no calls, no return value read (thiscall, zero arguments).
lf_checker_rt::export!(thiscall, rw_0065b380(this: u32) -> u32 {
    unsafe {
        let mut i = 0u32;
        while i < 27 {
            ((this + i * 4) as *mut u32).write_unaligned(0);
            i += 1;
        }
        0
    }
});
