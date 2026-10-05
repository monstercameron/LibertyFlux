// original: 0x00c49150 ccamcinematic_push_timed (proposed)
/// Append a kind-0x17 entry, or just bump the slot count on cooldown.
///
/// When more than `COOLDOWN` ticks passed since `LAST_TICK_GLOBAL`,
/// resets the shared slot count to `RESET` (-1), appends a `KIND`
/// entry at index `this + COUNT`, bumps the count and records the
/// tick. Otherwise only bumps the shared slot count and records the
/// tick. Either way notifies the owner (callee 1) and returns 1 in
/// the low byte.
///
/// Original: thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00c49150(this: u32) -> u32 {
    const TICK_GLOBAL: u32 = 0x011735c4;
    const LAST_TICK_GLOBAL: u32 = 0x016d8b44;
    const SLOT_COUNT_GLOBAL: u32 = 0x01048f50;
    const COOLDOWN: i32 = 0xbb8;
    const RESET: u32 = 0xffff_ffff;
    const TABLE: u32 = 0x140;
    const COUNT: u32 = 0x200;
    const KIND: u32 = 0x17;
    const NOTIFY: u32 = 1;
    unsafe {
        let tick = lf_checker_rt::global::<u32>(TICK_GLOBAL).read_unaligned();
        let last = lf_checker_rt::global::<u32>(LAST_TICK_GLOBAL).read_unaligned();
        if tick.wrapping_sub(last) as i32 > COOLDOWN {
            lf_checker_rt::global::<u32>(SLOT_COUNT_GLOBAL).write_unaligned(RESET);
            let n = ((this + COUNT) as *const u32).read_unaligned();
            ((this + TABLE + n.wrapping_mul(4)) as *mut u32).write_unaligned(KIND);
            ((this + COUNT) as *mut u32).write_unaligned(n.wrapping_add(1));
            lf_checker_rt::global::<u32>(LAST_TICK_GLOBAL).write_unaligned(tick);
            lf_checker_rt::callee_thiscall!(NOTIFY, u32, this);
        } else {
            let c = lf_checker_rt::global::<u32>(SLOT_COUNT_GLOBAL).read_unaligned();
            lf_checker_rt::global::<u32>(SLOT_COUNT_GLOBAL).write_unaligned(c.wrapping_add(1));
            lf_checker_rt::global::<u32>(LAST_TICK_GLOBAL).write_unaligned(tick);
            lf_checker_rt::callee_thiscall!(NOTIFY + 1, u32, this);
        }
    }
    1
});
