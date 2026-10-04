// original: 0x00dfc845 isxdigit_l
/// Locale-aware hex-digit test: whether `c` has the hex bit (0x80) set.
///
/// Acquires the locale context for `locale`, then either asks the locale
/// worker (multi-byte locales) or masks the locale's ctype-table entry for
/// `c` directly. Releases the locale lock when the context says it was taken.
export!(cdecl, rw_00dfc845(c: u32, locale: u32) -> u32 {
    unsafe {
        let mut ctx = [0u32; 4];
        callee_thiscall!(1, u32, ctx.as_mut_ptr() as u32, locale);
        let locale_obj = ctx[0];
        let result = if (locale_obj.wrapping_add(0x74) as *const u32).read() as i32 > 1 {
            callee_cdecl!(2, u32, c, 128, ctx.as_ptr() as u32)
        } else {
            let table = (locale_obj.wrapping_add(0x90) as *const u32).read();
            let entry = (table.wrapping_add(c.wrapping_mul(2)) as *const u16).read();
            (entry as u32) & 128
        };
        if ctx[3] & 0xFF != 0 {
            let slot = ctx[2].wrapping_add(0x70) as *mut u32;
            slot.write(slot.read() & !2);
        }
        result
    }
});
