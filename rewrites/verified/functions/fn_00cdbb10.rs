// original: 0x00CDBB10 match_balance_and_enable

/// Updates the NaturalMotion balance task after the incoming text key matches.
///
/// The stack argument points to a parameter record; its byte at `+0x0c` is
/// passed with the task's fixed seven-character key to a scripted comparison
/// helper. A nonzero comparison result is returned without changing the task.
/// On a match, byte `+0x28` is set. If byte `+0x29` is also nonzero, the
/// low sixteen bits of a scripted random value are converted to `f32`, scaled
/// in the original constant order, truncated to a signed integer, and used to
/// update the timer at `+0x1c` from the current game tick. The converted
/// integer remains the return value. The game object and record offsets are
/// 32-bit layout offsets from the original thiscall method.
lf_checker_rt::export!(thiscall, rw_00cdbb10(this: u32, params: u32) -> u32 {
    unsafe {
        const KEY_ADDRESS: u32 = 0x00ed_b1a8;
        const KEY_LENGTH: u32 = 7;
        const MATCHED: u32 = 0x28;
        const ACTIVE: u32 = 0x29;
        const TIMER: u32 = 0x1c;
        const GAME_TICK: u32 = 0x0117_35b4;
        const RANDOM_SCALE_A: u32 = 0x00fe_8680;
        const RANDOM_SCALE_B: u32 = 0x00fe_8e04;
        const TIMER_MARGIN: u32 = 500;
        const COMPARE_HELPER: u32 = 1;
        const RANDOM_HELPER: u32 = 2;

        #[inline(always)]
        fn multiply_in_order(left: f32, right: f32) -> f32 {
            core::hint::black_box(left) * core::hint::black_box(right)
        }

        let compare = lf_checker_rt::callee_cdecl!(
            COMPARE_HELPER,
            u32,
            lf_checker_rt::relocated(KEY_ADDRESS),
            params.wrapping_add(0x0c),
            KEY_LENGTH,
        );
        if compare != 0 {
            return compare;
        }

        ((this.wrapping_add(MATCHED)) as *mut u8).write(1);
        if ((this.wrapping_add(ACTIVE)) as *const u8).read() == 0 {
            return 0;
        }

        let current_tick = lf_checker_rt::global::<u32>(GAME_TICK).read();
        let random_word = lf_checker_rt::callee_cdecl!(RANDOM_HELPER, u32,);
        let bounded_sample = (random_word & 0xffff) as f32;
        let scale_a = f32::from_bits(lf_checker_rt::global::<u32>(RANDOM_SCALE_A).read());
        let scale_b = f32::from_bits(lf_checker_rt::global::<u32>(RANDOM_SCALE_B).read());
        let first_scale = multiply_in_order(bounded_sample, scale_a);
        let scaled_sample = multiply_in_order(first_scale, scale_b);
        // The original masks the random value to 16 bits before scaling; the
        // two image constants bound this result to the signed i32 range.
        let delay_ticks = scaled_sample as i32;
        let deadline = current_tick
            .wrapping_sub(delay_ticks as u32)
            .wrapping_add(TIMER_MARGIN);
        ((this.wrapping_add(TIMER)) as *mut u32).write_unaligned(deadline);
        delay_ticks as u32
    }
});
