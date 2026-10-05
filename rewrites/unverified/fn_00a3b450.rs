// original: 0x00a3b450 vehicle_tune_attach (proposed)

/// Attach a tune record to the object and derive its rate field.
///
/// Returns the argument unchanged unless the record at `arg + 0x34` and
/// its head word are both non-null and the record's byte at `+0xf20` has
/// bit 1. Otherwise stores `arg` at `obj + 0x80` and the reciprocal of the
/// tune float (row `TABLE[(i16)arg[0x2e]]`, field `+0x1c`) at `obj + 0x8c`,
/// and returns the row pointer. Thiscall/1, returns EAX.
lf_checker_rt::export!(thiscall, rw_00a3b450(obj: u32, arg: u32) -> u32 {
    unsafe {
        const LINK: u32 = 0x34;
        const FLAG_BYTE: u32 = 0xF20;
        const ROW_INDEX: u32 = 0x2E;
        const RATE: u32 = 0x1C;
        const TABLE: u32 = 0x0129_5CD8;
        const ONE: f32 = f32::from_bits(0x3F80_0000);
        let tune = core::ptr::read_unaligned((arg + LINK) as *const u32);
        if tune == 0 {
            return arg;
        }
        if core::ptr::read_unaligned(tune as *const u32) == 0 {
            return arg;
        }
        if core::ptr::read((arg + FLAG_BYTE) as *const u8) & 2 == 0 {
            return arg;
        }
        core::ptr::write_unaligned((obj + 0x80) as *mut u32, arg);
        let idx = core::ptr::read_unaligned((arg + ROW_INDEX) as *const u16) as i16 as i32;
        let row = core::ptr::read_unaligned(
            lf_checker_rt::global::<u32>(TABLE.wrapping_add((idx as u32).wrapping_mul(4))));
        let rate = f32::from_bits(core::ptr::read_unaligned((row + RATE) as *const u32));
        let inv = core::hint::black_box(ONE) / core::hint::black_box(rate);
        core::ptr::write_unaligned((obj + 0x8C) as *mut u32, inv.to_bits());
        row
    }
});
