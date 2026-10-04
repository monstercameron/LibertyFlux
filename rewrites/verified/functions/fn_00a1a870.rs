// original: 0x00a1a870 cam_follow_avail_test (proposed)

/// Tests whether a follow-camera mode is available for an object.
///
/// `obj` points to a record with a flag byte at `+FLAG_A` (bit 3),
/// a flag byte at `+FLAG_B` (bit 0), a mode word at `+MODE_OFF`, a
/// pointer at `+TARGET_OFF` to a record holding a float at `+TARGET_VAL`,
/// and an option word at `+OPT_OFF`. Returns 1 when both flag bits are
/// set, or when the mode is `WANT_MODE`, the pointed-to float is strictly
/// below zero and the option bit is set; otherwise 0. A NaN float takes
/// the false path (the compare-branch falls through). Only `al` carries
/// the result.
///
/// Original: 0x00a1a870 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_00a1a870(obj: u32) -> u32 {
    unsafe {
        const FLAG_A: u32 = 0xf17;
        const FLAG_B: u32 = 0x118;
        const MODE_OFF: u32 = 0x1304;
        const TARGET_OFF: u32 = 0x20;
        const TARGET_VAL: u32 = 0x28;
        const OPT_OFF: u32 = 0x24;
        const WANT_MODE: u32 = 2;
        const OPT_BIT: u32 = 0x2000_0000;
        let a = ((obj + FLAG_A) as *const u8).read();
        let b = ((obj + FLAG_B) as *const u8).read();
        if a & 8 != 0 && b & 1 != 0 {
            return 1;
        }
        if ((obj + MODE_OFF) as *const u32).read_unaligned() != WANT_MODE {
            return 0;
        }
        let target = ((obj + TARGET_OFF) as *const u32).read_unaligned();
        let val = f32::from_bits(((target + TARGET_VAL) as *const u32).read_unaligned());
        if !(0.0f32 > val) {
            return 0;
        }
        u32::from(((obj + OPT_OFF) as *const u32).read_unaligned() & OPT_BIT != 0)
    }
});
