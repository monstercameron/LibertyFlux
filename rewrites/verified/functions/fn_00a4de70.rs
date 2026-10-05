// original: 0x00A4DE70 vehicle_arm_f48 (proposed)

/// Arms the word at `this + ARM` (0x0F48) to `ARMED` (0x3A98) when idle.
///
/// Fails (returns 0, writes nothing) when any guard trips: the linked object
/// at `[this + LINK]` (0x6C) is present and its byte at `+0x0E` is non-zero,
/// the word at `this + MODE` (0x1304) is non-zero, or the arm word is not
/// `IDLE` (0xFFFF). Otherwise stores `ARMED` and returns 1. A null link
/// skips the first guard.
///
/// Original: 0x00A4DE70 (thiscall, no stack words), leaf, no globals.
lf_checker_rt::export!(thiscall, rw_00A4DE70(this: u32) -> u32 {
    unsafe {
        const LINK: u32 = 0x6C;
        const LINK_FLAG: u32 = 0x0E;
        const MODE: u32 = 0x1304;
        const ARM: u32 = 0x0F48;
        const IDLE: u16 = 0xFFFF;
        const ARMED: u16 = 0x3A98;
        let link = ((this + LINK) as *const u32).read_unaligned();
        if link != 0 && ((link + LINK_FLAG) as *const u8).read() != 0 {
            return 0;
        }
        if ((this + MODE) as *const u32).read_unaligned() != 0 {
            return 0;
        }
        let arm = (this + ARM) as *mut u16;
        if arm.read_unaligned() != IDLE {
            return 0;
        }
        arm.write_unaligned(ARMED);
        1
    }
});
