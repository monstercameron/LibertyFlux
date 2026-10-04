// original: 0x00E60A00 timer_slots_init_256 (proposed)

/// Bulk slot-table initialisation: zeroes 256 slot records, one per call.
///
/// Behaviour: for 256 records starting at `TABLE`, writes -1 to the head
/// word at cursor - 0x14 and 0 to the word at cursor - 0x10 (16-bit) and
/// the dwords at cursor - 0xC, -8, -4, 0, +4, +8, +0xC, +0x10, +0x14,
/// +0x20, +0x24, +0x28 and +0x2C (cursor + 0x18/+0x1C are left alone),
/// then calls the thiscall
/// slot initialiser (callee 1) with the record base (cursor - 0x14) in
/// ECX, and advances the cursor by 0x44. After the loop, registers the
/// callback `CALLBACK` with the cdecl registrar and returns its answer.
/// Callee 1 fires exactly 256 times, then the registrar once.
///
/// Original: cdecl, no stack arguments; callee-saved ESI/EDI preserved.
/// The store offsets and the 0x44 stride are the record layout.
lf_checker_rt::export!(cdecl, rw_00e60a00() -> u32 {
    unsafe {
        const RECORDS: u32 = 256;
        const STRIDE: u32 = 0x44;
        let mut cursor: u32 = lf_checker_rt::relocated(0x0019F3A94);
        let mut left = RECORDS;
        while left > 0 {
            let base: u32 = cursor.wrapping_sub(0x14);
            (base as *mut u32).write_unaligned(0xFFFF_FFFF);
            ((base + 0x04) as *mut u16).write_unaligned(0);
            for off in [0x08u32, 0x0Cu32, 0x10, 0x14, 0x18, 0x1C, 0x20, 0x24, 0x28, 0x34, 0x38, 0x3C, 0x40] {
                (base.wrapping_add(off) as *mut u32).write_unaligned(0);
            }
            lf_checker_rt::callee_thiscall!(1, u32, base);
            cursor = cursor.wrapping_add(STRIDE);
            left -= 1;
        }
        lf_checker_rt::callee_cdecl!(2, u32, lf_checker_rt::relocated(0x00E6FB30))
    }
});

