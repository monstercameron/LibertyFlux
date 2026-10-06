// original: 0x0065B380 shader_zero_9_groups (proposed)

/// Zero nine 3-dword groups at `this`, leaving every fourth word alone.
///
/// Writes 0 to the words at `+0x00`/`+0x04`/`+0x08`, `+0x10`/`+0x14`/`+0x18`,
/// and so on through `+0x80`/`+0x84`/`+0x88` (nine groups at stride 0x10).
/// The words at `+0x0c`, `+0x1c`, ... `+0x7c` are never touched.
/// No arguments, no calls, no return value read (thiscall, zero arguments).
lf_checker_rt::export!(thiscall, rw_0065b380(this: u32) -> u32 {
    unsafe {
        let mut g = 0u32;
        while g < 9 {
            let base = this + g * 0x10;
            (base as *mut u32).write_unaligned(0);
            ((base + 4) as *mut u32).write_unaligned(0);
            ((base + 8) as *mut u32).write_unaligned(0);
            g += 1;
        }
        0
    }
});
