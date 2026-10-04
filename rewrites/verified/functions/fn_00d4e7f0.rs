// original: 0x00D4E7F0 task_stamp_time_and_period (proposed)

// Stamps the current tick and a period onto the task: the global tick goes to
/// `+0x14` and the low 16 bits of the argument go to `+0x18`. Returns nothing.
///
/// Original: 0x00D4E7F0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00d4e7f0(this: u32, period: u32) -> u32 {
    unsafe {
        const TICK_SLOT: u32 = 0x011735B4;
        let tick = lf_checker_rt::global::<u32>(TICK_SLOT).read();
        ((this + 0x14) as *mut u32).write_unaligned(tick);
        ((this + 0x18) as *mut u32).write_unaligned(period & 0xFFFF);
        0
    }
});
