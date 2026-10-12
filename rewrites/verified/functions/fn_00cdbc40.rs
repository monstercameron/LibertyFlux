// original: 0x00CDBC40 CTaskSimpleNMBalance::vf21

/// Requests a balance blend-out when its named event has elapsed.
///
/// The method uses `this` from ECX and takes two stack arguments. It compares
/// the text at `params + 0x0c` with the image key `balance`; the comparison
/// helper is intercepted. On a match, it adds the unsigned 16-bit start value
/// at `this + 0x20` to the unsigned duration at `this + 0x24`. If the unsigned
/// game tick is greater than that sum, it sets the status byte at `+0x14`,
/// replaces the task word at `+0x1c` with its masked blend-out form and sets
/// the word at `+0xc0` to its blend-out form. It always calls the arm/grab
/// helper with `this + 0x60` and the two incoming stack arguments, returning
/// that helper's result. All object fields retain their 32-bit layout.
lf_checker_rt::export!(thiscall, rw_00cdbc40(this: u32, arg_one: u32, params: u32) -> u32 {
    unsafe {
        const BALANCE_KEY: u32 = 0x00ed_cb20;
        const KEY_LENGTH: u32 = 7;
        const START_TICK: u32 = 0x20;
        const DURATION: u32 = 0x24;
        const STATUS: u32 = 0x14;
        const BLEND_WORD: u32 = 0x1c;
        const TASK_FLAGS: u32 = 0xc0;
        const ARM_GRAB_CONTEXT: u32 = 0x60;
        const GAME_TICK: u32 = 0x0117_35b4;
        const BLEND_MASK: u32 = 0xff08_45ff;
        const BLEND_BITS: u32 = 0x0008_4500;
        const FLAG_MASK: u32 = 0xfff2_ffff;
        const BLEND_FLAG: u32 = 0x0002_0000;
        const COMPARE_HELPER: u32 = 1;
        const ARM_GRAB_HELPER: u32 = 2;

        let compare = lf_checker_rt::callee_cdecl!(
            COMPARE_HELPER,
            u32,
            lf_checker_rt::relocated(BALANCE_KEY),
            params.wrapping_add(0x0c),
            KEY_LENGTH,
        );
        if compare == 0 {
            let start_tick = (this.wrapping_add(START_TICK) as *const u16).read_unaligned() as u32;
            let duration = (this.wrapping_add(DURATION) as *const u32).read_unaligned();
            let end_tick = start_tick.wrapping_add(duration);
            let current_tick = lf_checker_rt::global::<u32>(GAME_TICK).read();
            if current_tick > end_tick {
                let status = this.wrapping_add(STATUS) as *mut u8;
                status.write(status.read() | 4);

                let blend_word = this.wrapping_add(BLEND_WORD) as *mut u32;
                let previous_blend = blend_word.read_unaligned();
                let adjusted_blend = (previous_blend & BLEND_MASK) | BLEND_BITS;
                blend_word.write_unaligned(adjusted_blend);
                (this.wrapping_add(BLEND_WORD) as *mut u8).write(4);

                let task_flags = this.wrapping_add(TASK_FLAGS) as *mut u32;
                let previous_flags = task_flags.read_unaligned();
                task_flags.write_unaligned((previous_flags & FLAG_MASK) | BLEND_FLAG);
            }
        }

        lf_checker_rt::callee_thiscall!(
            ARM_GRAB_HELPER,
            u32,
            this.wrapping_add(ARM_GRAB_CONTEXT),
            arg_one,
            params,
        )
    }
});
