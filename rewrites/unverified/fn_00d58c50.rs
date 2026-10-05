// original: 0x00d58c50 ccam_view_find_init
/// Initialise a view-finder camera's tuning block: enable tracking, load the
/// three float tunables from their globals, clear the found flag and arm it.
///
/// Sets byte `TRACKING` (+0x14d) to 1, copies the 32-bit tunables `SPEED`,
/// `RANGE` and `ANGLE` from their globals to `+0x150`, `+0x154` and `+0x158`
/// (exact bit copies, no arithmetic), clears byte `FOUND` (+0x14c) and sets
/// byte `ARMED` (+0x140) to 1. Returns 1.
///
/// Original: thiscall, no stack arguments, returns 1 in `al`.
lf_checker_rt::export!(thiscall, rw_00d58c50 (this: u32) -> u32 {
    unsafe {
        const TRACKING: u32 = 0x14d;
        const TUNE0: u32 = 0x150;
        const TUNE1: u32 = 0x154;
        const TUNE2: u32 = 0x158;
        const FOUND: u32 = 0x14c;
        const ARMED: u32 = 0x140;
        const G_SPEED: u32 = 0x01055644;
        const G_RANGE: u32 = 0x01723B8C;
        const G_ANGLE: u32 = 0x01055648;
        ((this + TRACKING) as *mut u8).write(1);
        let speed = lf_checker_rt::global::<u32>(G_SPEED).read_unaligned();
        ((this + TUNE0) as *mut u32).write_unaligned(speed);
        let range = lf_checker_rt::global::<u32>(G_RANGE).read_unaligned();
        ((this + TUNE1) as *mut u32).write_unaligned(range);
        let angle = lf_checker_rt::global::<u32>(G_ANGLE).read_unaligned();
        ((this + TUNE2) as *mut u32).write_unaligned(angle);
        ((this + FOUND) as *mut u8).write(0);
        ((this + ARMED) as *mut u8).write(1);
        1
    }
});
