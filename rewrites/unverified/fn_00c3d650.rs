// original: 0x00c3d650 train_activate_flagged_cars (proposed)
/// Activate the flagged cars of a global consist table.
///
/// Reads the table pointer from a global: count at `+0x08`, flag bytes
/// at the base in `+0x04`, stride at `+0x0c`, car slots at the base in
/// `+0x00`. For i from count-1 down to 0 (nothing when count is 0):
/// skips slot i when its flag byte has bit 7 set; computes the car
/// `stride*i + slots`; skips it when null or when its kind word at
/// `+0x1300` is not 3; otherwise sets bit 7 and clears bit 5 of its byte
/// at `+0x14e4`, clears the dword at `+0x14c4`, and writes 0x00 to its
/// byte at `+0xe6f` when its `+0x14e5` byte has bit 5 set, else 0x0a.
/// Returns nothing meaningful.
///
/// Original: 0x00c3d650 (cdecl, no stack words).
lf_checker_rt::export!(cdecl, rw_00c3d650() -> u32 {
    unsafe {
        const TABLE: u32 = 0x12e22a4;
        const COUNT: u32 = 8;
        const FLAGS: u32 = 4;
        const STRIDE: u32 = 0xc;
        const SLOTS: u32 = 0;
        const KIND: u32 = 0x1300;
        const KIND_WANT: u32 = 3;
        const MODE: u32 = 0x14e4;
        const SEL: u32 = 0x14e5;
        const CLEAR_W: u32 = 0x14c4;
        const LEVEL: u32 = 0xe6f;
        let t = lf_checker_rt::global::<u32>(TABLE).read_unaligned();
        let n = ((t + COUNT) as *const u32).read_unaligned();
        let flags = ((t + FLAGS) as *const u32).read_unaligned();
        let stride = ((t + STRIDE) as *const u32).read_unaligned();
        let slots = ((t + SLOTS) as *const u32).read_unaligned();
        let mut i = n;
        while i > 0 {
            i -= 1;
            if (((flags + i) as *const u8).read() & 0x80) != 0 {
                continue;
            }
            let car = slots.wrapping_add(stride.wrapping_mul(i));
            if car == 0 {
                continue;
            }
            if (((car + KIND) as *const u32).read_unaligned() != KIND_WANT) {
                continue;
            }
            let m = ((car + MODE) as *const u8).read();
            ((car + MODE) as *mut u8).write((m & 0xdf) | 0x80);
            ((car + CLEAR_W) as *mut u32).write_unaligned(0);
            let s = ((car + SEL) as *const u8).read();
            // neg/sbb/and/add fold to: bit set -> 0x00, clear -> 0x0a.
            ((car + LEVEL) as *mut u8).write(if (s & 0x20) != 0 { 0x00 } else { 0x0a });
        }
        0
    }
});
