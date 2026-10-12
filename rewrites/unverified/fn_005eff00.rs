// original: 0x005EFF00 timed_counter_bump (proposed)

/// Bump a counter when its accumulated time has not passed the tick.
///
/// Does nothing while the ready byte at `this + 0x214` is clear. When the
/// reset byte at `this + 0x215` is set, reseeds the accumulator at
/// `this + 0x20c` from the shared tick and clears the reset byte. Then adds
/// the accumulator and the limit at `this + 0x210` (wrapping) and compares
/// the sum against the tick as SIGNED 32-bit values: a sum past the tick
/// ends the call. Otherwise reseeds the accumulator from the tick, reloads
/// the limit with 3000, sets the ready byte and adds the low byte of the
/// stack argument to the counter at `this + 0x208`, returning that byte.
/// The original is thiscall with one stack argument, returning AL; the
/// ready-clear path returns whatever AL held on entry (this proof fixes
/// entry EAX to zero).
lf_checker_rt::export!(thiscall, rw_005eff00(this: u32, amount: u32) -> u32 {
    unsafe {
        const TICK_GLOBAL: u32 = 0x011735B4;
        const COUNT_OFFSET: u32 = 0x208;
        const ACCUM_OFFSET: u32 = 0x20c;
        const LIMIT_OFFSET: u32 = 0x210;
        const READY_OFFSET: u32 = 0x214;
        const RESET_OFFSET: u32 = 0x215;
        const RELOAD: u32 = 0x0bb8;
        let ready =
            (this.wrapping_add(READY_OFFSET) as *const u8).read_unaligned();
        if ready == 0 {
            return 0;
        }
        let tick = (lf_checker_rt::global::<u32>(TICK_GLOBAL)).read_unaligned();
        let reset =
            (this.wrapping_add(RESET_OFFSET) as *const u8).read_unaligned();
        if reset != 0 {
            (this.wrapping_add(ACCUM_OFFSET) as *mut u32).write_unaligned(tick);
            (this.wrapping_add(RESET_OFFSET) as *mut u8).write_unaligned(0);
        }
        let accum = (this.wrapping_add(ACCUM_OFFSET) as *const u32)
            .read_unaligned();
        let limit = (this.wrapping_add(LIMIT_OFFSET) as *const u32)
            .read_unaligned();
        let sum = accum.wrapping_add(limit);
        if (sum as i32) > (tick as i32) {
            return 0;
        }
        let byte = (amount & 0xff) as u8;
        (this.wrapping_add(ACCUM_OFFSET) as *mut u32).write_unaligned(tick);
        (this.wrapping_add(LIMIT_OFFSET) as *mut u32).write_unaligned(RELOAD);
        (this.wrapping_add(READY_OFFSET) as *mut u8).write_unaligned(1);
        let slot = this.wrapping_add(COUNT_OFFSET) as *mut u8;
        slot.write_unaligned(slot.read_unaligned().wrapping_add(byte));
        u32::from(byte)
    }
});
