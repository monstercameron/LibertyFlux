// original: 0x00661300 gamer_bulk_load
/// Reset the gamer table and bulk-load `count` entries from `src`.
///
/// Clears the count at `+0x2a0`, then copies `count` 16-byte entries into
/// consecutive rows starting at `+0xa0`, bumping the count per entry.
/// Returns 1 (the low byte; the upper bytes repeat the caller's EAX when
/// the table is empty, so only AL is meaningful).
export!(thiscall, rw_00661300(this: u32, _unused: u32, src: u32, count: u32) -> u32 {
    unsafe {
        ((this + 0x2a0) as *mut u32).write(0);
        let mut remaining = count as i32;
        if remaining > 0 {
            let mut from = src;
            let mut e = 0u32;
            while remaining != 0 {
                let n = ((this + 0x2a0) as *const u32).read();
                e = (n + 0xa) * 2;
                core::ptr::copy_nonoverlapping(
                    from as *const u8,
                    (this + e * 8) as *mut u8,
                    16,
                );
                from += 16;
                ((this + 0x2a0) as *mut u32).write(n.wrapping_add(1));
                remaining -= 1;
            }
            (e & 0xffff_ff00) | 1
        } else {
            1
        }
    }
});

