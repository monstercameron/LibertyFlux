// original: 0x00C33AC0 vehicle_effect_cooldown_update

/// Update the vehicle's effect cooldown state. The method continues when the
/// word at `0xf48` is zero or `0xffff`, or when the masked status at `0x28`
/// equals `0x0c00`; it then stops if the timer dword at `0x12ec` is nonzero.
/// A nonzero byte at `0x12f1` is decremented when the stack flag is zero.
/// Otherwise a scripted random value selects a replacement byte, and the timer
/// dword is set to `0x22` except for replacement buckets two and three. This
/// thiscall method accepts one stack flag and calls the random provider only
/// on the randomized path.
lf_checker_rt::export!(thiscall, rw_00c33ac0(
    this: *mut u8,
    stack_flag: u32,
) -> () {
    const MODE_OFFSET: usize = 0xf48;
    const STATUS_OFFSET: usize = 0x28;
    const COOLDOWN_OFFSET: usize = 0x12ec;
    const EFFECT_STATE_OFFSET: usize = 0x12f1;
    const STATUS_MASK: u32 = 0x7c00;
    const ACTIVE_STATUS: u32 = 0x0c00;
    const RANDOM_CALLEE: u32 = 1;
    const RANDOM_VALUE_MASK: u8 = 0x7f;
    const RANDOM_BIAS: u8 = 0x6a;
    const TIMER_VALUE: u32 = 0x22;

    unsafe {
        let mode = this.add(MODE_OFFSET).cast::<u16>().read_unaligned();
        let status = this.add(STATUS_OFFSET).cast::<u32>().read_unaligned();
        if mode != 0 && mode != u16::MAX && status & STATUS_MASK != ACTIVE_STATUS {
            return;
        }
        if this.add(COOLDOWN_OFFSET).cast::<u32>().read_unaligned() != 0 {
            return;
        }

        let effect_state = this.add(EFFECT_STATE_OFFSET).read();
        if effect_state != 0 && stack_flag as u8 == 0 {
            this.add(EFFECT_STATE_OFFSET).write(effect_state.wrapping_sub(1));
            return;
        }

        let random_word: u32 = lf_checker_rt::callee_cdecl!(RANDOM_CALLEE, u32);
        let next_state = (random_word as u8 & RANDOM_VALUE_MASK).wrapping_sub(RANDOM_BIAS);
        this.add(EFFECT_STATE_OFFSET).write(next_state);
        let bucket = next_state & 0x07;
        if bucket < 2 || bucket >= 4 {
            this.add(COOLDOWN_OFFSET).cast::<u32>().write_unaligned(TIMER_VALUE);
        }
    }
});
