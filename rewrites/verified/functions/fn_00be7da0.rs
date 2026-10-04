// original: 0x00BE7DA0 timer_slot_init (proposed)
/// Initialise a timer slot with an interval and the current tick.
///
/// `slot` points to the slot object, `interval` is stored at `+0x2c` and
/// `+0x24`, the global tick counter is copied to `+0x20`, and the ready
/// flag byte at `+0x28` is set to 1. Returns the tick that was stored.
///
/// Original: 0x00BE7DA0 (thiscall, one stack word). Straight-line code;
/// the return value is the global, read fresh on every call.
lf_checker_rt::export!(thiscall, rw_00BE7DA0(slot: u32, interval: u32) -> u32 {
    unsafe {
        const STAMP: u32 = 0x20;
        const INTERVAL_COPY: u32 = 0x24;
        const READY: u32 = 0x28;
        const INTERVAL: u32 = 0x2c;
        const TICK_GLOBAL: u32 = 0x011735B4;
        ((slot + INTERVAL) as *mut u32).write_unaligned(interval);
        let tick = (lf_checker_rt::global::<u32>(TICK_GLOBAL)).read_unaligned();
        ((slot + STAMP) as *mut u32).write_unaligned(tick);
        ((slot + INTERVAL_COPY) as *mut u32).write_unaligned(interval);
        ((slot + READY) as *mut u8).write(1);
        tick
    }
});

